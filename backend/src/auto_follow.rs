//! Opt-in replay followers. Search results are only candidates: the normal
//! manifest and download path still checks the subscriber's access.

use crate::{
    bin_util::start_download,
    config_util::{get_config, CONFIG_PATH},
    net_util::{get_vod_manifest, search_replay_playlists, search_replay_vods},
    state_util::get_dlq,
    txt_util::create_uuid,
    ws_util::{emit_auto_download_started, emit_config_update, emit_vod_download_progress},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{HashMap, HashSet},
    fs,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::{interval, MissedTickBehavior};
use ufcr_libs::{log_err, log_info, log_warn};

const POLL_INTERVAL: Duration = Duration::from_secs(30 * 60);
const MAX_PAGES: u64 = 10;
/// Upper bound on Fight Night event keys deep-searched per check.
const FIGHT_NIGHT_KEY_LIMIT: usize = 64;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct FollowState {
    handled_ids: HashSet<u64>,
    handled_titles: HashSet<String>,
    contender_initialized: bool,
    fight_night_initialized: bool,
    fight_night_baseline: String,
    fight_night_latest_event: String,
    ufc_initialized_number: u16,
    highest_ufc_number: u16,
    ufc_baseline_number: u16,
    ufc_baseline_ts: String,
    bjj_initialized_number: u16,
    highest_bjj_number: u16,
}

#[derive(Clone, Debug)]
struct Replay {
    id: u64,
    title: String,
    published: String,
}

pub fn start() {
    tokio::spawn(async {
        let mut timer = interval(POLL_INTERVAL);
        timer.set_missed_tick_behavior(MissedTickBehavior::Skip);
        let path = CONFIG_PATH.with_file_name("auto_follow_state.json");
        let stored = match fs::read(&path) {
            Ok(bytes) => match serde_json::from_slice(&bytes) {
                Ok(state) => state,
                Err(error) => {
                    log_err!("Replay follower state is invalid; automation paused: {error}");
                    return;
                }
            },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => FollowState::default(),
            Err(error) => {
                log_err!("Replay follower state cannot be read; automation paused: {error}");
                return;
            }
        };
        let state = Arc::new(Mutex::new(stored));
        loop {
            timer.tick().await;
            let (ufc, bjj, contender, fight_nights, existing) = {
                let config = get_config();
                if !config.auto_follow_replays || config.auth_token.is_empty() {
                    continue;
                }
                (
                    config.auto_follow_ufc_number,
                    config.auto_follow_bjj_number,
                    config.auto_follow_contender_series,
                    config.auto_follow_fight_nights,
                    config.auto_follow_existing,
                )
            };
            if let Err(error) =
                poll(&state, &path, ufc, bjj, contender, fight_nights, existing).await
            {
                log_warn!("Replay follower check failed: {error}");
            }
        }
    });
}

async fn poll(
    state: &Arc<Mutex<FollowState>>,
    path: &std::path::Path,
    ufc: u16,
    bjj: u16,
    contender: bool,
    fight_nights: bool,
    include_existing: bool,
) -> anyhow::Result<()> {
    if get_dlq().values().any(|vod| vod.status == "downloading") {
        return Ok(());
    }
    if ufc > 0 {
        let (is_new_selection, baseline_number) = {
            let s = state.lock().unwrap();
            (s.ufc_initialized_number != ufc, s.ufc_baseline_number)
        };
        if is_new_selection {
            let mut s = state.lock().unwrap();
            s.highest_ufc_number = ufc.saturating_sub(1);
        }
        // Record the first-check publication watermark before searching so that
        // existing recordings for later numbers are skipped when they are
        // discovered on later checks. The paired baseline number keeps the
        // stamp stable across retries when a first search fails.
        if ufc_take_baseline(include_existing, baseline_number, ufc) {
            let mut s = state.lock().unwrap();
            s.ufc_baseline_ts = now_utc_iso();
            s.ufc_baseline_number = ufc;
            save_state(path, &s)?;
        }
        let highest = state.lock().unwrap().highest_ufc_number.max(ufc);
        let mut batches = Vec::new();
        for number in highest.saturating_sub(2).max(ufc)..=highest.saturating_add(1) {
            let items = search_replays(&format!("\"UFC {number}\""), |n, d| {
                classify_ufc(n, d, number)
            })
            .await?;
            batches.push((number, items));
        }
        if is_new_selection && !include_existing {
            let mut s = state.lock().unwrap();
            for (number, items) in &batches {
                s.handled_ids.extend(items.iter().map(|i| i.id));
                s.handled_titles
                    .extend(items.iter().map(|i| i.title.clone()));
                if *number > s.highest_ufc_number && !items.is_empty() {
                    s.highest_ufc_number = *number;
                }
            }
            s.ufc_initialized_number = ufc;
            save_state(path, &s)?;
        } else {
            for (number, items) in batches {
                {
                    let mut s = state.lock().unwrap();
                    s.ufc_initialized_number = ufc;
                    if number > s.highest_ufc_number && !items.is_empty() {
                        s.highest_ufc_number = number;
                    }
                    save_state(path, &s)?;
                }
                for item in items {
                    let config = get_config();
                    if !config.auto_follow_replays || config.auto_follow_ufc_number != ufc {
                        return Ok(());
                    }
                    let allowed = {
                        let s = state.lock().unwrap();
                        numbered_replay_allowed(
                            include_existing,
                            &s.ufc_baseline_ts,
                            &item.published,
                        )
                    };
                    if !allowed {
                        continue;
                    }
                    if queue_one(state, path, item).await? {
                        return Ok(());
                    }
                }
            }
        }
    }
    if bjj > 0 {
        {
            let mut s = state.lock().unwrap();
            if s.bjj_initialized_number != bjj {
                s.highest_bjj_number = bjj.saturating_sub(1);
            }
        }
        let highest = state.lock().unwrap().highest_bjj_number.max(bjj);
        let initial = state.lock().unwrap().bjj_initialized_number != bjj;
        let mut batches = Vec::new();
        for number in highest.saturating_sub(2).max(bjj)..=highest.saturating_add(1) {
            let items = search_replays(&format!("\"UFC BJJ {number}\""), |n, d| {
                classify_bjj(n, d, number)
            })
            .await?;
            batches.push((number, items));
        }
        if initial && !include_existing {
            let mut s = state.lock().unwrap();
            for (number, items) in &batches {
                s.handled_ids.extend(items.iter().map(|i| i.id));
                s.handled_titles
                    .extend(items.iter().map(|i| i.title.clone()));
                if *number > s.highest_bjj_number && !items.is_empty() {
                    s.highest_bjj_number = *number;
                }
            }
            s.bjj_initialized_number = bjj;
            save_state(path, &s)?;
        } else {
            for (number, items) in batches {
                {
                    let mut s = state.lock().unwrap();
                    s.bjj_initialized_number = bjj;
                    if number > s.highest_bjj_number && !items.is_empty() {
                        s.highest_bjj_number = number;
                    }
                    save_state(path, &s)?;
                }
                for item in items {
                    let config = get_config();
                    if !config.auto_follow_replays || config.auto_follow_bjj_number != bjj {
                        return Ok(());
                    }
                    if queue_one(state, path, item).await? {
                        return Ok(());
                    }
                }
            }
        }
    }
    if contender {
        let mut items =
            search_replays("\"Dana White's Contender Series\"", classify_contender).await?;
        items.sort_by(|a, b| b.published.cmp(&a.published));
        {
            let mut s = state.lock().unwrap();
            if !s.contender_initialized {
                let keep = if include_existing { 1 } else { 0 };
                s.handled_ids.extend(items.iter().skip(keep).map(|i| i.id));
                s.handled_titles
                    .extend(items.iter().skip(keep).map(|i| i.title.clone()));
                s.contender_initialized = true;
                save_state(path, &s)?;
            }
        }
        for item in items {
            let config = get_config();
            if !config.auto_follow_replays || !config.auto_follow_contender_series {
                return Ok(());
            }
            if queue_one(state, path, item).await? {
                return Ok(());
            }
        }
    }
    if fight_nights {
        let baseline = state.lock().unwrap().fight_night_baseline.clone();
        let mut replays = collect_fight_night_replays(&baseline).await?;
        if replays.is_empty() {
            return Ok(());
        }
        replays.sort_by(|a, b| b.published.cmp(&a.published).then_with(|| b.id.cmp(&a.id)));
        {
            let mut guard = state.lock().unwrap();
            if !guard.fight_night_initialized {
                guard.fight_night_baseline = replays[0].published.clone();
                let latest = if include_existing {
                    replays
                        .first()
                        .and_then(|item| fight_night_key(&item.title))
                } else {
                    None
                };
                guard.fight_night_latest_event = latest.unwrap_or("").to_string();
                for replay in &replays {
                    if fight_night_key(&replay.title) == latest && latest.is_some() {
                        continue;
                    }
                    guard.handled_ids.insert(replay.id);
                    guard.handled_titles.insert(replay.title.clone());
                }
                guard.fight_night_initialized = true;
                save_state(path, &guard)?;
            }
        }
        for replay in replays {
            let older = {
                let guard = state.lock().unwrap();
                !guard.fight_night_baseline.is_empty()
                    && replay.published < guard.fight_night_baseline
                    && fight_night_key(&replay.title)
                        != Some(guard.fight_night_latest_event.as_str())
            };
            if older {
                continue;
            }
            let config = get_config();
            if !config.auto_follow_replays || !config.auto_follow_fight_nights {
                return Ok(());
            }
            if queue_one(state, path, replay).await? {
                return Ok(());
            }
        }
    }
    Ok(())
}

async fn search_replays(
    query: &str,
    classify: impl Fn(&str, u64) -> bool,
) -> anyhow::Result<Vec<Replay>> {
    let mut found = Vec::new();
    for page in 0..MAX_PAGES {
        let result = search_replay_vods(query, page).await?;
        let pages = result["nbPages"].as_u64().unwrap_or(0);
        if pages > MAX_PAGES {
            anyhow::bail!("Replay search exceeded the safe page limit");
        }
        let hits = result["hits"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid replay results"))?;
        for hit in hits {
            if let Some(item) = candidate(hit, &classify) {
                found.push(item);
            }
        }
        if page + 1 >= pages {
            break;
        }
    }
    Ok(found)
}

/// Collects full Fight Night replays. The broad phrase search is capped by the
/// search backend (~1000 hits) and ranked by relevance, so recent full replays
/// are not reachable directly. Instead we derive event keys from any matching
/// title (individual bouts end with the event name) and deep-search the newest
/// keys, stopping at the known baseline and a hard cap.
async fn collect_fight_night_replays(baseline: &str) -> anyhow::Result<Vec<Replay>> {
    let mut keys: HashMap<String, String> = HashMap::new();
    for page in 0..MAX_PAGES {
        let result = search_replay_vods("\"UFC Fight Night:\"", page).await?;
        let pages = result["nbPages"].as_u64().unwrap_or(0);
        if page == 0
            && (pages > MAX_PAGES || result["nbHits"].as_u64().unwrap_or(0) > MAX_PAGES * 100)
        {
            log_warn!(
                "Fight Night search has more than {MAX_PAGES} pages; only the first {MAX_PAGES} are checked"
            );
        }
        let hits = result["hits"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid replay results"))?;
        for hit in hits {
            let Some(name) = hit["name"].as_str() else {
                continue;
            };
            let Some(key) = fight_night_event_key_any(name) else {
                continue;
            };
            let published = hit["publishedDate"].as_str().unwrap_or("");
            let entry = keys.entry(key.to_string()).or_default();
            if published > entry.as_str() {
                *entry = published.to_string();
            }
        }
        if page + 1 >= pages {
            break;
        }
    }

    // Recent Fight Night event names are only exposed through year-tagged
    // playlists (`UFC2026`), so add those keys for the latest year and the one
    // before it.
    if let Some(year) = latest_year(&keys) {
        for y in [year, year.saturating_sub(1)] {
            let result = search_replay_playlists("\"UFC Fight Night:\"", y).await?;
            let hits = result["hits"]
                .as_array()
                .ok_or_else(|| anyhow::anyhow!("Invalid replay results"))?;
            for hit in hits {
                let Some(name) = hit["name"].as_str() else {
                    continue;
                };
                if let Some(key) = fight_night_event_key_any(name) {
                    keys.entry(key.to_string()).or_default();
                }
            }
        }
    }

    let selected =
        select_fight_night_keys(keys.into_iter().collect(), baseline, FIGHT_NIGHT_KEY_LIMIT);
    if selected.is_empty() {
        return Ok(Vec::new());
    }

    let mut found: Vec<Replay> = Vec::new();
    let mut seen: HashSet<u64> = HashSet::new();
    for key in selected {
        let result = search_replay_vods(&format!("\"UFC Fight Night: {key}\""), 0).await?;
        let hits = result["hits"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid replay results"))?;
        for hit in hits {
            if let Some(replay) = candidate(hit, &classify_fight_night) {
                if seen.insert(replay.id) {
                    found.push(replay);
                }
            }
        }
    }
    Ok(found)
}

fn candidate(hit: &Value, classify: &impl Fn(&str, u64) -> bool) -> Option<Replay> {
    let name = hit["name"].as_str()?;
    let duration = hit["duration"].as_u64()?;
    if !classify(name, duration) {
        return None;
    }
    Some(Replay {
        id: hit["id"].as_u64()?,
        title: name.into(),
        published: hit["publishedDate"].as_str().unwrap_or("").into(),
    })
}

/// Returns the text after `marker` (for example `UFC 331: `), allowing an
/// optional ASCII sponsor prefix such as `Crypto.com ` before it. Prefixes that
/// look like localized/branded wrappers (parentheses, non-ASCII characters or a
/// language name) are rejected so foreign-language uploads stay excluded.
fn event_suffix<'a>(name: &'a str, marker: &str) -> Option<&'a str> {
    let index = name.find(marker)?;
    let prefix = &name[..index];
    if !prefix.is_empty() {
        let sponsor_like = prefix.ends_with(' ')
            && prefix.len() <= 32
            && !prefix.contains(" - ")
            && !prefix.to_ascii_lowercase().contains(" vs ")
            && prefix
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | ' ' | '&' | '\''));
        let language_word = [
            "portuguese",
            "english",
            "espanol",
            "francais",
            "deutsch",
            "italiano",
        ]
        .iter()
        .any(|word| prefix.to_ascii_lowercase().contains(word));
        if !sponsor_like || language_word {
            return None;
        }
    }
    Some(&name[index + marker.len()..])
}

