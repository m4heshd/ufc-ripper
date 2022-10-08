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
      <article class="border round mod-config__content__section mod-config__content__account">
        <h5>Account</h5>
        <div
            v-if="store.isLoggedIn"
            class="mod-config__content__account__user"
        >
          <i>person</i>
          {{ modConfig.data.user }}
        </div>
        <div
            v-else
            class="mod-config__content__account__creds"
        >
          <div class="field label small border round">
            <input
                v-model="txtEmail"
                type="text"
                autocomplete="off"
                :disabled="busy"
            >
            <label>Email</label>
          </div>
          <div class="field label small border round">
            <input
                v-model="txtPass"
                type="password"
                autocomplete="off"
                :disabled="busy"
            >
            <label>Password</label>
          </div>
        </div>
        <button
            :disabled="!store.isLoggedIn"
            @click="onBtnLogoutClick"
        >
          Logout
        </button>
        <button
            :disabled="store.isLoggedIn || busy"
            @click="onBtnLoginClick"
        >Login
        </button>
      </article>

      <article class="border round mod-config__content__section mod-config__content__previews">
        <h5>Previews (spoilers)</h5>
        <nav>
          <div class="max">
            <h6>Show thumbnails</h6>
            <div>Thumbnail image on previews and downloads</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.showThumb"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
        <nav>
          <div class="max">
            <h6>Show description</h6>
            <div>Bout/event description from Fight Pass</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.showDesc"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
      </article>
    </div>

    <nav class="right-align">
      <button
          class="border"
          data-ui="#modConfig"
      >
        Cancel
      </button>
      <button @click="save">
        <i>save</i>
        <span>Save</span>
      </button>
    </nav>
  </div>
</template>

<script setup>
// Core
import {ref} from 'vue';
// Store
import {useAppStore} from '@/store';
// Modules
import {useWSUtil} from '@/modules/ws-util';

// Store
const store = useAppStore();
const modConfig = store.modals.modConfig;

// Local state
const busy = ref(false);
const switchBusyState = (busyState) => busy.value = busyState === undefined ? !busy.value : busyState;
const fail = (error) => {
  store.popError(error);
  modConfig.data = JSON.parse(JSON.stringify(store.config));
};

// Websocket
const {login, saveConfig} = useWSUtil();

// Account
const txtEmail = ref('');
const txtPass = ref('');

function onBtnLogoutClick() {
  modConfig.data.user = "";
  modConfig.data.authToken = "";
  modConfig.data.refreshToken = "";
  save();
}

function onBtnLoginClick() {
  switchBusyState();
  login(txtEmail.value, txtPass.value)
      .then(() => {
        store.popSuccess('Successfully logged in');
        window.ui('#modConfig');
      })
      .catch(fail)
      .finally(switchBusyState);
}

// Misc functions
function save() {
  saveConfig(modConfig.data)
      .then(() => {
        store.popSuccess('Configuration successfully updated');
        window.ui('#modConfig');
      })
      .catch(fail);
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
        font-weight: bold;
        margin-bottom: 15px;
      }

      margin: 15px 0;
    }

    &__account {
      &__user {
        margin-bottom: 20px;
      }

      &__creds {
        & > div:first-child {
          margin-bottom: 5px;
        }
      }
    }

    &__previews {
      & > nav > div {
        & > h6 {
          font-size: 15px;
          font-weight: bold;
        }

        & > div {
          font-size: 11px;
        }
      }
    }
  }
}
</style>
