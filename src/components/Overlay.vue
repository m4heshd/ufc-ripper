<template>
  <div
      class="overlay"
      :class="{'active': vActive}"
  >
    <div class="overlay__content center-content">
      <a class="loader large"></a>
      <h5>{{ vMsg }}</h5>
    </div>
  </div>
</template>

<script>
export default {
  name: "Overlay"
};
</script>

<script setup>
defineProps({
  vActive: {
    type: Boolean,
    default: false
  },
  vMsg: {
    type: String,
    default: 'Looking for the backend... 🤷'
  }
});
</script>

<style lang="scss">
.overlay {
  &__content {
    width: 100%;
    height: 100%;
    flex-direction: column;
    gap: 15px;
  }
}
</style>
