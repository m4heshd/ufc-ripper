// Core
import {reactive} from "vue";
// Modules
import {useToast} from 'vue-toastification';

// Toast
const toast = useToast();

const state = reactive({
    config: {},
    ui: {
        overlay: true
    }
});

const actions = {
    showOverlay: () => state.ui.overlay = true,
    hideOverlay: () => state.ui.overlay = false,
    popError: (error) => {
        const msg = typeof error === 'string' ? error : error?.message || 'Task failed. Check console for the error information';
        toast.error(msg);
        console.error(error || msg);
    },
    popInfo: msg => toast.info(msg)
};

export default {state, actions};
