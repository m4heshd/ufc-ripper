// Store
import {useAppStore} from '@/store';
import {useModBinDLStore} from '@/store/modBinDL';
// Modules
import {io} from 'socket.io-client';

// Websocket
let socket;

export function useWSUtil() {
    // Store
    const store = useAppStore();
    const modBinDL = useModBinDLStore();

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
        socket.on('media-tool-dl-progress', onMediaToolDLProgress);
    }

    // Socket event handles
    async function onSocketConnection() {
        console.log('Connected to backend');
        await getConfig();
        store.hideOverlay();
        await validateMediaTools();
        if (store.missingTools.length) modBinDL.showModBinDL();
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

    function onMediaToolDLProgress(tool, updates) {
        modBinDL.downloads[tool] = updates;
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

    async function validateMediaTools() {
        const toolsAvail = await emitPromise('validate-media-tools');

        for (const tool in toolsAvail) {
            store.mediaTools[tool].avail = toolsAvail[tool];
        }

        return toolsAvail;
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

    function getMediaTools(missingTools) {
        if (!Object.keys(modBinDL.downloads).length) {
            modBinDL.downloads = Object.fromEntries(missingTools.map(bin => [bin, {}]));
            return emitPromise('get-media-tools', missingTools);
        }
        return Promise.resolve();
    }

    return {
        initSocket,
        getConfig,
        saveConfig,
        login,
        verifyURL,
        downloadVOD,
        cancelDownload,
        openDownloadsDir,
        validateMediaTools,
        getMediaTools
    };
}
