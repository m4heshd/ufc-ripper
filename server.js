// Modules
const path = require('path');
const express = require('express');
const http = require('http');
const clr = require('ansi-colors');
const {readConfig, getConfig} = require('./server-modules/config-util');
const {getVODMeta, getVODStream} = require('./server-modules/net-util');
const {sendError, sendVODMeta} = require('./server-modules/ws-util');
const {openDLSession} = require('./server-modules/bin-util');

// Init server
const xApp = express();

// Configs
readConfig();
const port = getConfig('port') || 8383;

/* Middleware
=============*/
xApp.use(express.static(path.join(__dirname, 'dist')));

/* Socket
=========*/
const xServer = http.createServer(xApp);
const io = require('socket.io')(xServer, {
    pingTimeout: 90000,
    cors: {
        origin: '*'
    }
});

io.on('connection', (socket) => {
    console.log(`GUI connected (ID - ${socket.id})\n`);

    socket.on('get-config', cb => cb(getConfig()));
    socket.on('verify-url', verifyVOD);
    socket.on('download', downloadVOD);
});

/* Start server
===============*/
xServer.listen(port, () => {
    console.log(clr.greenBright(`UFC Ripper GUI is live at http://localhost:${port}\n`));
});

/* Misc functions
=================*/
async function verifyVOD(url, cb) {
    try {
        sendVODMeta(await getVODMeta(url), cb);
    } catch (error) {
        sendError(error, cb);
    }
}

async function downloadVOD(VOD, cb) {
    try {
        openDLSession({
            ...VOD,
            hls: await getVODStream(VOD.id)
        }, cb);
    } catch (error) {
        sendError(error, cb);
    }
}