fn classify_ufc(name: &str, duration: u64, number: u16) -> bool {
    let Some(suffix) = event_suffix(name, &format!("UFC {number}: ")) else {
        return false;
    };
    if matches!(suffix, "Prelims" | "Fight Pass Prelims") {
        return duration >= 3600;
    }
    duration >= 5400
        && suffix.contains(" vs ")
        && !suffix.contains('(')
        && !suffix.contains(" - ")
        && !suffix.contains("Promo")
}
fn classify_bjj(name: &str, duration: u64, number: u16) -> bool {
    let Some(suffix) = event_suffix(name, &format!("UFC BJJ {number}: ")) else {
        return false;
    };
    duration >= 3600 && suffix.contains(" vs ") && !suffix.contains('(') && !suffix.contains(" - ")
}
fn classify_contender(name: &str, duration: u64) -> bool {
    if duration < 3600 {
        return false;
    }
    let Some(rest) = name.strip_prefix("Dana White's Contender Series: Season ") else {
        return false;
    };
    let Some((season, week)) = rest.split_once(", Week ") else {
        return false;
    };
    !season.is_empty()
        && season.bytes().all(|c| c.is_ascii_digit())
        && !week.is_empty()
        && week.bytes().all(|c| c.is_ascii_digit())
}

fn fight_night_key(name: &str) -> Option<&str> {
    let suffix = event_suffix(name, "UFC Fight Night: ")?;
    let base = [" Fight Pass Prelims", " Early Prelims", " Prelims"]
        .iter()
        .find_map(|ending| suffix.strip_suffix(ending))
        .unwrap_or(suffix);
    if base.contains(" vs ")
        && !base.contains('(')
        && !base.contains(" - ")
        && ![
            "Promo",
            "Press Conference",
            "Highlights",
            "Recap",
            "Countdown",
            "Interview",
            "Weigh-In",
            "Preview",
            "Full Fight",
        ]
        .iter()
        .any(|word| base.contains(word))
    {
        Some(base)
    } else {
        None
    }
}

