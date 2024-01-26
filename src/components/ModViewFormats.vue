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
        <span class="vod-info__title">{{ modViewFormats.vodData.title }}:</span>
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
              <th>VCodec</th>
              <th>ACodec</th>
            </tr>
            </thead>
            <tbody>
            <tr v-for="format in modViewFormats.vodData.formats">
              <td :class="{ 'grayed-out': isInvalid(format.format_id)}">{{ format.format_id || 'N/A' }}</td>
              <td :class="{ 'grayed-out': isInvalid(format.resolution)}">{{ format.resolution || 'N/A' }}</td>
              <td :class="{ 'grayed-out': isInvalid(format.fps)}">{{ format.fps || 'N/A' }}</td>
              <td :class="{ 'grayed-out': isInvalid(format.tbr)}">
                {{ format.tbr ? Math.trunc(format.tbr) + 'k' : 'N/A' }}
              </td>
              <td :class="{ 'grayed-out': isInvalid(format.vcodec)}">{{ format.vcodec || 'N/A' }}</td>
              <td :class="{ 'grayed-out': isInvalid(format.acodec)}">{{ format.acodec || 'N/A' }}</td>
            </tr>
            </tbody>
          </table>
        </div>
      </article>

      <div class="instructions">
        You can use any of the above metadata as the custom download format in the configuration. Please read more about
        format selection in the <a href="https://github.com/yt-dlp/yt-dlp#format-selection">yt-dlp documentation</a>
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

// Store
const modViewFormats = useModViewFormatsStore();

// Table
function isInvalid(data) {
  return (data == null || data === 'none');
}
</script>

<style lang="scss">
.mod-view-formats {
  width: 100%;
  max-width: 700px;
  max-height: 80%;
  display: grid;
  grid-template-rows: max-content minmax(0px, 1fr) max-content;
  overflow: hidden;

  &__content {
    overflow-y: auto;

    .vod-info {
      margin: 25px 0;

      &__title {
        font-size: 24rem;
        font-weight: bold;
      }
    }

    .formats-table {
      th {
        font-size: 15rem;
        font-weight: bold;
        color: var(--primary);
        border-bottom: 1px solid var(--primary);
      }

      td {
        &:first-child {
          color: var(--secondary-text);
        }

        &.grayed-out {
          color: var(--inactive-text);
        }
      }
    }

    .instructions {
      margin: 25px 0;
      font-size: 15rem;

      & > a {
        color: var(--primary);
        text-decoration: underline;
      }
    }
  }
}
</style>
