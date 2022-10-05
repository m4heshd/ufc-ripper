// Core
import {createApp} from 'vue';
// Store
import {createPinia} from 'pinia';
// Modules
import Toast, {POSITION} from "vue-toastification";
import "vue-toastification/dist/index.css";
import "beercss";
import "material-dynamic-colors";
import Landing from './pages/Landing.vue';

// App theme
window.ui("theme", "#df2722");

// Toast
const toastOptions = {
    position: POSITION.BOTTOM_RIGHT,
    timeout: 5000
};

createApp(Landing)
    .use(createPinia())
    .use(Toast, toastOptions)
    .mount('#app');
