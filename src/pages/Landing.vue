<template>
  <div class="ufcr">
    <div class="logo center-content">
      <a
          href="https://github.com/m4heshd/ufc-ripper"
          target="_blank"
          rel="noopener noreferrer"
      >
        <img
            alt="UFC Ripper logo"
            title="UFC Ripper by m4heshd"
            src="@/assets/images/ufcr-logo.svg"
        >
      </a>
    </div>

    <div class="url-section center-content">
      <div class="field label suffix border round url-section__txt-link">
        <input
            v-model="txtLink"
            type="text"
            autocomplete="off"
            :disabled="busy"
            @keyup.enter="searchVOD(txtLink)"
        >
        <label>Link / Search</label>
        <span class="helper">Insert the link to Fight Pass video or search query here</span>
        <a
            v-if="busy"
            class="loader"
        ></a>
        <i v-else>link</i>
      </div>

      <button
          class="square round large"
          title="Search fight pass library"
          :disabled="busy"
          @click="searchVOD(txtLink)"
      >
        <i>video_search</i>
      </button>

      <button
          class="square round large"
          title="Download from link"
          :disabled="busy"
          @click="verifyVODURL(txtLink)"
      >
        <i>download</i>
      </button>

      <button
          class="square round large"
          title="Get available formats"
          :disabled="busy"
          @click="getAvailableFormats(txtLink)"
      >
        <i>stock_media</i>
      </button>

      <button
          class="square round large"
          title="Configuration"
          :disabled="busy"
          @click="store.showModConfig"
      >
        <i>settings</i>
      </button>

      <button
          class="square round large"
          title="Support this project"
          :disabled="busy"
          @click="onBtnSupportClick"
      >
        <i>favorite</i>
      </button>
    </div>

    <article
        v-show="!store.search.showResults"
        class="border round vod-section"
    >
      <div class="vod-section__header">
        <h5>Downloads {{ store.activeDownloads ? `(${store.activeDownloads} active)` : '' }}</h5>
        <div class="vod-section__header__actions">
          <button
              class="border circle small"
              title="Open downloads directory"
              @click="onBtnOpenDLDir"
          >
            <i>folder_open</i>
          </button>
          <button
              class="border circle small"
              title="Show search results"
              @click="store.showSearchResults"
              :disabled="!store.searchIsResultsAvailable"
          >
            <i>manage_search</i>
          </button>
          <button
              class="border circle small"
              title="Clear downloads queue"
              @click="onBtnClearDLQueueClick"
          >
            <i>playlist_remove</i>
          </button>
        </div>
      </div>

      <div class="vod-section__downloads">
        <VODCard
            v-for="vod of store.downloadQueue"
            :vVODData="vod"
            :vShowThumb="store.config.showThumb"
            :vShowDesc="store.config.showDesc"
            @cancelDL="onDownloadCancel"
            @retryDL="onDownloadRetry"
        ></VODCard>
      </div>
    </article>

    <article
        v-show="store.search.showResults"
        class="border round vod-section"
    >
      <div class="vod-section__header">
        <h5>Search results</h5>
        <div class="vod-section__header__actions">
          <button
              v-if="store.searchIsResultsAvailable"
              class="border circle small"
              title="Previous page"
              :disabled="!store.searchCanPrevious"
              @click="searchPreviousPage"
          >
            <i>navigate_before</i>
          </button>
          <div
              v-if="store.searchIsResultsAvailable"
              class="center-content vod-section__header__actions__pagination"
          >
            {{ store.searchCurrentPage }} / {{ store.search.result.nbPages }}
          </div>
          <button
              v-if="store.searchIsResultsAvailable"
              class="border circle small"
              title="Next page"
              :disabled="!store.searchCanNext"
              @click="searchNextPage"
          >
            <i>navigate_next</i>
          </button>
          <button
              class="border circle small"
              title="How to use the search feature"
              @click="onBtnSearchHelpClick"
          >
            <i>question_mark</i>
          </button>
          <button
              class="border circle small"
              title="Close search results"
              @click="store.hideSearchResults"
          >
            <i>search_off</i>
          </button>
        </div>
      </div>

      <div
          v-if="store.searchIsResultsAvailable"
          class="vod-section__search-results"
      >
        <BlockVODCard
            v-for="vod of store.search.result.hits"
            :vVODData="vod"
            :vShowThumb="store.config.showThumb"
            :vShowDuration="store.config.showDuration"
            :vShowDesc="store.config.showDesc"
            :vBusyState="busy"
            @download="(id) => verifyVODURL(store.getFightPassURLByID(id))"
            @getFormats="(id) => getAvailableFormats(store.getFightPassURLByID(id))"
            @openExternal="(id) => store.openVODInFightPass(id)"
        ></BlockVODCard>
      </div>

      <div
          v-else
          class="center-content vod-section__empty-result"
      >
        <div class="vod-section__empty-result__icon">
          <i>sentiment_dissatisfied</i>
        </div>
        <div class="vod-section__empty-result__text">
          No matching videos were found for your search query..
        </div>
      </div>
    </article>

    <div class="credits">
      <p>
        Made with ❤️ by
        <a
            href="https://github.com/m4heshd"
            target="_blank"
            rel="noopener noreferrer"
        >
          m4heshd
        </a>
      </p>
    </div>

    <!-- Modals -->
    <ModVODConfirm
        vID="modVODConfirm"
        :vVODData="verifiedVOD"
        :vShowThumb="store.config.showThumb"
        :vShowDesc="store.config.showDesc"
        @onConfirm="download"
    ></ModVODConfirm>
    <ModConfig></ModConfig>
    <ModBinDL></ModBinDL>
    <ModViewFormats></ModViewFormats>
    <ModSearchHelp></ModSearchHelp>
    <ModSupport></ModSupport>
    <ModMsgBox
        vID="modUpdatePrompt"
        vIcon="upgrade"
        vTitle="App update available"
        vType="yes-no"
        @on-yes="store.openAppDownloadPage"
    >
      A new update <b>(v{{ store.update.version }})</b> for UFC Ripper is available. Would you like to visit the
      download page?
    </ModMsgBox>

    <!-- Overlay -->
    <Overlay :vActive="store.ui.overlay"></Overlay>
  </div>
