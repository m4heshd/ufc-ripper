// Core
import {createApp} from 'vue';
// Modules
import {io} from 'socket.io-client';
import "beercss";
import "material-dynamic-colors";
import Landing from './pages/Landing.vue';

// Socket
const socket = io();

// App theme
window.ui("theme", "#df2722");

createApp(Landing)
    .provide('socket', socket)
    .mount('#app');
