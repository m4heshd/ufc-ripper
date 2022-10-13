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
      <img
          v-if="vShowThumb"
          class="responsive small top-round"
          :src="vVODData.thumb"
      >
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
          :data-ui="`#${vID}`"
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
  vID: String,
  vVODData: Object,
  vShowThumb: Boolean,
  vShowDesc: Boolean
});

defineEmits([
  'onConfirm'
]);
</script>

<style lang="scss">
.mod-vod-confirm {
  max-width: 420px;

  & > article {
    margin: 25px 10px;

    & > img {
      aspect-ratio: 16/9;
      height: 100% !important;
    }
  }
}
</style>