</template>

<script setup>
// Core
import {ref, nextTick, onMounted} from 'vue';
// Store
import {useAppStore} from '@/store';
import {useModViewFormatsStore} from '@/store/modViewFormats';
// Modules
import {useWSUtil} from '@/modules/ws-util';
// Components
import VODCard from '@/components/VODCard.vue';
import BlockVODCard from '@/components/BlockVODCard.vue';
import ModVODConfirm from '@/components/ModVODConfirm.vue';
import ModConfig from '@/components/ModConfig.vue';
import ModBinDL from '@/components/ModBinDL.vue';
import ModViewFormats from '@/components/ModViewFormats.vue';
import ModSupport from '@/components/ModSupport.vue';
import ModMsgBox from '@/components/ModMsgBox.vue';
import ModSearchHelp from '@/components/ModSearchHelp.vue';
import Overlay from '@/components/Overlay.vue';

// Store
const store = useAppStore();
const modViewFormats = useModViewFormatsStore();

// Websocket
const {
  cancelDownload,
  clearDLQ,
  downloadVOD,
  initSocket,
  openDownloadsDir,
  searchVODs,
  verifyURL,
  getFormats
} = useWSUtil();

initSocket();

// Local state
const busy = ref(false);
const switchBusyState = (busyState) => busy.value = busyState === undefined ? !busy.value : busyState;
const verifiedVOD = ref({});

// URL Section
const txtLink = ref('');

function onBtnSupportClick() {
  window.ui('#modSupport');
}

// Downloads section
function onBtnOpenDLDir() {
  openDownloadsDir().catch(store.popError);
}

function onBtnClearDLQueueClick() {
  clearDLQ();
}

