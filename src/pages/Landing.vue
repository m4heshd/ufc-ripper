<template>
  <div class="ufcr">
    <div class="logo center-content">
      <img
          alt="UFC Ripper logo"
          src="@/assets/images/ufcr-logo.svg"
      >
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
          :disabled="busy"
          @click="onBtnDownloadClick"
      >
        <i>download</i>
      </button>

      <button
          class="square round large"
          :disabled="busy"
          @click="onBtnConfigClick"
      >
        <i>settings</i>
      </button>
    </div>

    <article class="border round dls-section">
      <h5>Downloads</h5>

      <div class="dls-section__downloads">
        <VODCard
            v-for="vod of downloadQueue"
            :vVODData="vod"
        ></VODCard>
      </div>
    </article>

    <!-- Modals -->
    <ModVODConfirm
        vId="modVODConfirm"
        :vVODData="verifiedVOD"
        @onConfirm="download"
    ></ModVODConfirm>
    <ModConfig></ModConfig>

    <!-- Overlay -->
    <Overlay :vActive="store.ui.overlay"></Overlay>
  </div>
</template>

<script setup>
// Core
import {ref, nextTick, onMounted} from 'vue';
// Store
import {useAppStore} from '@/store';
// Modules
import {useWSUtil} from '@/modules/ws-util';
// Components
import VODCard from '@/components/VODCard';
import ModVODConfirm from '@/components/ModVODConfirm';
import ModConfig from '@/components/ModConfig';
import Overlay from '@/components/Overlay';

// Store
const store = useAppStore();
const modConfig = store.modals.modConfig;

// Websocket
const {downloadVOD, initSocket, verifyURL} = useWSUtil();

initSocket();

// Local state
const busy = ref(false);
const switchBusyState = (busyState) => busy.value = busyState === undefined ? !busy.value : busyState;
const downloadQueue = ref([]);
const verifiedVOD = ref({});

// URL Section
const txtLink = ref('');

function onBtnDownloadClick() {
  switchBusyState();

  verifyURL(txtLink.value)
      .then((res) => {
        txtLink.value = '';
        verifiedVOD.value = res;

        window.ui('#modVODConfirm');
      })
      .catch(store.popError)
      .finally(switchBusyState);
}

function onBtnConfigClick() {
  modConfig.data = JSON.parse(JSON.stringify(store.config));
  window.ui('#modConfig');
}

// Lifecycle hooks
onMounted(() => nextTick(() => {
  window.ui();
}));

// Misc functions
function download(VOD) {
  switchBusyState();

  downloadVOD(VOD)
      .then((res) => {
        downloadQueue.value.unshift(res);
        store.popInfo('Download started');
      })
      .catch(store.popError)
      .finally(switchBusyState);
}
</script>

<style lang="scss">
@use "~@/assets/styles/app.scss";
@use "~@/assets/styles/common.scss";
@use "~@/assets/styles/overrides.scss";

.ufcr {
  display: grid;
  grid-gap: 50px;
  grid-template-rows: max-content max-content minmax(0px, 1fr);
  justify-items: center;
  margin: 3vw;

  .logo > img {
    height: 70px;
  }

  .url-section {
    gap: 10px;

    &__txt-link {
      width: 60vw;
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

    & > h5 {
      margin-bottom: 30px;
    }

    &__downloads {
      margin: 10px;
      overflow: auto;
      border-radius: 0;
    }
  }
}
</style>
