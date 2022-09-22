<template>
  <div class="ufcr">
    <div class="logo center-content">
      <img
          alt="UFC Ripper logo"
          src="@/assets/images/ufcr-logo.svg"
      >
    </div>

    <div class="url-section center-content">
      <div class="field label border round url-section__txt-link">
        <input
            type="text"
            autocomplete="off"
        >
        <label>Link</label>
        <span class="helper">Paste the direct link to Fight Pass video here</span>
      </div>
      <button class="square round large">
        <i>download</i>
      </button>
      <button class="square round large">
        <i>settings</i>
      </button>
    </div>

    <article class="border round dls-section">
      <h5>Downloads</h5>

      <div class="dls-section__downloads">
        <VODCard v-for="vod of vods" :v-vod-data="vod"></VODCard>
      </div>
    </article>
  </div>
</template>

<script setup>
// Core
import {ref, inject} from 'vue';
// Components
import VODCard from '@/components/VODCard';

// Socket
const socket = inject('socket');

// Config
const config = ref({});

socket.emit('get-config', (res) => {
  config.value = res;
})

// Downloads
let vods = ref([]);
</script>

<style lang="scss">
@use "~@/assets/styles/app.scss";
@use "~@/assets/styles/common.scss";
@use "~@/assets/styles/overrides.scss";

.ufcr {
  display: grid;
  grid-gap: 50px;
  grid-template-rows: max-content max-content minmax(0px, 1fr);
  margin: 3vw;

  .logo > img {
    height: 70px;
  }

  .url-section {
    gap: 10px;

    &__txt-link {
      width: 60vw;
      max-width: 900px;
      margin-bottom: 0;
    }
  }

  .dls-section {
    display: grid;
    grid-template-rows: max-content minmax(0px, 1fr);
    height: 100%;
    margin: 0;

    & > h5 {
      margin-bottom: 30px;
    }

    &__downloads {
      margin: 10px;
      overflow: auto;
      border-radius: 0;
    }
  }
}
</style>
