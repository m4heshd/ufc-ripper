<template>
  <div class="v-media-player">
    <video
        ref="video"
        :data-poster="vPoster"
        controls
        crossorigin
        playsinline
    >
    </video>
  </div>
</template>

<script setup>
// Core
import {nextTick, onBeforeUnmount, onMounted, ref} from 'vue';
// Modules
import Plyr from 'plyr';
import Hls from 'hls.js';

// Props
const props = defineProps({
  vPoster: String,
  vSource: String,
});

// Template refs
const video = ref(null);

// Video player setup
let player = null;
let hls = null;

function onQualityChange(quality) {
  if (quality === 0) {
    hls.currentLevel = -1;
  } else {
    hls.levels.forEach((level, levelIndex) => {
      if (level.height === quality) {
        hls.currentLevel = levelIndex;
      }
    });
  }
}

function initPlayer(resolutions) {
  player = new Plyr(video.value, {
    captions: {
      active: false,
      update: true,
      language: 'en'
    },
    quality: {
      default: 0,
      options: resolutions,
      forced: true,
      onChange: (quality) => onQualityChange(quality)
    },
    i18n: {
      qualityLabel: {
        0: 'Auto'
      }
    },
    keyboard: {
      global: true
    }
  });

  player.on('languagechange', () => {
    setTimeout(() => hls.subtitleTrack = player.currentTrack, 50);
  });

  hls.attachMedia(video.value);
}

function playSource(source) {
  if (!Hls.isSupported()) {
    video.src = source;
  } else {
    hls = new Hls();
    hls.loadSource(source);

    hls.on(Hls.Events.MANIFEST_PARSED, () => {
      const resolutions = hls.levels.map((l) => l.height);
      resolutions.unshift(0);

      initPlayer(resolutions);
    });
  }
}

// Lifecycle hooks
onMounted(() =>
    nextTick(() =>
        playSource(props.vSource)
    )
);

onBeforeUnmount(() => {
  player.destroy(() => {
    hls.destroy();
  }, false);
});
</script>

<style lang="scss">
.v-media-player {
  min-width: 100%;
  --plyr-color-main: var(--primary);

  video {
    height: 100%;
    min-width: 100%;
    aspect-ratio: 16 / 9;
    object-fit: cover;
  }

  * {
    all: revert;

    &:after {
      all: revert;
    }
  }
}

@import "plyr/dist/plyr";
</style>