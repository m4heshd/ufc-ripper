<template>
  <article class="no-padding vod-card">
    <div class="grid no-space">
      <div class="s3">
        <img
            class="responsive"
            :src="vVodData.thumb"
        >
      </div>
      <div class="s9">
        <div class="padding">
          <h5>{{ vVodData.title }}</h5>
          <p>{{ vVodData.desc }}</p>
        </div>
      </div>
    </div>
  </article>
</template>

<script>
export default {
  name: "VODCard"
};
</script>

<script setup>
defineProps({
  vVodData: Object
});
</script>

<style lang="scss">
.vod-card {
  & img {
    aspect-ratio: 16/9;
    max-width: 300px;
  }
}
</style>