/// Extracts the event key from any `UFC Fight Night: …` title, including
/// individual bouts where the event name is a suffix (for example
/// `A vs B UFC Fight Night: C vs D` yields `C vs D`). Used to discover events
/// whose full replays are not in the reachable search window.
fn fight_night_event_key_any(name: &str) -> Option<&str> {
    let marker = "UFC Fight Night: ";
    let index = name.rfind(marker)?;
    let mut base = &name[index + marker.len()..];
    for ending in [" Fight Pass Prelims", " Early Prelims", " Prelims"] {
        if let Some(stripped) = base.strip_suffix(ending) {
            base = stripped;
            break;
        }
    }
    if base.contains(" vs ")
        && !base.contains('(')
        && !base.contains(" - ")
        && ![
            "Promo",
            "Press Conference",
            "Highlights",
            "Recap",
            "Countdown",
            "Interview",
            "Weigh-In",
            "Preview",
            "Full Fight",
        ]
        .iter()
        .any(|word| base.contains(word))
    {
        Some(base)
    } else {
        None
    }
}

/// Highest release year present among the dated keys (for year-tag lookups).
fn latest_year(keys: &HashMap<String, String>) -> Option<u32> {
    keys.values()
        .filter(|published| !published.is_empty())
        .max()
        .and_then(|published| published.get(0..4))
        .and_then(|year| year.parse().ok())
}

