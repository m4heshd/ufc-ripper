<template>
  <div
      id="modConfig"
      class="modal mod-config"
  >
    <div class="modal-title">
      <i>settings</i>
      <h5>Configuration</h5>
    </div>

    <div class="mod-config__content">
      <article class="border round mod-config__content__section mod-config__content__about">
        <h5>About</h5>
        <p>
          UFC Ripper is a free and open-source tool. Make sure that you downloaded this tool only from the official
          source, located at
          <VAnchor href="https://github.com/m4heshd/ufc-ripper">https://github.com/m4heshd/ufc-ripper</VAnchor>
          .
        </p>
        <div class="mod-config__content__about__version-info">
          <span>Version:</span> <span>v{{ store.appMeta.version }}</span>
        </div>
        <div class="mod-config__content__about__socials">
          <div class="mod-config__content__about__socials__title">Join the conversation on these platforms:</div>
          <div class="center-content mod-config__content__about__socials__links">
            <VAnchor href="https://discord.gg/cQeXkvxCMn">
              <img
                  src="@/assets/images/discord-button.svg"
                  alt="UFC Ripper Discord server"
                  title="UFC Ripper Discord server"
              >
            </VAnchor>
            <VAnchor href="https://t.me/+1aacMrVWdr1jOWY1">
              <img
                  src="@/assets/images/telegram-button.svg"
                  alt="UFC Ripper Telegram support group"
                  title="UFC Ripper Telegram support group"
              >
            </VAnchor>
          </div>
        </div>
      </article>

      <article class="border round mod-config__content__section mod-config__content__account">
        <h5>Fight Pass account</h5>
        <div
            v-if="store.isLoggedIn"
            class="mod-config__content__account__user"
        >
          <i>person</i>
          {{ modConfig.data.user }}
        </div>
        <div
            v-else
            class="mod-config__content__account__creds"
        >
          <div class="field label suffix border round small">
            <select v-model="modConfig.data.region">
              <option value="dce.ufc">Global/US</option>
              <option value="dce.ufcbrazil">Brazil</option>
            </select>
            <label class="active">Region</label>
            <i>arrow_drop_down</i>
          </div>
          <div class="field label small border round">
            <input
                v-model="txtEmail"
                type="text"
                autocomplete="off"
                :disabled="busy"
            >
            <label>Email</label>
          </div>
          <div class="field label small border round">
            <input
                v-model="txtPass"
                type="password"
                autocomplete="off"
                :disabled="busy"
            >
            <label>Password</label>
          </div>
        </div>
        <button
            :disabled="!store.isLoggedIn"
            @click="onBtnLogoutClick"
        >
          Logout
        </button>
        <button
            :disabled="store.isLoggedIn || busy"
            @click="onBtnLoginClick"
        >Login
        </button>
      </article>

      <article class="border round mod-config__content__section">
        <h5>Search</h5>
        <nav class="v-switch">
          <div class="max">
            <h6>Search titles only</h6>
            <div>Refines the search only to be based on bout/event titles, which can fetch more accurate results</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.searchTitleOnly"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
      </article>

      <article class="border round mod-config__content__section">
        <h5>Previews (spoilers)</h5>
        <nav class="v-switch">
          <div class="max">
            <h6>Show thumbnails</h6>
            <div>Thumbnail image on previews and downloads</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.showThumb"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
        <nav class="v-switch">
          <div class="max">
            <h6>Show duration</h6>
            <div>Time duration of the bout/event</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.showDuration"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
        <nav class="v-switch">
          <div class="max">
            <h6>Show description</h6>
            <div>Bout/event description from Fight Pass</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.showDesc"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
      </article>

      <article class="border round mod-config__content__section mod-config__content__downloads">
        <h5>Downloads (output)</h5>
        <div class="field label suffix border round small">
          <input
              v-model="modConfig.data.dlPath"
              type="text"
              autocomplete="off"
              :disabled="store.appMeta.isContainer"
          >
          <label>Location (directory)</label>
          <i>folder_open</i>
        </div>
        <div class="field label suffix border round small">
          <select
              v-model="modConfig.data.mergeExt"
              :disabled="modConfig.data.cusFormat"
          >
            <option value="mp4">mp4</option>
            <option value="mkv">mkv</option>
            <option value="mov">mov</option>
            <option value="avi">avi</option>
            <option value="webm">webm</option>
          </select>
          <label class="active">Extension (container)</label>
          <i>arrow_drop_down</i>
        </div>
        <nav class="v-switch">
          <div class="max">
            <h6>Number files</h6>
            <div>Number each file increasingly with each download</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.numberFiles"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
        <div class="short-text">
          <span>Current file number:</span>
          <div class="field border round small no-margin">
            <input
                v-model.number="modConfig.data.curNumber"
                type="number"
                min="1"
                :disabled="!modConfig.data.numberFiles"
            >
          </div>
          <button
              class="circle fill medium"
              title="Reset file number to 1"
              @click="modConfig.data.curNumber = 1"
          >
            <i>restart_alt</i>
          </button>
        </div>
        <nav class="v-switch">
          <div class="max">
            <h6>Multi-fragment download</h6>
            <div>Concurrently download the given number of fragments (improves download speed)</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.multiFrag"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
        <div class="short-text">
          <span>Number of fragments:</span>
          <div class="field border round small no-margin">
            <input
                v-model.number="modConfig.data.concurFrags"
                type="number"
                min="1"
                :disabled="!modConfig.data.multiFrag"
            >
          </div>
        </div>
        <nav class="v-switch">
          <div class="max">
            <h6>Throttle downloads</h6>
            <div>Limit the speed of each download to the following</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.throttle"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
        <div class="short-text">
          <span>Download speed:</span>
          <div class="field border round small no-margin">
            <input
                v-model="modConfig.data.dlRate"
                type="text"
                :disabled="!modConfig.data.throttle"
            >
          </div>
        </div>
        <nav class="v-switch">
          <div class="max">
            <h6>Use custom download format</h6>
            <div>Use a custom download format provided by
              <VAnchor
                  style="text-decoration: none"
                  href="https://github.com/yt-dlp/yt-dlp?tab=readme-ov-file#format-selection"
              >
                <code>yt-dlp --list-formats (click here to learn more)</code>
              </VAnchor>
              .
              <br>
              This will override any video and audio settings you have specified below
            </div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.cusFormat"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
        <div class="long-text">
          <span>Format template:</span>
          <div class="field border round small no-margin">
            <input
                v-model="modConfig.data.formatID"
                type="text"
                :disabled="!modConfig.data.cusFormat"
            >
          </div>
        </div>
        <nav class="v-switch">
          <div class="max">
            <h6>Add metadata</h6>
            <div>Write any available metadata into the downloaded file (consumes extra time)</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.metadata"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
      </article>

      <article class="border round mod-config__content__section">
        <h5>Video</h5>
        <div
            v-if="modConfig.data.cusFormat"
            class="mod-config__content__section__warning"
        >
          <i>warning</i>
          <span>These settings are disabled because you're using a custom format template</span>
        </div>
        <div class="field label suffix border round small">
          <select
              v-model="modConfig.data.resolution"
              :disabled="modConfig.data.cusFormat"
          >
            <option value="288">288p</option>
            <option value="360">360p</option>
            <option value="504">504p</option>
            <option value="720">720p</option>
            <option value="1080">1080p</option>
          </select>
          <label class="active">Resolution</label>
          <i>arrow_drop_down</i>
        </div>
        <div class="field label suffix border round small">
          <select
              v-model="modConfig.data.vidQuality"
              :disabled="modConfig.data.cusFormat"
          >
            <option value="bestvideo">Best (huge filesize)</option>
            <option value="worstvideo">Worst</option>
          </select>
          <label class="active">Quality</label>
          <i>arrow_drop_down</i>
        </div>
      </article>

      <article class="border round mod-config__content__section">
        <h5>Audio</h5>
        <div
            v-if="modConfig.data.cusFormat"
            class="mod-config__content__section__warning"
        >
          <i>warning</i>
          <span>These settings are disabled because you're using a custom format template</span>
        </div>
        <div class="field label suffix border round small">
          <select
              v-model="modConfig.data.audQuality"
              :disabled="modConfig.data.cusFormat"
          >
            <option value="bestaudio">Best</option>
            <option value="worstaudio">Worst</option>
          </select>
          <label class="active">Quality</label>
          <i>arrow_drop_down</i>
        </div>
      </article>

      <article class="border round mod-config__content__section mod-config__content__proxy">
        <h5>Proxy</h5>
        <nav class="v-switch">
          <div class="max">
            <h6>Use proxy</h6>
            <div>Use the following proxy for API requests (downloads will not be proxied through this server)</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.useProxy"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
        <div class="field label suffix border round small">
          <select
              v-model="modConfig.data.proxyConfig.protocol"
              :disabled="!modConfig.data.useProxy"
          >
            <option value="http">http</option>
            <option value="https">https</option>
          </select>
          <label class="active">Protocol</label>
          <i>arrow_drop_down</i>
        </div>
        <div class="mod-config__content__proxy__split-section mod-config__content__proxy__endpoint">
          <div class="field label small border round">
            <input
                v-model="modConfig.data.proxyConfig.host"
                type="text"
                autocomplete="off"
                :disabled="!modConfig.data.useProxy"
            >
            <label>Host</label>
          </div>
          <div class="field label small border round">
            <input
                v-model.number="modConfig.data.proxyConfig.port"
                type="number"
                autocomplete="off"
                :disabled="!modConfig.data.useProxy"
            >
            <label>Port</label>
          </div>
        </div>
        <nav class="v-switch">
          <div class="max">
            <h6>Authentication</h6>
            <div>Leave these fields blank if your proxy server doesn't require authentication</div>
          </div>
        </nav>
        <div class="mod-config__content__proxy__split-section">
          <div class="field label small border round">
            <input
                v-model="modConfig.data.proxyConfig.auth.username"
                type="text"
                autocomplete="off"
                :disabled="!modConfig.data.useProxy"
            >
            <label>Username</label>
          </div>
          <div class="field label small border round">
            <input
                v-model="modConfig.data.proxyConfig.auth.password"
                type="password"
                autocomplete="off"
                :disabled="!modConfig.data.useProxy"
            >
            <label>Password</label>
          </div>
        </div>
      </article>
    </div>

    <nav class="right-align">
      <button
          class="border"
          data-ui="#modConfig"
      >
        Cancel
      </button>
      <button @click="save">
        <i>save</i>
        <span>Save</span>
      </button>
    </nav>
  </div>
