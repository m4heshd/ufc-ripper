<template>
  <div
      :id="vID"
      class="modal mod-msg-box"
  >
    <div class="modal-title">
      <i>{{ vIcon }}</i>
      <h5>{{ vTitle }}</h5>
    </div>

    <div class="mod-msg-box__content">
      <slot></slot>
    </div>

    <nav
        v-if="!vType"
        class="right-align"
    >
      <button
          :data-ui="`#${vID}`"
      >
        <i>close</i>
        <span>Close</span>
      </button>
    </nav>

    <nav
        v-if="vType === 'yes-no'"
        class="right-align"
    >
      <button
          class="border"
          :data-ui="`#${vID}`"
          @click="$emit('onNo')"
      >
        <span>No</span>
      </button>
      <button
          :data-ui="`#${vID}`"
          @click="$emit('onYes')"
      >
        <span>Yes</span>
      </button>
    </nav>
  </div>
</template>

<script setup>
// Props
defineProps({
  vID: String,
  vIcon: String,
  vTitle: String,
  vType: String
});

// Emits
defineEmits([
  'onYes',
  'onNo'
]);
</script>

<style lang="scss">
.mod-msg-box {
  width: 100%;
  max-width: 500px;
  max-height: 80%;
  display: grid;
  grid-template-rows: max-content minmax(0px, 1fr) max-content;
  overflow: hidden;

  &__content {
    overflow-y: auto;
    margin: 10px 0;
    border-radius: 0;
  }
}
</style>