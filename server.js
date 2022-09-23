// Modules
const path = require('path');
const express = require('express');
const http = require('http');
const {readConfig, getConfig} = require('./src/modules/config-util');

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
    pingTimeout: 90000
});

io.on('connection', (socket) => {
    console.log(`GUI connected (ID - ${socket.id})\n`);

    socket.on('get-config', (cb) => {
        cb(getConfig());
    });
});

/* Start server
===============*/
xServer.listen(port, () => {
    console.log(`UFC Ripper is live at http://localhost:${port}\n`);
});
