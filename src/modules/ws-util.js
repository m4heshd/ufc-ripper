// Store
import {useAppStore} from '@/store';
// Modules
import {io} from 'socket.io-client';

// Websocket
let socket;

export function useWSUtil() {
    // Store
    const store = useAppStore();

    function emitPromise(event, ...args) {
        return new Promise((resolve, reject) => {
            if (socket) {
                socket.emit(event, ...args, (res) => res?.error ? reject(res.error) : resolve(res));
            } else {
                reject(new Error('Socket not initiated'));
            }
        });
    }

    function initSocket() {
        socket = io(process.env.VUE_APP_WS_URI);

        socket.on('connect', onSocketConnection);
        socket.on('disconnect', store.showOverlay);
        socket.on('config-update', onConfigUpdate);
        socket.on('server-error', onServerError);
        socket.on('dl-progress', onDownloadProgress);
    }

    // Socket event handles
    async function onSocketConnection() {
        await getConfig();
        store.hideOverlay();
        console.log('Connected to backend');

        store.modals.modBinDL.data.bins = await validateHelperTools();

        if (store.missingBins.length) window.ui('#modBinDL');
    }

    function onConfigUpdate(newConfig) {
        store.config = newConfig;
    }

    function onServerError(error) {
        store.popError(error);
    }

    function onDownloadProgress(qID, updates) {
        store.downloads[qID] = {
            ...store.downloads[qID],
            ...updates
        };
    }

    // Socket emits
    async function getConfig() {
        try {
            store.config = await emitPromise('get-config');
        } catch (error) {
            store.popError(error);
        }
    }

    async function saveConfig(newConfig) {
        store.config = await emitPromise('save-config', newConfig);
    }

    async function login(email, pass) {
        store.config = await emitPromise('login', email, pass);
    }

    function validateHelperTools() {
        return emitPromise('validate-bins');
    }

    function verifyURL(url) {
        return emitPromise('verify-url', url);
    }

    function downloadVOD(VOD) {
        return emitPromise('download', VOD);
    }

    function cancelDownload(VOD) {
        return emitPromise('cancel-download', VOD);
    }

    function openDownloadsDir() {
        return emitPromise('open-dl-dir');
    }

    return {
        initSocket,
        getConfig,
        saveConfig,
        login,
        verifyURL,
        downloadVOD,
        cancelDownload,
        openDownloadsDir
    };
}
