// Core
import {createApp} from 'vue';
// Store
import {state, actions} from '@/store';
// Modules
import Toast, {POSITION} from "vue-toastification";
import "vue-toastification/dist/index.css";
import {io} from 'socket.io-client';
import "beercss";
import "material-dynamic-colors";
import Landing from './pages/Landing.vue';

// Socket and config
const socket = io();

socket.on("connect", () => {
    socket.emit('get-config', (res) => {
        if (res.error) return console.error(res.error);
        state.config = res;
    });
    actions.hideOverlay();
});
socket.on("disconnect", actions.showOverlay);

// App theme
window.ui("theme", "#df2722");

// Toast
const toastOptions = {
    position: POSITION.BOTTOM_RIGHT,
    timeout: 5000
};

createApp(Landing)
    .use(Toast, toastOptions)
    .provide('socket', socket)
    .provide('store', {state, actions})
    .mount('#app');
