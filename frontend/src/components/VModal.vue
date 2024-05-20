<template>
  <Transition name="mod-backdrop">
    <div
        v-if="model"
        class="v-modal-backdrop"
        @click="model = false"
    ></div>
  </Transition>
  <Transition name="mod">
    <div
        v-if="model"
        v-bind="$attrs"
        class="v-modal"
    >
      <div class="modal-title">
        <i>{{ vIcon }}</i>
        <h5>{{ vTitle }}</h5>
      </div>

      <div class="v-modal__content">
        <slot name="content"></slot>
      </div>

      <nav class="right-align v-modal__controls">
        <slot name="controls"></slot>
      </nav>
    </div>
  </Transition>
</template>

<script setup>
// Vue options
defineOptions({
  inheritAttrs: false
});

// Model
const model = defineModel();

// Props
defineProps({
  vIcon: String,
  vTitle: String
});
</script>

<style lang="scss">
.mod-enter-from,
.mod-leave-to {
  opacity: 0;
  transform: translate(-50%, -64rem) !important;
}

.mod-backdrop-enter-from,
.mod-backdrop-leave-to {
  opacity: 0 !important;
}

.v-modal {
  position: fixed;
  display: grid;
  grid-template-rows: max-content minmax(0rem, 1fr) max-content;
  min-width: 320rem;
  max-width: 100%;
  max-height: 80%;
  left: 50%;
  top: 10%;
  overflow-x: hidden;
  overflow-y: auto;
  z-index: 100;
  box-shadow: var(--elevate2);
  color: var(--on-surface);
  background-color: var(--surface);
  padding: 16rem;
  transition: var(--speed3) opacity, var(--speed3) transform;
  transform: translateX(-50%);
  border-radius: 12rem;

  &-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 100;
    color: var(--on-background);
    background-color: var(--overlay);
    backdrop-filter: saturate(200%) blur(2rem);
    transition: var(--speed3) all, 0s background-color;
  }
}
</style>