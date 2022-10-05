<template>
  <div
      id="modConfig"
      class="modal mod-config"
  >
    <div class="modal-title">
      <i>settings</i>
      <h5>Configuration</h5>
    </div>

    <div class="mod-config__content">
      <article class="border round mod-config__content__section">
        <h5>Account</h5>
        <button :disabled="store.isLoggedIn">Login</button>
        <button
            :disabled="!store.isLoggedIn"
            @click="onBtnLogoutClick"
        >
          Logout
        </button>
      </article>
    </div>

    <nav class="right-align">
      <button
          class="border"
          data-ui="#modConfig"
      >
        Cancel
      </button>
      <button
          data-ui="#modConfig"
          @click="$emit('onSave')"
      >
        <i>save</i>
        <span>Save</span>
      </button>
    </nav>
  </div>
</template>

<script setup>
// Store
import {useAppStore} from '@/store';
// Modules
import {useWSUtil} from '@/modules/ws-util';

defineEmits([
  'onSave'
]);

// Store
const store = useAppStore();
const modConfig = store.modals.modConfig;

// Websocket
const {saveConfig} = useWSUtil();

// Account
function onBtnLogoutClick() {
  modConfig.data.authToken = "";
  modConfig.data.refreshToken = "";
  save();
}

// Misc functions
function save() {
  saveConfig(modConfig.data)
      .then(() => {
        store.popSuccess('Configuration successfully updated');
        window.ui('#modConfig');
      })
      .catch((error) => {
        store.popError(error);
        modConfig.data = JSON.parse(JSON.stringify(store.config));
      });
}
</script>

<style lang="scss">
.mod-config {
  width: 100%;
  max-width: 400px;

  &__content {
    &__section {
      & > h5 {
        font-size: 18px;
        margin-bottom: 15px;
      }

      margin: 15px 0;
    }
  }
}
</style>