/// Chooses which Fight Night event keys to deep-search. Undated keys (from
/// year-tagged playlists, i.e. recent events) come first so they are never
/// starved by the dated broad scan; dated keys follow newest-first, stopping
/// once older than the baseline. The total is bounded by `limit`. Pure so the
/// pagination/limit behaviour can be tested.
fn select_fight_night_keys(
    keys: Vec<(String, String)>,
    baseline: &str,
    limit: usize,
) -> Vec<String> {
    let (undated, mut dated): (Vec<_>, Vec<_>) = keys
        .into_iter()
        .partition(|(_, published)| published.is_empty());
    dated.sort_by(|a, b| b.1.cmp(&a.1));
    let mut selected = Vec::new();
    for (key, _) in undated {
        if selected.len() >= limit {
            return selected;
        }
        selected.push(key);
    }
    for (key, published) in dated {
        if !baseline.is_empty() && published.as_str() < baseline {
            break;
        }
        if selected.len() >= limit {
            break;
        }
        selected.push(key);
    }
    selected
}

fn classify_fight_night(name: &str, duration: u64) -> bool {
    if fight_night_key(name).is_none() {
        return false;
    }
    let Some(suffix) = event_suffix(name, "UFC Fight Night: ") else {
        return false;
    };
    if [" Fight Pass Prelims", " Early Prelims", " Prelims"]
        .iter()
        .any(|ending| suffix.ends_with(ending))
    {
        duration >= 3600
    } else {
        duration >= 5400
    }
}

