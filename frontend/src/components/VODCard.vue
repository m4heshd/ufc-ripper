<template>
  <article
      class="no-padding vod-card"
      :class="{'vod-card-failed': isFailed}"
      :title="statusDescs[vVODData.status]"
  >
    <div
        v-if="isDownloading"
        class="vod-card__progress"
    ></div>

    <div class="grid no-space">
      <div class="s3">
        <img
            v-if="vShowThumb"
            class="responsive"
            :class="{'downloading': isDownloading}"
            :src="vVODData.thumb"
        >
      </div>

      <div
          class="vod-card__details"
          :class="`${vShowThumb ? 's9' : 's12'}`"
      >
        <div class="padding vod-card__details__meta">
          <h5>{{ vVODData.title }}</h5>
          <p v-if="vShowDesc">{{ vVODData.desc }}</p>
          <div
              v-if="isDownloading"
              class="vod-card__details__meta__stats"
          >
            <span>{{ taskDescs[vVODData.task] }}</span>
            <div v-if="isData">
              <span>Size: ~{{ vVODData.size }}</span>
              <span>Speed: {{ vVODData.speed }}</span>
              <span>ETA: {{ vVODData.eta }}</span>
            </div>
          </div>
        </div>

        <div
            class="center-content vod-card__details__status"
            :class="`vod-card__details__status-${vVODData.status}`"
        >
          <div
              v-if="isDownloading"
              class="vod-card__details__status__actions"
          >
            <button
                class="square round fill small"
                title="Cancel download"
                @click="$emit('cancelDL',vVODData)"
            >
              <i>close</i>
            </button>
            <span>{{ `${vVODData.progress}%` }}</span>
          </div>
          <div
              v-else
              class="center-content vod-card__details__status__post-action"
          >
            <button
                v-if="isFailed"
                class="square round fill small"
                title="Retry download"
                @click="$emit('retryDL',vVODData)"
            >
              <i>refresh</i>
            </button>
            <i>{{ statusIcons[vVODData.status] }}</i>
          </div>
        </div>
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
  vShowDesc: Boolean
});

// Emits
defineEmits([
  'cancelDL',
  'retryDL'
]);

// Status and progress
const progressBar = computed(() => `0% 0%, 0% 100%, ${props.vVODData.progress}% 100%, ${props.vVODData.progress}% 0%`);
const isDownloading = computed(() => props.vVODData.status === 'downloading');
const isData = computed(() => props.vVODData.task === 'video' || props.vVODData.task === 'audio');
const isFailed = computed(() => props.vVODData.status === 'failed' || props.vVODData.status === 'cancelled');
const statusIcons = {
  completed: 'check_circle',
  failed: 'error',
  cancelled: 'block'
};
const statusDescs = {
  downloading: 'File is being downloaded',
  completed: 'Download is complete',
  failed: 'Download failed',
  cancelled: 'Download cancelled by user'
};
const taskDescs = {
  prepare: 'Preparing download...',
  video: '[video]',
  audio: '[audio]',
  fragErr: 'Fragment error. Retrying...',
  fragSkip: 'Skipping unavailable fragment...',
  merge: 'Merging files...',
  fix: 'Converting container...',
  cleanup: 'Removing temporary files...',
  meta: 'Adding video metadata...'
};
</script>

<style lang="scss">
.vod-card {
  &-failed {
    background: var(--failure-bg) !important;
  }

  &__progress {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 0;
    right: 0;
    background-color: var(--active);
    transition: var(--speed4) clip-path;
    clip-path: polygon(v-bind(progressBar));
  }

  & img {
    aspect-ratio: 16/9;
    max-width: 300rem;

    &.downloading {
      opacity: 0.6;
    }
  }

  &__details {
    display: grid;
    grid-template-columns: auto max-content;

    &__meta {
      &__stats, &__stats > div {
        display: flex;
        gap: 20rem;
        font-weight: bold;
        font-size: 15rem;
        color: var(--warning);
      }
    }

    &__status {
      min-width: 65rem;

      &:hover > &__actions {
        & > span {
          display: none;
        }

        & > button {
          display: block;
        }
      }

      &-failed, &-cancelled {
        color: var(--failure);
      }

      &-completed {
        color: var(--success);
      }

      &__actions {
        & > button {
          display: none;
        }

        & > span {
          font-weight: bold;
          font-size: 15rem;
          color: var(--warning);
        }
      }

      &__post-action {
        flex-direction: column;
        gap: 12rem;

        & > i {
          cursor: default;
        }
      }
    }
  }
}
</style>
