<template>
  <article class="no-padding vod-card">
    <div class="grid no-space">
      <div class="s3">
        <img
            v-if="store.config.showThumb"
            class="responsive"
            :src="vVODData.thumb"
        >
      </div>
      <div
          class="vod-card__details"
          :class="`${store.config.showThumb ? 's9' : 's12'}`"
      >
        <div class="padding">
          <h5>{{ vVODData.title }}</h5>
          <p v-if="store.config.showDesc">{{ vVODData.desc }}</p>
        </div>
        <div
            class="center-content vod-card__details__status"
            :class="vVODData.status"
        >
          <i v-if="statusIcons[vVODData.status]">{{ statusIcons[vVODData.status] }}</i>
        </div>
      </div>
    </div>
  </article>
</template>

<script setup>
// Store
import {useAppStore} from '@/store';

defineProps({
  vVODData: Object
});

// Store
const store = useAppStore();

// Status
const statusIcons = {
  completed: 'check_circle',
  failed: 'error'
};
</script>

<style lang="scss">
.vod-card {
  & img {
    aspect-ratio: 16/9;
    max-width: 300px;
  }

  &__details {
    display: flex;

    &__status {
      min-width: 65px;

      &.failed {
        color: var(--failure);
      }

      &.completed {
        color: var(--success);
      }
    }
  }
}
</style>