function onDownloadCancel(VOD) {
  cancelDownload(VOD)
      .then(() => {
        store.setDownloadCancelled(VOD.qID);
      })
      .catch(store.popError);
}

function onDownloadRetry(VOD) {
  store.setDownloadRestart(VOD);
  store.popInfo('Download restarted');

  downloadVOD(VOD, true)
      .catch(store.popError);
}

// Search results section
function onBtnSearchHelpClick() {
  window.ui('#modSearchHelp');
}

// Lifecycle hooks
onMounted(() => nextTick(() => {
  window.ui();
}));

// Misc functions
function searchVOD(query, page = 0) {
  if (!store.isLoggedIn) return store.popError('You need to be logged in to search videos');

  switchBusyState();

  searchVODs(query, page)
      .then((res) => {
        store.search.result = res;
        store.showSearchResults();
      })
      .catch(store.popError)
      .finally(switchBusyState);
}

function searchNextPage() {
  searchVOD(store.search.result.query, store.search.result.page + 1);
}

function searchPreviousPage() {
  searchVOD(store.search.result.query, store.search.result.page - 1);
}

function verifyVODURL(url) {
  if (!store.isLoggedIn) return store.popError('You need to be logged in to download videos');

  switchBusyState();

  verifyURL(url)
      .then((res) => {
        verifiedVOD.value = res;

        window.ui('#modVODConfirm');
      })
      .catch(store.popError)
      .finally(switchBusyState);
}

function getAvailableFormats(url) {
  if (!store.isLoggedIn) return store.popError('You need to be logged in to check download formats');

  switchBusyState();

  getFormats(url)
      .then((res) => {
        modViewFormats.setVODData(res);
        window.ui('#modViewFormats');
      })
      .catch(store.popError)
      .finally(switchBusyState);
}

function download(VOD) {
  switchBusyState();
  store.hideSearchResults();

  downloadVOD(VOD, false)
      .then((res) => {
        store.addDownload(res);
        store.popInfo('Download started');

        txtLink.value = '';
      })
      .catch(store.popError)
      .finally(switchBusyState);
}
</script>

<style lang="scss">
@use "@/assets/styles/app.scss";
@use "@/assets/styles/common.scss";
@use "@/assets/styles/overrides.scss";

.ufcr {
  display: grid;
  grid-template-rows: max-content max-content minmax(0px, 1fr) max-content;
  justify-items: center;
  margin: 3vw 3vw 1vw;

  .logo {
    margin-bottom: 35px;

    & > a > img {
      height: 55px;
    }
  }

  .url-section {
    gap: 10px;
    margin-bottom: 50px;

    &__txt-link {
      width: 40vw;
      max-width: 900px;
      margin-bottom: 0;
    }
  }

  .vod-section {
    display: grid;
    grid-template-rows: max-content minmax(0px, 1fr);
    height: 100%;
    max-width: 1300px;
    width: 100%;
    margin: 0;

    &__header {
      display: grid;
      grid-template-columns: auto max-content;

      & > h5 {
        margin-bottom: 30px;
        color: var(--primary);
        font-weight: bold;
      }

      &__actions {
        display: flex;

        &__pagination {
          height: 32rem;
          margin: 0 5px;
          font-size: 15rem;
          font-weight: bold;
          color: var(--primary);
        }
      }
    }

    &__downloads, &__search-results {
      margin: 10px;
      overflow: auto;
      border-radius: 0;
    }

    &__search-results {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
      grid-auto-rows: max-content;
      justify-items: center;
      grid-gap: 30px;

      * + article {
        margin-top: 0;
      }
    }

    &__empty-result {
      flex-direction: column;
      gap: 30px;
      color: var(--inactive-text);

      &__icon > i {
        width: 60rem;
        font-size: 60rem;
      }

      &__text {
        font-size: 20rem;
      }
    }
  }

  .credits {
    margin-top: 5px;

    & > p > a {
      color: var(--primary);
      font-weight: bold;
      font-size: 15px;
      text-decoration: underline;
    }
  }
}
</style>