</template>

<script setup>
// Core
import {ref} from 'vue';
// Store
import {useAppStore} from '@/store';
// Modules
import {useWSUtil} from '@/modules/ws-util';
// Components
import VAnchor from '@/components/VAnchor.vue';

// Store
const store = useAppStore();
const modConfig = store.modals.modConfig;

// Local state
const busy = ref(false);
const switchBusyState = (busyState) => busy.value = busyState === undefined ? !busy.value : busyState;
const fail = (error) => {
  store.popError(error);
  modConfig.data = JSON.parse(JSON.stringify(store.config));
};

// Websocket
const {login, saveConfig} = useWSUtil();

// Account
const txtEmail = ref('');
const txtPass = ref('');

function onBtnLogoutClick() {
  modConfig.data.user = '';
  modConfig.data.authToken = '';
  modConfig.data.refreshToken = '';
  save();
}

function onBtnLoginClick() {
  switchBusyState();
  login(modConfig.data.region, txtEmail.value, txtPass.value)
      .then(() => {
        store.popSuccess('Successfully logged in');
        window.ui('#modConfig');
      })
      .catch(fail)
      .finally(switchBusyState);
}

// Misc functions
function save() {
  saveConfig(modConfig.data)
      .then(() => {
        store.popSuccess('Configuration successfully updated');
        window.ui('#modConfig');
      })
      .catch(fail);
}
</script>

