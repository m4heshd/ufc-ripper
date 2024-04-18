<template>
  <div
      id="modViewFormats"
      class="modal mod-view-formats"
  >
    <div class="modal-title">
      <i>stock_media</i>
      <h5>Available download formats</h5>
    </div>

    <div class="mod-view-formats__content">
      <div class="vod-info">
        <span class="vod-info__title">{{ modViewFormats.vodData.VOD.title }}:</span>
      </div>

      <article class="border round">
        <div class="formats-table">
          <table>
            <thead>
            <tr>
              <th>Format ID</th>
              <th>Resolution</th>
              <th>Framerate</th>
              <th>Bitrate</th>
              <th title="Video Codec">VCodec</th>
              <th title="Audio Codec">ACodec</th>
              <th></th>
            </tr>
            </thead>
            <tbody>
            <tr v-for="format in modViewFormats.vodData.formats">
              <td :class="{ 'grayed-out': isInvalid(format.format_id)}">{{ format.format_id || 'N/A' }}</td>
              <td :class="{ 'grayed-out': isInvalid(format.resolution)}">{{ format.resolution || 'N/A' }}</td>
              <td :class="{ 'grayed-out': isInvalid(format.fps)}">{{ format.fps || 'N/A' }}</td>
              <td :class="{ 'grayed-out': isInvalid(format.tbr)}">{{ getBitrate(format.tbr) }}</td>
              <td :class="{ 'grayed-out': isInvalid(format.vcodec)}">{{ format.vcodec || 'N/A' }}</td>
              <td :class="{ 'grayed-out': isInvalid(format.acodec)}">{{ format.acodec || 'N/A' }}</td>
              <td>
                <button
                    v-if="format.vcodec !== 'none'"
                    class="circle fill medium"
                    :title="`Download (${format.resolution} at ${getBitrate(format.tbr)} bitrate)`"
                    @click="$emit('download', modViewFormats.vodData.VOD, format)"
                >
                  <i>download</i>
                </button>
              </td>
            </tr>
            </tbody>
          </table>
        </div>
      </article>

      <div class="instructions">
        You can use any of the above metadata as the custom download format in the configuration. Please read more about
        format selection in the
        <VAnchor href="https://github.com/yt-dlp/yt-dlp#format-selection">yt-dlp documentation</VAnchor>
        before using this feature.
      </div>
    </div>

    <nav class="right-align">
      <button data-ui="#modViewFormats">
        <i>close</i>
        <span>Close</span>
      </button>
    </nav>
  </div>
</template>

<script setup>
// Store
import {useModViewFormatsStore} from '@/store/modViewFormats';
// Components
import VAnchor from '@/components/VAnchor.vue';

// Emits
defineEmits([
  'download'
]);

// Store
const modViewFormats = useModViewFormatsStore();

// Table
function isInvalid(data) {
  return (data == null || data === 'none');
}

function getBitrate(tbr) {
  return tbr ? Math.trunc(tbr) + 'k' : 'N/A';
}
</script>

<style lang="scss">
.mod-view-formats {
  width: 100%;
  max-width: 750rem;
  max-height: 80%;
  display: grid;
  grid-template-rows: max-content minmax(0rem, 1fr) max-content;
  overflow: hidden;

  &__content {
    overflow-y: auto;

    .vod-info {
      margin: 25rem 0;

      &__title {
        font-size: 24rem;
        font-weight: bold;
      }
    }

    .formats-table {
      overflow-x: auto;
      border-radius: 0;

      th {
        font-size: 15rem;
        font-weight: bold;
        color: var(--primary);
        border-bottom: 1rem solid var(--primary);
      }

      td {
        &:first-child {
          color: var(--secondary-text);
          max-width: 150rem;
          overflow: hidden;
        }

        &:last-child {
          max-width: 40rem;
        }

        &.grayed-out {
          color: var(--inactive-text);
        }

        & > button {
          margin: 0;
        }
      }
    }

    .instructions {
      margin: 25rem 0;
      font-size: 15rem;
    }
  }
}
</style>
