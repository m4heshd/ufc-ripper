// Store
import {defineStore} from 'pinia';
// Modules
import {useToast} from 'vue-toastification';

// Toast
const toast = useToast();

export const useAppStore = defineStore('app', {
    state: () => ({
        appMeta: {
            isContainer: false,
            version: ''
        },
        update: {
            updatable: false,
            version: '',
            url: ''
        },
        fpMeta: {
            domain: {
                "dce.ufc": "ufcfightpass.com",
                "dce.ufcbrazil": "ufcfightpass.com.br",
            }
        },
        downloads: {},
        search: {
            showResults: false,
            result: {
                hits: [],
                page: 0,
                nbPages: 0
            }
        },
        config: {},
        mediaTools: {
            atomicParsley: {
                name: 'AtomicParsley',
                avail: false
            },
            ffmpeg: {
                name: 'FFmpeg',
                avail: false
            },
            ffprobe: {
                name: 'FFprobe',
                avail: false
            },
            ytDlp: {
                name: 'yt-dlp',
                avail: false
            }
        },
        ui: {
            overlay: true
        },
        modals: {
            modConfig: {
                data: {
                    proxyConfig: {
                        auth: {}
                    }
                }
            }
        }
    }),
    getters: {
        isLoggedIn: (state) => !!state.config.authToken,
        downloadQueue: (state) => Object.values(state.downloads).sort((a, b) => b.idx - a.idx),
        activeDownloads() {
            return this.downloadQueue.filter((dl) => dl.status === 'downloading').length;
        },
        missingTools: (state) => Object.keys(state.mediaTools).filter((bin) => state.mediaTools[bin].avail === false),
        searchIsResultsAvailable: (state) => !!state.search.result.hits.length,
        searchCurrentPage: (state) => state.search.result.page + 1,
        searchCanPrevious: (state) => state.search.result.page > 0,
        searchCanNext: (state) => state.search.result.page < state.search.result.nbPages - 1,
        getFPDomain: (state) => state.fpMeta.domain[state.config.region]
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
        showSearchResults() {
            this.search.showResults = true;
        },
        hideSearchResults() {
            this.search.showResults = false;
        },
        showModUpdatePrompt() {
            window.ui('#modUpdatePrompt');
        },
        showModConfigResetPrompt() {
            window.ui('#modConfigResetPrompt');
        },
        showModConfig() {
            this.modals.modConfig.data = JSON.parse(JSON.stringify(this.config));
            window.ui('#modConfig');
        },
        openAppDownloadPage() {
            window.open(this.update.url, '_blank');
        },
        addDownload(VOD) {
            this.downloads[VOD.qID] = {
                ...VOD,
                idx: this.downloadQueue.length + 1
            };
        },
        setDownloadRestart(VOD) {
            this.downloads[VOD.qID] = {
                ...VOD,
                task: 'prepare',
                status: 'downloading',
                progress: 0,
                size: 'N/A',
                speed: 'N/A',
                eta: 'N/A'
            };
        },
        setDownloadCancelled(qID) {
            this.downloads[qID].status = 'cancelled';
        },
        getFightPassURLByID(id) {
            return `https://${this.getFPDomain}/video/${id}`;
        },
        openVODInFightPass(id) {
            window.open(this.getFightPassURLByID(id), '_blank');
        }
    }
});
