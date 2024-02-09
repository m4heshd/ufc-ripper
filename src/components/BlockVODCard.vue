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
                @click="$emit('download', vVODData.id)"
            >
              <i>download</i>
            </button>
            <button
                class="circle fill medium"
                title="Open in Fight Pass"
                @click="$emit('openExternal', vVODData.id)"
            >
              <i>open_in_new</i>
            </button>
          </div>

          <div class="block-vod-card__details__thumbnail__actions__bg"></div>
        </div>

        <img
            :class="{'blur': !vShowThumb}"
            :src="vVODData.thumbnailUrl"
        >
      </div>

      <div class="block-vod-card__details__title">
        {{ vVODData.name }}
      </div>

      <div
          v-if="vShowDesc"
          class="block-vod-card__details__description"
          :title="vVODData.description"
      >
        {{ vVODData.description }}
      </div>
    </div>
  </article>
</template>

<script setup>
// Core
import {computed} from 'vue';

// Props
const props = defineProps({
  vVODData: Object,
  vShowThumb: Boolean,
  vShowDuration: Boolean,
  vShowDesc: Boolean
});

// Emits
defineEmits([
  'download',
  'openExternal'
]);

// Duration
const duration = computed(() => {
  const fullDuration =
      new Date(props.vVODData.duration * 1000)
          .toISOString()
          .substring(11, 19);

  return fullDuration.startsWith('00') ? fullDuration.slice(3) : fullDuration;
});
</script>

<style lang="scss">
.block-vod-card {
  display: grid;
  grid-gap: 10px;
  width: 250px;
  max-height: 288px;

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

          #{$actions}__buttons {
            transform: none;
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
        max-width: 100%;

        &.blur {
          filter: blur(15px);
        }
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