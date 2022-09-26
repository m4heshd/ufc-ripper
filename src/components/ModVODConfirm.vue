<template>
  <div
      :id="vId"
      class="modal mod-vod-confirm"
  >
    <h5>Confirm download</h5>

    <article
        v-if="vVODData?.title"
        class="no-padding round secondary-container"
    >
      <img
          class="responsive small top-round"
          :src="vVODData.thumb"
      >
      <div class="padding">
        <h5>{{ vVODData.title }}</h5>
        <p>{{ vVODData.desc }}</p>
      </div>
    </article>

    <nav class="right-align">
      <button
          class="border"
          :data-ui="`#${vId}`"
      >
        Cancel
      </button>
      <button
          :data-ui="`#${vId}`"
          @click="$emit('onConfirm', vVODData)"
      >
        <i>download</i>
        <span>Download</span>
      </button>
    </nav>
  </div>
</template>

<script setup>
defineProps({
  vId: String,
  vVODData: Object
});

defineEmits([
  'onConfirm'
]);
</script>

<style lang="scss">
.mod-vod-confirm {
  max-width: 380px;

  & > article {
    margin: 25px 10px;

    & > img {
      aspect-ratio: 16/9;
      height: 100% !important;
    }
  }
}
</style>
