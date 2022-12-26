<template>
  <div class="small-space border v-progress-bar">
    <div class="v-progress-bar__progress"></div>
  </div>
</template>

<script setup>
// Core
import {computed} from 'vue';

const props = defineProps({
  vProgress: {
    type: Number,
    default: 0
  }
});

const progress = computed(() => `${props.vProgress}%`);
</script>

<style lang="scss">
.v-progress-bar {
  position: relative;

  &__progress {
    height: 100%;
    width: v-bind(progress);
    background: var(--active);
  }
}
</style>
