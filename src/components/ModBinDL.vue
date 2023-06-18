<template>
  <div
      id="modBinDL"
      class="modal mod-bin-dl"
  >
    <div
        v-if="!modBinDL.isDownloading"
        class="modal-title"
    >
      <i>warning</i>
      <h5>Missing third party tools</h5>
    </div>

    <div
        v-else
        class="modal-title"
    >
      <i>download</i>
      <h5>Downloading third party tools</h5>
    </div>

    <div
        v-if="!modBinDL.isDownloading"
        class="modal-desc"
    >
      <span>Following essential third party tools seems to be missing.</span>
      <ul>
        <li v-for="bin of store.missingTools">{{ mediaTools[bin].name }}</li>
      </ul>
      <span>Would you like to download them now?</span>
    </div>

    <div
        v-else
        class="mod-bin-dl__downloads"
    >
      <div
          v-for="(download, tool) in modBinDL.downloads"
          class="mod-bin-dl__downloads__progress"
      >
        <span>{{ mediaTools[tool].name }}</span>
        <VProgressBar :vProgress="download.progress"></VProgressBar>
      </div>
    </div>

    <nav
        v-if="!modBinDL.isDownloading"
        class="right-align"
    >
      <button
          class="border"
          data-ui="#modBinDL"
      >
        No
      </button>
      <button
          @click="onBtnYesClick"
      >
        Yes
      </button>
    </nav>
  </div>
</template>

<script setup>
// Store
import {useAppStore} from '@/store';
import {useModBinDLStore} from '@/store/modBinDL';
// Modules
import {useWSUtil} from '@/modules/ws-util';
// Components
import VProgressBar from '@/components/VProgressBar';

// Store
const store = useAppStore();
const modBinDL = useModBinDLStore();
const mediaTools = store.mediaTools;

// Websocket
const {getMediaTools, validateMediaTools} = useWSUtil();

async function onBtnYesClick() {
  try {
    await getMediaTools(store.missingTools);
    await validateMediaTools();
    if (store.missingTools.length) throw 'Failed to download third-party tools';
    window.ui('#modBinDL');
    store.popSuccess('Third-party tools downloaded successfully');
  } catch (error) {
    store.popError(error);
    modBinDL.resetDownloads();
  }
}
</script>

<style lang="scss">
.mod-bin-dl {
  max-width: 450px;

  & > .modal-desc {
    display: flex;
    flex-direction: column;
    gap: 10px;

    & li {
      margin-left: 30px;
      font-weight: bold;
    }
  }

  &__downloads {
    display: flex;
    flex-direction: column;
    gap: 15px;

    &__progress {
      & > span {
        margin-left: 5px;
        font-weight: bold;
      }

      & > div {
        margin-top: 5px;
      }
    }
  }
}
</style>