<style lang="scss">
.mod-config {
  width: 100%;
  max-width: 480rem;
  max-height: 80%;
  display: grid;
  grid-template-rows: max-content minmax(0rem, 1fr) max-content;
  overflow: hidden;

  & .v-switch > div {
    & > h6 {
      font-size: 15rem;
      font-weight: bold;
    }

    & > div {
      font-size: 11rem;
    }
  }

  &__content {
    overflow-y: auto;

    &__section {
      margin: 15rem 0;

      & > h5 {
        margin-bottom: 15rem;
        color: var(--primary);
        font-size: 18rem;
        font-weight: bold;
      }

      &__warning {
        display: flex;
        gap: 5rem;
        margin-bottom: 20rem;

        & > i {
          font-size: 20rem;
          color: var(--warning);
        }
      }

      & .short-text,
      & .long-text {
        display: flex;
        align-items: center;
        gap: 5rem;
        margin: 16rem 0 0 0;

        & > span {
          white-space: nowrap;
        }
      }

      & .short-text > div {
        width: 100rem;
      }

      & .long-text > div {
        width: 100%;
      }

      & > .field {
        margin-bottom: 16rem;

        &:last-child {
          margin-bottom: 0;
        }
      }
    }

    &__about {
      &__version-info {
        margin: 16rem 0;
        font-size: 15rem;

        & > span:first-child {
          font-weight: bold;
        }
      }

      &__socials {
        &__title {
          margin-bottom: 15rem;
          font-size: 15rem;
          font-weight: bold;
        }

        &__links {
          gap: 30rem;

          & > a {
            &:after {
              content: "";
              position: absolute;
              top: 0;
              left: 0;
              z-index: 1;
              border-radius: 8rem;
              width: 100%;
              height: 100%;
              background-position: center;
              background-image: radial-gradient(circle, rgba(255, 255, 255, .4) 1%, transparent 1%);
              opacity: 0;
              transition: none;
            }

            &:hover:after {
              background-size: 15000%;
              opacity: 1;
              transition: var(--speed2) background-size linear;
            }

            & > img {
              border-radius: 0;
            }
          }
        }
      }
    }

    &__account {
      &__user {
        margin-bottom: 20rem;
      }

      &__creds {
        & > .field {
          margin-bottom: 16rem;

          &:last-child {
            margin-bottom: 25rem;
          }
        }
      }
    }

    &__downloads {
      & > .field {
        margin-top: 25rem;
        margin-bottom: 10rem;
      }
    }

    &__proxy {
      & > .v-switch {
        margin-bottom: 20rem;
      }

      &__split-section {
        display: grid;
        grid-template-columns: 1fr 1fr;
        margin-top: 16rem;
        grid-gap: 5rem;

        & > .field {
          margin-top: 0;
          margin-bottom: 0;
        }
      }

      &__endpoint {
        grid-template-columns: 3fr 1.5fr;
      }
    }
  }
}
</style>
