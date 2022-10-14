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
        <h5>Fight Pass account</h5>
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

      <article class="border round mod-config__content__section">
        <h5>Previews (spoilers)</h5>
        <nav class="v-switch">
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
        <nav class="v-switch">
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

      <article class="border round mod-config__content__section mod-config__content__downloads">
        <h5>Downloads (output)</h5>
        <div class="field label suffix border round small">
          <input
              v-model="modConfig.data.dlPath"
              type="text"
              autocomplete="off"
          >
          <label>Location (directory)</label>
          <i>folder_open</i>
        </div>
        <nav class="v-switch">
          <div class="max">
            <h6>Number files</h6>
            <div>Number each file increasingly with each download</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.numberFiles"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
        <div class="short-text">
          <span>Current file number:</span>
          <div class="field border round small no-margin">
            <input
                v-model.number="modConfig.data.curNumber"
                type="number"
                min="1"
            >
          </div>
        </div>
        <nav class="v-switch">
          <div class="max">
            <h6>Throttle downloads</h6>
            <div>Limit the speed of each download to the following</div>
          </div>
          <label class="switch">
            <input
                v-model="modConfig.data.throttle"
                type="checkbox"
            >
            <span></span>
          </label>
        </nav>
        <div class="short-text">
          <span>Download speed:</span>
          <div class="field border round small no-margin">
            <input
                v-model="modConfig.data.dlRate"
                type="text"
            >
          </div>
        </div>
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
  max-width: 480px;
  max-height: 80%;
  display: grid;
  grid-template-rows: max-content minmax(0px, 1fr) max-content;
  overflow: hidden;

  & .v-switch > div {
    & > h6 {
      font-size: 15px;
      font-weight: bold;
    }

    & > div {
      font-size: 11px;
    }
  }

  &__content {
    overflow-y: auto;

    &__section {
      margin: 15px 0;

      & > h5 {
        margin-bottom: 15px;
        color: var(--primary);
        font-size: 18px;
        font-weight: bold;
      }

      & .short-text {
        display: flex;
        align-items: center;
        gap: 5px;
        margin: 16rem 0 0 0;

        & > div {
          width: 100px;
        }
      }
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

    &__downloads {
      & > .field {
        margin-top: 25px;
        margin-bottom: 10px;
      }
    }
  }
}
</style>
