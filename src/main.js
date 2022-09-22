import {createApp} from 'vue';
import Landing from './pages/Landing.vue';
import "beercss";
import "material-dynamic-colors";

// App theme
ui("theme", "#df2722");

createApp(Landing).mount('#app');
