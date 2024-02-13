<template>
  <article class="block-vod-card">
    <div class="block-vod-card__details">
      <div class="block-vod-card__details__thumbnail">
        <div
            v-if="vShowDuration"
            class="block-vod-card__details__thumbnail__duration"
        >
          <div class="block-vod-card__details__thumbnail__duration__clock">
            {{ duration }}
          </div>
        </div>

        <div class="block-vod-card__details__thumbnail__actions">
          <div class="center-content block-vod-card__details__thumbnail__actions__buttons">
            <button
                class="circle fill medium"
                title="Download"
                :disabled="vBusyState"
                @click="$emit('download', vVODData.id)"
            >
              <i>download</i>
            </button>
            <button
                class="circle fill medium"
                title="Get available formats"
                :disabled="vBusyState"
                @click="$emit('getFormats', vVODData.id)"
            >
              <i>stock_media</i>
            </button>
            <button
                class="circle fill medium"
                title="Open in Fight Pass"
                :disabled="vBusyState"
                @click="$emit('openExternal', vVODData.id)"
            >
              <i>open_in_new</i>
            </button>
          </div>

          <div class="block-vod-card__details__thumbnail__actions__bg"></div>
        </div>

        <img
            ref="imgThumb"
            v-show="!isThumbLoading"
            :class="{'blur': !vShowThumb}"
            @load="isThumbLoading = false"
        >
        <VImgSkeleton v-show="isThumbLoading"></VImgSkeleton>
      </div>

      <div
          class="block-vod-card__details__title"
          v-html="vVODData._highlightResult.name.value"
      ></div>

      <div
          v-if="vShowDesc"
          class="block-vod-card__details__description"
          :title="vVODData.description"
          v-html="vVODData._highlightResult.description.value"
      ></div>
    </div>
  </article>
</template>

<script setup>
// Core
import {computed, nextTick, onMounted, ref, watch} from 'vue';
// Components
import VImgSkeleton from '@/components/VImgSkeleton.vue';

// Props
const props = defineProps({
  vVODData: Object,
  vShowThumb: Boolean,
  vShowDuration: Boolean,
  vShowDesc: Boolean,
  vBusyState: Boolean
});

// Emits
defineEmits([
  'download',
  'getFormats',
  'openExternal'
]);

// Thumbnail
const imgThumb = ref(null);
const isThumbLoading = ref(true);

async function loadThumbnailImg() {
  isThumbLoading.value = true;
  imgThumb.value.src = props.vVODData.thumbnailUrl;
}

// Duration
const duration = computed(() => {
  const fullDuration =
      new Date(props.vVODData.duration * 1000)
          .toISOString()
          .substring(11, 19);

  return fullDuration.startsWith('00') ? fullDuration.slice(3) : fullDuration;
});

// Watchers
watch(() => props.vVODData.thumbnailUrl, loadThumbnailImg);

// Lifecycle hooks
onMounted(() => nextTick(loadThumbnailImg));
</script>

<style lang="scss">
.block-vod-card {
  display: grid;
  grid-gap: 10px;
  width: 250px;
  height: 100%;

  &__details {
    display: grid;
    grid-template-rows: max-content max-content minmax(0px, 1fr);
    grid-gap: 10px;

    &__thumbnail {
      overflow: hidden;

      &__duration {
        z-index: 1;
        position: absolute;
        top: 10px;
        right: 10px;
        border-radius: 5px;
        background: rgba(0, 0, 0, 0.55);

        &__clock {
          padding: 3px;
          font-size: 12rem;
          font-weight: bold;
        }
      }

      &__actions {
        z-index: 1;
        position: absolute;
        width: 100%;
        height: 100%;
        border-radius: 0;
        opacity: 0;
        transition: opacity .2s ease-in-out;

        $actions: &;

        &:hover {
          opacity: 1;

          & > #{$actions}__buttons {
            transform: none;
          }

          & + img {
            transition: transform 3s ease-out;
            transform: scale(1.15);
          }
        }

        &__buttons {
          position: absolute;
          bottom: 0;
          z-index: 1;
          width: 100%;
          height: 55%;
          transform: translateY(20px);
          transition: transform .2s ease-in-out;
        }

        &__bg {
          width: 100%;
          height: 100%;
          border-radius: 0;
          background: rgba(0, 0, 0, 0.9);
          mask-image: linear-gradient(0deg, #000 30%, transparent);
        }
      }

      & > img {
        aspect-ratio: 16/9;
        object-fit: cover;
        width: 100%;
        max-width: 100%;
        transition: transform .2s ease-in-out;

        &.blur {
          filter: blur(15px);
        }
      }
    }

    &__title, &__description {
      & > em {
        all: unset;
        color: var(--primary);
      }
    }

    &__title {
      font-size: 15rem;
      font-weight: bold;
    }

    &__description {
      height: 80px;
      border-radius: 0;
      overflow: hidden;
      mask-image: linear-gradient(180deg, #000 60%, transparent);
      font-size: 13rem;
      opacity: 0.85;
    }
  }
}
</style>