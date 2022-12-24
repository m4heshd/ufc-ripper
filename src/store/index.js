// Store
import {defineStore} from 'pinia';
// Modules
import {useToast} from 'vue-toastification';

// Toast
const toast = useToast();

export const useAppStore = defineStore('app', {
    state: () => ({
        downloads: {},
        config: {},
        ui: {
            overlay: true
        },
        modals: {
            modConfig: {
                data: {}
            },
            modBinDL: {
                data: {
                    bins: {
                        atomicParsley: true,
                        ffmpeg: false,
                        ffprobe: true,
                        ytDlp: false
                    }
                }
            }
        }
    }),
    getters: {
        isLoggedIn: (state) => !!state.config.authToken,
        downloadQueue: (state) => Object.values(state.downloads).sort((a, b) => b.idx - a.idx),
        missingBins: (state) => Object.keys(state.modals.modBinDL.data.bins).filter((bin) => state.modals.modBinDL.data.bins[bin] === false)
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
        },
        addDownload(vod) {
            this.downloads[vod.qID] = {
                ...vod,
                idx: this.downloadQueue.length + 1
            };
        },
        setDownloadCancelled(qID) {
            this.downloads[qID].status = 'cancelled';
        },
        clearDownloadQueue(clearAll = false) {
            if (clearAll) return (this.downloads = {});

            for (const dl in this.downloads) {
                if (this.downloads[dl].status !== 'downloading') delete this.downloads[dl];
            }
        }
    }
});
