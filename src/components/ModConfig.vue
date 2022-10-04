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
        <button :disabled="loggedIn">Login</button>
        <button
            :disabled="!loggedIn"
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
// Core
import {inject, computed} from 'vue';
// Modules
import {useWSUtil} from '@/modules/ws-util';

defineEmits([
  'onSave'
]);

// Injects
const {state, actions} = inject('store');

// Composables
const {saveConfig} = useWSUtil();

// Account
const loggedIn = computed(() => !!state.modals.modConfig.data.authToken);

function onBtnLogoutClick() {
  state.modals.modConfig.data.authToken = "";
  save();
}

// Misc functions
function save() {
  saveConfig(state.modals.modConfig.data)
      .then(() => {
        actions.popSuccess('Configuration successfully updated');
        window.ui('#modConfig');
      })
      .catch((error) => {
        actions.popError(error);
        state.modals.modConfig.data = JSON.parse(JSON.stringify(state.config));
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