/// Whether the selected numbered-UFC event should (re)record its first-check
/// publication watermark: when Include-available is off and the stored baseline
/// number does not match the selected number. Because the baseline number is
/// persisted together with the timestamp, a retry after a failed first search
/// keeps the original watermark instead of moving it forward, and changing the
/// selected number re-baselines exactly once.
fn ufc_take_baseline(include_existing: bool, baseline_number: u16, selected: u16) -> bool {
    !include_existing && baseline_number != selected
}

/// Whether a numbered-UFC replay may be queued. With Include-available off and a
/// watermark set, only publications strictly after the watermark qualify;
/// missing or malformed dates are treated as pre-existing so old content is
/// never backfilled.
fn numbered_replay_allowed(include_existing: bool, baseline: &str, published: &str) -> bool {
    if include_existing || baseline.is_empty() {
        return true;
    }
    has_iso_date(published) && published > baseline
}

/// True when `value` begins with an ISO-8601 UTC date (`YYYY-MM-DD`).
fn has_iso_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 10
        && bytes[0..4].iter().all(u8::is_ascii_digit)
        && bytes[4] == b'-'
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[7] == b'-'
        && bytes[8..10].iter().all(u8::is_ascii_digit)
}

/// Current UTC time as an ISO-8601 string, used as the numbered-UFC watermark.
fn now_utc_iso() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs() as i64);
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

/// Converts days since the Unix epoch to a civil `(year, month, day)`.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if month <= 2 { year + 1 } else { year }, month, day)
}

async fn queue_one(
    state: &Arc<Mutex<FollowState>>,
    path: &std::path::Path,
    replay: Replay,
) -> anyhow::Result<bool> {
    {
        let s = state.lock().unwrap();
        if s.handled_ids.contains(&replay.id) || s.handled_titles.contains(&replay.title) {
            return Ok(false);
        }
    }
    let mut vod = get_vod_manifest(replay.id, true).await?;
    if !vod.access {
        return Ok(false);
    }
    if existing_media(&get_config().dl_path, &vod.title) {
        let mut s = state.lock().unwrap();
        s.handled_ids.insert(replay.id);
        s.handled_titles.insert(replay.title);
        save_state(path, &s)?;
        return Ok(false);
    }
    let config = get_config();
    if !config.auto_follow_replays
        || config.auth_token.is_empty()
        || get_dlq().values().any(|item| item.status == "downloading")
    {
        return Ok(false);
    }
    vod.q_id = create_uuid();
    let id = replay.id;
    let title = replay.title.clone();
    let done_state = Arc::clone(state);
    let done_path = path.to_path_buf();
    let queued = start_download(
        &vod,
        false,
        |q, updates| emit_vod_download_progress(q, updates),
        move |q| {
            let mut s = done_state.lock().unwrap();
            s.handled_ids.insert(id);
            s.handled_titles.insert(title);
            if let Err(e) = save_state(&done_path, &s) {
                log_err!("Could not save completed replay ID: {e}");
            }
            emit_vod_download_progress(q, serde_json::json!({"status":"completed"}));
        },
        |q, error| {
            log_warn!("Automatic replay download failed: {error}");
            emit_vod_download_progress(q, serde_json::json!({"status":"failed"}));
        },
    )
    .await?;
    emit_auto_download_started(&queued);
    emit_config_update();
    log_info!("Queued replay: {}", replay.title);
    Ok(true)
}

