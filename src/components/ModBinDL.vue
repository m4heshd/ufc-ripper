<template>
  <div
      id="modBinDL"
      class="modal mod-bin-dl"
  >
    <div class="modal-title">
      <i>warning</i>
      <h5>Download third party tools</h5>
    </div>

    <div class="modal-desc">
      <span>Following essential third party tools seems to be missing.</span>
      <ul>
        <li v-for="bin of store.missingBins">{{ bin }}</li>
      </ul>
      <span>Would you like to download them now?</span>
    </div>

    <nav class="right-align">
      <button
          class="border"
          data-ui="#modBinDL"
      >
        No
      </button>
      <button
          data-ui="#modBinDL"
      >
        Yes
      </button>
    </nav>
  </div>
</template>

<script setup>
// Store
import {useAppStore} from '@/store';

// Store
const store = useAppStore();
</script>

<style lang="scss">
.mod-bin-dl {
  max-width: 420px;

  & > .modal-desc {
    display: flex;
    flex-direction: column;
    gap: 10px;

    & li {
      margin-left: 30px;
      font-weight: bold;
    }
  }
}
</style>
