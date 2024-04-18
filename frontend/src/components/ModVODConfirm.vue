<template>
  <div
      :id="vID"
      class="modal mod-vod-confirm"
  >
    <div class="modal-title">
      <i>download</i>
      <h5>Confirm download</h5>
    </div>

    <article
        v-if="vVODData?.title"
        class="no-padding round secondary-container"
    >
      <div class="mod-vod-confirm__thumbnail">
        <div
            v-if="!vVODData.access"
            class="center-content responsive top-round mod-vod-confirm__thumbnail__access-warning"
        >
          <div class="center-content mod-vod-confirm__thumbnail__access-warning__icon">
            <i>lock</i>
          </div>
          <div class="mod-vod-confirm__thumbnail__access-warning__text">
            You need to purchase this video on Fight Pass to get access to download (💰)
          </div>
        </div>
        <img
            ref="imgThumb"
            v-show="!isThumbLoading"
            class="responsive small top-round"
            @load="isThumbLoading = false"
        >
        <VImgSkeleton v-show="isThumbLoading"></VImgSkeleton>
      </div>

      <div class="padding">
        <h5>{{ vVODData.title }}</h5>
        <p v-if="vShowDesc">{{ vVODData.desc }}</p>
      </div>
    </article>

    <nav class="right-align">
      <button
          class="border"
          :data-ui="`#${vID}`"
      >
        Cancel
      </button>
      <button
          class="border"
          title="Check available formats before downloading"
          :data-ui="`#${vID}`"
          @click="$emit('onCheckFormats', vVODData.vodURL)"
      >
        <i>stock_media</i>
      </button>
      <button
          :data-ui="`#${vID}`"
          :disabled="!vVODData.access"
          @click="$emit('onConfirm', vVODData)"
      >
        <i>download</i>
        <span>Download</span>
      </button>
    </nav>
  </div>
</template>

<script setup>
// Core
import {nextTick, ref, watch} from 'vue';
// Components
import VImgSkeleton from '@/components/VImgSkeleton.vue';

// Props
const props = defineProps({
  vID: String,
  vVODData: Object,
  vShowThumb: Boolean,
  vShowDesc: Boolean
});

// Emits
defineEmits([
  'onConfirm',
  'onCheckFormats'
]);

// Thumbnail
const imgThumb = ref(null);
const isThumbLoading = ref(true);

async function loadThumbnailImg() {
  isThumbLoading.value = true;
  imgThumb.value.src = props.vVODData.thumb;
}

// Watchers
watch(() => props.vVODData.thumb, () => nextTick(loadThumbnailImg));
</script>

<style lang="scss">
.mod-vod-confirm {
  max-width: 420rem;

  & > article {
    margin: 25rem 10rem;
  }

  &__thumbnail {
    &__access-warning {
      flex-direction: column;
      gap: 25rem;
      position: absolute;
      z-index: 1;
      top: 0;
      bottom: 0;
      right: 0;
      left: 0;
      background: rgba(0, 0, 0, 0.7);
      backdrop-filter: saturate(200%) blur(2rem);

      &__icon {
        border-radius: 50%;
        background: rgba(121, 121, 121, 0.5);
        width: 60rem;
        height: 60rem;
      }

      &__text {
        margin: 0rem 15rem;
        text-align: center;
        font-size: 16rem;
        font-weight: bold;
        color: var(--primary);
      }
    }

    & > img {
      aspect-ratio: 16/9;
      height: 100% !important;
    }

    & > .v-img-skeleton {
      border-bottom-left-radius: 0;
      border-bottom-right-radius: 0;
    }
  }
}
</style>
