// Modules
const {Server} = require('socket.io');
const {randomUUID} = require('crypto');
const {fightPassLogin, getVODMeta, getVODStream, downloadMediaTools} = require('./net-util');
const {writeConfig, getConfig} = require('./config-util');
const {getEnumerableError, createUFCRError} = require('./error-util');

// Websocket
let io;

// Downloads
const DLQ = {};

module.exports = {
    initIO,
    sendError,
    sendVODMeta,
    sendVODDownload,
    sendDLCancel,
    emitConfigUpdate,
    emitError,
    emitDownloadProgress,
    emitMediaToolDLProgress
};

function initIO(httpServer) {
    io = new Server(httpServer, {
        pingTimeout: 90000,
        cors: {
            origin: '*'
        }
    });

    io.on('connection', (socket) => {
        console.log(`GUI connected (ID - ${socket.id})\n`);

        socket.on('get-config', cb => cb(getConfig()));
        socket.on('get-dlq', cb => cb(DLQ));
        socket.on('login', login);
        socket.on('verify-url', verifyVOD);
        socket.on('download', downloadVOD);
        socket.on('cancel-download', cancelDownload);
        socket.on('clear-dlq', clearDLQ);
        socket.on('save-config', saveConfig);
        socket.on('open-dl-dir', openDownloadsDir);
        socket.on('validate-media-tools', validateMediaTools);
        socket.on('get-media-tools', getMediaTools);
    });
}

function checkIO() {
    if (!io) throw createUFCRError('WebSocket instance not initiated');
}

// Socket handles
async function login(email, pass, cb) {
    try {
        cb(await fightPassLogin(email, pass));
    } catch (error) {
        sendError(error, cb);
    }
}

async function verifyVOD(url, cb) {
    try {
        sendVODMeta(await getVODMeta(url), cb);
    } catch (error) {
        sendError(error, cb);
    }
}

async function downloadVOD(VOD, isRestart, cb) {
    try {
        require('./bin-util').openDLSession(
            {
                ...VOD,
                hls: await getVODStream(VOD.id)
            },
            isRestart,
            cb
        );
    } catch (error) {
        sendError(error, cb);
    }
}

async function cancelDownload(VOD, cb) {
    try {
        require('./bin-util').cancelDLSession(VOD, cb);
    } catch (error) {
        sendError(error, cb);
    }
}

async function clearDLQ(cb) {
    try {
        for (const dl in DLQ) {
            if (DLQ[dl].status !== 'downloading') delete DLQ[dl];
        }
        cb(DLQ);
    } catch (error) {
        sendError(error, cb);
    }
}

function saveConfig(newConfig, cb) {
    try {
        cb(writeConfig(newConfig, false));
    } catch (error) {
        sendError(error, cb);
    }
}

function openDownloadsDir(cb) {
    try {
        require('./bin-util').openDLDir(cb);
    } catch (error) {
        sendError(error, cb);
    }
}

function validateMediaTools(cb) {
    try {
        require('./bin-util').validateBins(cb);
    } catch (error) {
        sendError(error, cb);
    }
}

async function getMediaTools(missingTools, cb) {
    try {
        await downloadMediaTools(missingTools);
        cb();
    } catch (error) {
        sendError(error, cb);
    }
}

// Socket callbacks
function sendError(error, cb) {
    console.error(`${getConfig('verboseLogging') ? error.stack : error}\n`);
    cb({error: getEnumerableError(error)});
}

function sendVODMeta(VOD, cb) {
    cb({
        ...VOD,
        qID: randomUUID()
    });
}

function sendVODDownload(VOD, isRestart, cb) {
    DLQ[VOD.qID] = {
        ...VOD,
        idx: isRestart ? VOD.idx : Object.values(DLQ).length + 1
    };

    cb(VOD);
}

function sendDLCancel(VOD, cb) {
    DLQ[VOD.qID].status = 'cancelled';
    if (cb) cb();
}

// IO emits
function emitConfigUpdate() {
    checkIO();
    io.emit('config-update', getConfig());
}

function emitError(error) {
    console.error(`${getConfig('verboseLogging') ? error.stack : error}\n`);
    checkIO();
    io.emit('server-error', getEnumerableError(error));
}

function emitDownloadProgress(qID, updates) {
    DLQ[qID] = {
        ...DLQ[qID],
        ...updates
    };

    checkIO();
    io.emit('dl-progress', qID, updates);
}

function emitMediaToolDLProgress(tool, updates) {
    checkIO();
    io.emit('media-tool-dl-progress', tool, updates);
}
