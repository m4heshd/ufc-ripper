// Modules
const path = require('path');
const express = require('express');
const http = require('http');
const {readConfig, getConfig} = require('./src/modules/config-util');
const {getVODMeta} = require('./src/modules/net-util');
const {sendError, sendVODMeta} = require('./src/modules/ws-util');
const {getVODIDFromURL} = require('./src/modules/txt-util');

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
});

/* Start server
===============*/
xServer.listen(port, () => {
    console.log(`UFC Ripper GUI is live at http://localhost:${port}\n`);
});

/* Misc functions
=================*/
async function verifyVOD(url, cb) {
    try {
        const meta = await getVODMeta(getVODIDFromURL(url));

        sendVODMeta(meta, cb);
    } catch (error) {
        sendError(error, cb);
    }
}
