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
        >
        <label>Link</label>
        <span class="helper">Paste the direct link to Fight Pass video here</span>
        <a
            v-if="busy"
            class="loader"
        ></a>
        <i v-else>link</i>
      </div>

      <button
          class="square round large"
          title="Download"
          :disabled="busy"
          @click="onBtnDownloadClick"
      >
        <i>download</i>
      </button>

      <button
          class="square round large"
          title="Get available formats"
          :disabled="busy"
          @click="onBtnGetFmtClick"
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

    <article class="border round dls-section">
      <div class="dls-section__header">
        <h5>Downloads {{ store.activeDownloads ? `(${store.activeDownloads} active)` : '' }}</h5>
        <div class="dls-section__header__actions">
          <button
              class="border square round small"
              title="Open downloads directory"
              @click="onBtnOpenDLDir"
          >
            <i>folder_open</i>
          </button>
          <button
              class="border square round small"
              title="Clear downloads queue"
              @click="onBtnClearDLQueueClick"
          >
            <i>playlist_remove</i>
          </button>
        </div>
      </div>

      <div class="dls-section__downloads">
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
    <ModSupport></ModSupport>

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
import ModVODConfirm from '@/components/ModVODConfirm.vue';
import ModConfig from '@/components/ModConfig.vue';
import ModBinDL from '@/components/ModBinDL.vue';
import ModViewFormats from '@/components/ModViewFormats.vue';
import ModSupport from '@/components/ModSupport.vue';
import Overlay from '@/components/Overlay.vue';

// Store
const store = useAppStore();
const modViewFormats = useModViewFormatsStore();

// Websocket
const {cancelDownload, clearDLQ, downloadVOD, initSocket, openDownloadsDir, verifyURL, getFormats} = useWSUtil();

initSocket();

// Local state
const busy = ref(false);
const switchBusyState = (busyState) => busy.value = busyState === undefined ? !busy.value : busyState;
const verifiedVOD = ref({});

// URL Section
const txtLink = ref('');

function onBtnDownloadClick() {
  if (!store.isLoggedIn) return store.popError('You need to be logged in to download videos');

  switchBusyState();

  verifyURL(txtLink.value)
      .then((res) => {
        verifiedVOD.value = res;

        window.ui('#modVODConfirm');
      })
      .catch(store.popError)
      .finally(switchBusyState);
}

function onBtnGetFmtClick() {
  if (!store.isLoggedIn) return store.popError('You need to be logged in to check download formats');

  switchBusyState();

  getFormats(txtLink.value)
      .then((res) => {
        modViewFormats.setVODData(res);
        window.ui('#modViewFormats');
      })
      .catch(store.popError)
      .finally(switchBusyState);
}

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

// Lifecycle hooks
onMounted(() => nextTick(() => {
  window.ui();
}));

// Misc functions
function download(VOD) {
  switchBusyState();

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
      width: 55vw;
      max-width: 900px;
      margin-bottom: 0;
    }
  }

  .dls-section {
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
    }

    &__downloads {
      margin: 10px;
      overflow: auto;
      border-radius: 0;
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
