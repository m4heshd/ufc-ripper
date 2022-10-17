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
            v-if="vShowThumb"
            class="responsive small top-round"
            :src="vVODData.thumb"
        >
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
  }

  &__thumbnail {
    &__access-warning {
      flex-direction: column;
      gap: 25px;
      position: absolute;
      z-index: 1;
      top: 0;
      bottom: 0;
      right: 0;
      left: 0;
      background: rgba(0, 0, 0, 0.7);
      backdrop-filter: saturate(200%) blur(2px);

      &__icon {
        border-radius: 50%;
        background: rgba(121, 121, 121, 0.5);
        width: 60px;
        height: 60px;
      }

      &__text {
        margin: 0px 15px;
        text-align: center;
        font-size: 16px;
        font-weight: bold;
        color: var(--primary);
      }
    }

    & > img {
      aspect-ratio: 16/9;
      height: 100% !important;
    }
  }
}
</style>
