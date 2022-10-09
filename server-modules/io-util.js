// Modules
const {Server} = require('socket.io');
const {randomUUID} = require('crypto');
const {fightPassLogin, getVODMeta, getVODStream} = require('./net-util');
const {writeConfig, getConfig} = require('./config-util');

// Websocket
let io;

module.exports = {
    initIO,
    sendError,
    sendVODMeta,
    sendVODDownload
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
        socket.on('login', login);
        socket.on('verify-url', verifyVOD);
        socket.on('download', downloadVOD);
        socket.on('save-config', saveConfig);
    });
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

async function downloadVOD(VOD, cb) {
    try {
        require('./bin-util').openDLSession({
            ...VOD,
            hls: await getVODStream(VOD.id)
        }, cb);
    } catch (error) {
        sendError(error, cb);
    }
}

function saveConfig(newConfig, cb) {
    try {
        cb(writeConfig(newConfig));
    } catch (error) {
        sendError(error, cb);
    }
}

// Socket callbacks
function sendError(error, cb) {
    console.error(`${error}\n`);
    cb({error});
}

function sendVODMeta(VOD, cb) {
    cb({
        ...VOD,
        qID: randomUUID()
    });
}

function sendVODDownload(VOD, cb) {
    cb(VOD);
}