fn existing_media(dir: &str, title: &str) -> bool {
    let Ok(entries) = fs::read_dir(dir) else {
        return false;
    };
    entries.flatten().any(|entry| {
        let path = entry.path();
        let stem = path.file_stem().and_then(|p| p.to_str());
        let numbered = stem.and_then(|s| s.split_once(". "));
        path.is_file()
            && (stem == Some(title)
                || numbered
                    .is_some_and(|(p, r)| p.bytes().all(|c| c.is_ascii_digit()) && r == title))
            && matches!(
                path.extension().and_then(|e| e.to_str()),
                Some("mp4" | "mkv" | "mov" | "avi" | "webm")
            )
    })
}
fn save_state(path: &std::path::Path, state: &FollowState) -> anyhow::Result<()> {
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_vec(state)?)?;
    if let Err(error) = fs::rename(&tmp, path) {
        if !path.exists() {
            return Err(error.into());
        }
        fs::remove_file(path)?;
        fs::rename(&tmp, path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        civil_from_days, classify_bjj, classify_contender, classify_fight_night, classify_ufc,
        existing_media, fight_night_event_key_any, fight_night_key, has_iso_date, latest_year,
        now_utc_iso, numbered_replay_allowed, save_state, select_fight_night_keys,
        ufc_take_baseline, FollowState,
    };
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn selects_complete_replays_only() {
        assert!(classify_ufc("UFC 321: Aspinall vs Gane", 10811, 321));
        assert!(classify_ufc("UFC 321: Prelims", 13733, 321));
        assert!(!classify_ufc(
            "UFC 321: Post-Fight Press Conference",
            4197,
            321
        ));
        assert!(classify_bjj("UFC BJJ 9: Fowler vs Johnson", 7200, 9));
        assert!(!classify_bjj("UFC BJJ 9: Fowler vs Johnson", 600, 9));
        assert!(classify_contender(
            "Dana White's Contender Series: Season 10, Week 7",
            10223
        ));
        assert!(!classify_contender(
            "Dana White's Contender Series: Season 10, Week 7 Highlights",
            9000
        ));
    }

    #[test]
    fn accepts_sponsored_numbered_ufc_full_replays() {
        assert!(classify_ufc(
            "Crypto.com UFC 331: Van vs Pantoja 2",
            11335,
            331
        ));
        assert!(classify_ufc("Crypto.com UFC 331: Prelims", 7346, 331));
        assert!(classify_ufc(
            "Crypto.com UFC 331: Fight Pass Prelims",
            5389,
            331
        ));
        assert!(classify_ufc("UFC 331: Van vs Pantoja 2", 11335, 331));
        assert!(classify_ufc(
            "UFC 330: Makhachev vs Machado Garry",
            12543,
            330
        ));
    }

    #[test]
    fn rejects_sponsored_extras_and_foreign_language() {
        assert!(!classify_ufc(
            "Crypto.com UFC 331: Post-Fight Press Conference",
            6590,
            331
        ));
        assert!(!classify_ufc("Crypto.com UFC 331: Fight Motion", 479, 331));
        assert!(!classify_ufc(
            "Crypto.com UFC 331: Embedded: Vlog Series - Episode 6 (RUS)",
            521,
            331
        ));
        assert!(!classify_ufc(
            "(ESPAÑOL) UFC 331: Van vs Pantoja 2",
            10923,
            331
        ));
        assert!(!classify_ufc(
            "(FRANÇAIS) UFC 331: Van vs Pantoja 2",
            11077,
            331
        ));
        assert!(!classify_ufc(
            "(PORTUGUESE) UFC 331: Van x Pantoja 2",
            23833,
            331
        ));
        assert!(!classify_ufc(
            "UFC 331: ヴァン VS パントーヤ 2 (日本語実況解説)",
            11312,
            331
        ));
        assert!(!classify_ufc(
            "UFC 331: Ван vs Пантоха 2 (русские комментаторы)",
            23541,
            331
        ));
        assert!(!classify_ufc(
            "Tom Aspinall vs Ciryl Gane UFC 331",
            944,
            331
        ));
        assert!(!classify_ufc("UFC 332: Prelims", 14000, 331));
    }

    #[test]
    fn accepts_sponsored_bjj_full_events_only() {
        assert!(classify_bjj(
            "Crypto.com UFC BJJ 11: Smith vs Jones",
            7200,
            11
        ));
        assert!(classify_bjj("UFC BJJ 10: Tackett vs Gracie", 10126, 10));
        assert!(!classify_bjj(
            "Crypto.com UFC BJJ 11: Smith vs Jones",
            600,
            11
        ));
        assert!(!classify_bjj(
            "(ESPAÑOL) UFC BJJ 11: Smith vs Jones",
            7200,
            11
        ));
        assert!(!classify_bjj("Fowler vs Johnson UFC BJJ 11", 7200, 11));
        assert!(!classify_bjj(
            "Crypto.com UFC BJJ 11: Post-Fight Press Conference",
            4000,
            11
        ));
    }

    #[test]
    fn state_and_completed_media_survive_restart() {
        let dir = std::env::temp_dir().join(format!("ufcr-follow-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let state_path = dir.join("state.json");
        let mut state = FollowState::default();
        state.handled_ids.insert(1024887);
        save_state(&state_path, &state).unwrap();
        let restored: FollowState =
            serde_json::from_slice(&std::fs::read(&state_path).unwrap()).unwrap();
        assert!(restored.handled_ids.contains(&1024887));
        let title = "UFC 321 - Prelims";
        std::fs::write(dir.join(format!("{title}.mp4.part")), b"").unwrap();
        assert!(!existing_media(dir.to_str().unwrap(), title));
        std::fs::write(dir.join(format!("42. {title}.mkv")), b"complete").unwrap();
        assert!(existing_media(dir.to_str().unwrap(), title));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn selects_full_fight_night_segments_only() {
        assert!(classify_fight_night(
            "UFC Fight Night: Bautista vs Oliveira",
            9800
        ));
        assert!(classify_fight_night(
            "UFC Fight Night: Du Plessis vs Usman Prelims",
            7200
        ));
        assert!(classify_fight_night(
            "UFC Fight Night: Du Plessis vs Usman Early Prelims",
            5400
        ));
        assert!(classify_fight_night(
            "Crypto.com UFC Fight Night: Royval vs Kape Fight Pass Prelims",
            5400
        ));
        assert_eq!(
            fight_night_key("UFC Fight Night: Du Plessis vs Usman Prelims"),
            Some("Du Plessis vs Usman")
        );
        assert_eq!(
            fight_night_key("UFC Fight Night: Du Plessis vs Usman"),
            Some("Du Plessis vs Usman")
        );
        assert!(!classify_fight_night(
            "UFC Fight Night: Bautista vs Oliveira Highlights",
            9000
        ));
        assert!(!classify_fight_night(
            "UFC Fight Night: Bautista vs Oliveira Post-Fight Press Conference",
            7200
        ));
        assert!(!classify_fight_night(
            "(ESPAÑOL) UFC Fight Night: Bautista vs Oliveira",
            9000
        ));
        assert!(!classify_fight_night(
            "UFC Fight Night: Bautista vs Oliveira (RUS)",
            9000
        ));
        assert!(!classify_fight_night(
            "Alex Morono vs Daniil Donchenko UFC Fight Night: Bautista vs Oliveira",
            9000
        ));
        assert!(!classify_fight_night(
            "UFC Fight Night: Bautista vs Oliveira",
            1200
        ));
    }

    #[test]
    fn old_follower_state_leaves_fight_nights_uninitialized() {
        let state: FollowState = serde_json::from_value(json!({
            "handled_ids": [1024887],
            "handled_titles": ["Dana White's Contender Series: Season 10, Week 7"],
            "contender_initialized": true
        }))
        .unwrap();
        assert!(!state.fight_night_initialized);
        assert!(state.fight_night_baseline.is_empty());
        assert!(state.handled_ids.contains(&1024887));
    }

    #[test]
    fn fight_night_event_key_from_any_title() {
        assert_eq!(
            fight_night_event_key_any("A vs B UFC Fight Night: C vs D"),
            Some("C vs D")
        );
        assert_eq!(
            fight_night_event_key_any("UFC Fight Night: C vs D Prelims"),
            Some("C vs D")
        );
        assert_eq!(
            fight_night_event_key_any("Crypto.com UFC Fight Night: C vs D Early Prelims"),
            Some("C vs D")
        );
        assert_eq!(
            fight_night_event_key_any("UFC Fight Night: C vs D Highlights"),
            None
        );
        assert_eq!(fight_night_event_key_any("Unrelated title"), None);
    }

    #[test]
    fn fight_night_key_selection_respects_baseline_and_limit() {
        let keys = vec![
            ("old".to_string(), "2023-06-10T00:00:00Z".to_string()),
            ("new".to_string(), "2026-08-11T16:20:04Z".to_string()),
            ("mid".to_string(), "2025-07-26T22:00:00Z".to_string()),
            ("playlist".to_string(), String::new()),
        ];
        // No baseline: undated (playlist) keys first, then dated newest first.
        assert_eq!(
            select_fight_night_keys(keys.clone(), "", 2),
            vec!["playlist".to_string(), "new".to_string()]
        );
        assert_eq!(
            select_fight_night_keys(keys.clone(), "", 4),
            vec![
                "playlist".to_string(),
                "new".to_string(),
                "mid".to_string(),
                "old".to_string()
            ]
        );
        // A baseline stops older dated keys but keeps undated (playlist) keys.
        assert_eq!(
            select_fight_night_keys(keys.clone(), "2026-01-01T00:00:00Z", 10),
            vec!["playlist".to_string(), "new".to_string()]
        );
        // Undated keys still respect the cap.
        assert_eq!(
            select_fight_night_keys(keys, "2027-01-01T00:00:00Z", 1),
            vec!["playlist".to_string()]
        );
    }

    #[test]
    fn latest_year_picks_highest_dated_year() {
        let mut keys = HashMap::new();
        keys.insert("a".to_string(), "2025-07-26T22:00:00Z".to_string());
        keys.insert("b".to_string(), "2026-08-11T16:20:04Z".to_string());
        keys.insert("c".to_string(), String::new());
        assert_eq!(latest_year(&keys), Some(2026));
        assert_eq!(latest_year(&HashMap::new()), None);
    }

    #[test]
    fn numbered_ufc_watermark_skips_old_and_queues_new() {
        let baseline = "2026-09-24T21:00:00Z";
        // Selecting 320 and later discovering 322-331: their publications
        // predate the watermark and must be skipped, not downloaded.
        assert!(!numbered_replay_allowed(
            false,
            baseline,
            "2026-01-05T00:00:00Z"
        ));
        assert!(!numbered_replay_allowed(
            false,
            baseline,
            "2024-11-30T12:00:00.000Z"
        ));
        // A genuinely new publication after the watermark is eligible.
        assert!(numbered_replay_allowed(
            false,
            baseline,
            "2026-09-25T02:11:00.000Z"
        ));
        assert!(numbered_replay_allowed(
            false,
            baseline,
            "2026-09-24T21:00:01Z"
        ));
        // Missing or malformed dates are treated conservatively as pre-existing.
        assert!(!numbered_replay_allowed(false, baseline, ""));
        assert!(!numbered_replay_allowed(false, baseline, "not-a-date"));
        assert!(!numbered_replay_allowed(
            false,
            baseline,
            "2026-9-1T00:00:00Z"
        ));
        // Include-available ON preserves backfill of existing recordings.
        assert!(numbered_replay_allowed(
            true,
            baseline,
            "2020-01-01T00:00:00Z"
        ));
        assert!(numbered_replay_allowed(true, "", "2020-01-01T00:00:00Z"));
        // No watermark yet: the first check is baselined separately, so allow.
        assert!(numbered_replay_allowed(false, "", "2020-01-01T00:00:00Z"));
    }

    #[test]
    fn ufc_watermark_stamped_once_per_number() {
        // New selection (no baseline yet) and legacy state, Include-available off.
        assert!(ufc_take_baseline(false, 0, 320));
        assert!(ufc_take_baseline(false, 0, 330));
        // Retry after a failed first search with the same number -> no restamp.
        assert!(!ufc_take_baseline(false, 320, 320));
        assert!(!ufc_take_baseline(false, 330, 330));
        // Changing the selected number re-baselines once.
        assert!(ufc_take_baseline(false, 320, 322));
        assert!(ufc_take_baseline(false, 330, 320));
        // Include-available on -> never stamp (backfill preserved).
        assert!(!ufc_take_baseline(true, 0, 320));
        assert!(!ufc_take_baseline(true, 320, 322));
    }

    #[test]
    fn failed_first_search_does_not_move_the_watermark() {
        // First check stamped ts for number 320 before the search failed.
        let (number, ts) = (320u16, "2026-09-24T21:00:00Z");
        // Retry with the same number must not restamp.
        assert!(!ufc_take_baseline(false, number, 320));
        // A replay published after the original watermark still qualifies...
        assert!(numbered_replay_allowed(false, ts, "2026-09-24T21:05:00Z"));
        // ...while one published before it is still skipped.
        assert!(!numbered_replay_allowed(false, ts, "2026-09-24T20:59:59Z"));
        // Changing the number re-baselines (new stamp) once.
        assert!(ufc_take_baseline(false, number, 322));
    }

    #[test]
    fn now_utc_iso_is_well_formed() {
        let now = now_utc_iso();
        assert_eq!(now.len(), 20);
        assert!(has_iso_date(&now));
        assert!(now.ends_with('Z'));
        // Epoch-day conversion sanity checks.
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(10_957), (2000, 1, 1));
    }
}
