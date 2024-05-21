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
      <button @click="modPlayVOD.closeModPlayVOD()">
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

// Store
const modPlayVOD = useModPlayVODStore();
</script>

<style lang="scss">
.mod-play-vod {
  width: 100%;
  max-width: 950rem;
  max-height: 80%;
  overflow: hidden;
}
</style>