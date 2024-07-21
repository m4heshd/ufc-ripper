<template>
  <VModal
      class="mod-play-vod"
      v-model="modPlayVOD.active"
      vIcon="live_tv"
      :vTitle="modPlayVOD.VOD.title"
  >
    <template #content>
      <VMediaPlayer
          :vPoster="modPlayVOD.VOD.thumb"
          :v-source="modPlayVOD.VOD.hls"
      ></VMediaPlayer>
    </template>

    <template #controls>
      <button
          class="border"
          title="Check available formats for this video"
          @click="$emit('onCheckFormats', modPlayVOD.VOD.vodURL)"
      >
        <i>stock_media</i>
      </button>
      <button
          class="border"
          title="Download this video"
          @click="$emit('onDownload', modPlayVOD.VOD)"
      >
        <i>download</i>
      </button>
      <button @click="modPlayVOD.close()">
        <i>close</i>
        <span>Close</span>
      </button>
    </template>
  </VModal>
</template>

<script setup>
// Store
import {useModPlayVODStore} from '@/store/modPlayVOD';
// Components
import VModal from '@/components/VModal.vue';
import VMediaPlayer from '@/components/VMediaPlayer.vue';

// Emits
defineEmits([
  'onDownload',
  'onCheckFormats'
]);

// Store
const modPlayVOD = useModPlayVODStore();
</script>

<style lang="scss">
.mod-play-vod {
  width: 100%;
  max-width: 950rem;
  max-height: 80%;
  overflow: hidden;

  .v-media-player {
    height: 100%;

    & > .plyr {
      height: 100%;
      max-height: 100%;
    }
  }
}
</style>