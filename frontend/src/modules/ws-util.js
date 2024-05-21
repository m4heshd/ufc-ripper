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
        socket = io(import.meta.env.VITE_WS_URI);

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
        await getAppMeta();
        await getConfig();
        await getDLQ();
        store.hideOverlay();

        await checkAppUpdate();
        if (store.update.updatable) store.showModUpdatePrompt();

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
    async function getAppMeta() {
        try {
            store.appMeta = await emitPromise('get-app-meta');
        } catch (error) {
            store.popError(error);
        }
    }

    async function getConfig() {
        try {
            store.config = await emitPromise('get-config');
        } catch (error) {
            store.popError(error);
        }
    }

    async function getDLQ() {
        try {
            store.downloads = {
                ...(await emitPromise('get-dlq'))
            };
        } catch (error) {
            store.popError(error);
        }
    }

    async function saveConfig(newConfig) {
        store.config = await emitPromise('save-config', newConfig);
    }

    async function login(region, email, pass) {
        store.config = await emitPromise('login', region, email, pass);
    }

    async function checkAppUpdate() {
        const update = await emitPromise('check-app-update');

        Object.assign(store.update, update);

        return update;
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

    function getPlayableVOD(url) {
        return emitPromise('get-playable', url);
    }

    function searchVODs(query, page) {
        return emitPromise('search-vods', query, page);
    }

    function getFormats(url) {
        return emitPromise('get-formats', url);
    }

    function downloadVOD(VOD, isRestart) {
        return emitPromise('download', VOD, isRestart);
    }

    function cancelDownload(VOD) {
        return emitPromise('cancel-download', VOD);
    }

    async function clearDLQ() {
        try {
            store.downloads = {
                ...(await emitPromise('clear-dlq'))
            };
        } catch (error) {
            store.popError(error);
        }
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
        getPlayableVOD,
        getFormats,
        downloadVOD,
        cancelDownload,
        searchVODs,
        clearDLQ,
        openDownloadsDir,
        validateMediaTools,
        getMediaTools
    };
}
