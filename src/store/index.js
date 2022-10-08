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
        isLoggedIn: (state) => !!state.modals.modConfig.data.authToken
    },
    actions: {
        popError: (error) => {
            const msg = typeof error === 'string' ? error : error?.message || 'Task failed. Check console for the error information';
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
