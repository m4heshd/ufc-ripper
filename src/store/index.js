// Store
import {defineStore} from 'pinia';
// Modules
import {useToast} from 'vue-toastification';

// Toast
const toast = useToast();

export const useAppStore = defineStore('app', {
    state: () => ({
        config: {},
        ui: {
            overlay: true
        },
        modals: {
            modConfig: {
                data: {}
            }
        }
    }),
    getters: {
        isLoggedIn: (state) => !!state.config.authToken
    },
    actions: {
        popError: (error) => {
            const msg = typeof error === 'string' ?
                error :
                error?.userMsg || error?.message || 'Task failed. Check the console for error information';
            toast.error(msg);
            console.error(error || msg);
        },
        popInfo: msg => toast.info(msg),
        popSuccess: msg => toast.success(msg),
        showOverlay() {
            this.ui.overlay = true;
        },
        hideOverlay() {
            this.ui.overlay = false;
        },
        showModConfig() {
            this.modals.modConfig.data = JSON.parse(JSON.stringify(this.config));
            window.ui('#modConfig');
        }
    }
});
