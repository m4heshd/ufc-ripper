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
          @click="onBtnDownloadClick"
          :disabled="busy"
      >
        <i>download</i>
      </button>

      <button
          class="square round large"
          :disabled="busy"
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
    <VODConfirm
        vId="modVODConfirm"
        :vVODData="verifiedVOD"
        @onConfirm="download"
    ></VODConfirm>

    <!-- Overlay -->
    <Overlay :vActive="state.ui.overlay"></Overlay>
  </div>
</template>

<script setup>
// Core
import {ref, inject, nextTick, onMounted} from 'vue';
// Components
import VODCard from '@/components/VODCard';
import VODConfirm from '@/components/VODConfirm';
import Overlay from '@/components/Overlay';

// Injects
const {state, actions} = inject('store');
const socket = inject('socket');

// State
const busy = ref(false);
const setBusy = () => busy.value = true;
const unsetBusy = () => busy.value = false;
const downloadQueue = ref([]);
const verifiedVOD = ref({});

// URL Section
const txtLink = ref('');

function onBtnDownloadClick() {
  setBusy();

  socket.emit('verify-url', txtLink.value, (res) => {
    unsetBusy();
    if (res.error) return actions.popError(res.error);

    txtLink.value = '';
    verifiedVOD.value = res;

    window.ui('#modVODConfirm');
  });
}

// Lifecycle hooks
onMounted(() => nextTick(() => {
  window.ui();
}));

// Misc functions
function download(VOD) {
  setBusy();
  window.ui('#modVODConfirm');

  socket.emit('download', VOD, (res) => {
    unsetBusy();
    if (res.error) return actions.popError(res.error);

    downloadQueue.value.unshift(res);
    actions.popInfo('Download started');
  });
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
