// Store
import {defineStore} from 'pinia';

export const useModBinDLStore = defineStore('modBinDL', {
    state: () => ({
        downloads: {}
    }),
    getters: {
        isDownloading: (state) => !!Object.keys(state.downloads).length
    },
    actions: {
        showModBinDL() {
            window.ui('#modBinDL');
        },
        resetDownloads() {
            this.downloads = {}
        }
    }
});
