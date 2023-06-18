// Modules
const path = require('path');
const express = require('express');
const http = require('http');
const clr = require('ansi-colors');
const {readConfig, getConfig} = require('./server-modules/config-util');
const {initIO} = require('./server-modules/io-util');

// Global
global.__isContainer = () => process.env.RUN_ENV === 'container';

// Configs
readConfig();
const port = getConfig('port') || 8383;

// Init server
const xApp = express();
const xServer = http.createServer(xApp);

/* Middleware
=============*/
xApp.use(express.static(path.join(__dirname, 'dist')));

/* Websocket
============*/
initIO(xServer);

/* Start server
===============*/
xServer.listen(port, () => {
    console.log(clr.greenBright(`UFC Ripper GUI is live at http://localhost:${port}${__isContainer() ? ' (container)' : ''}\n`));
});
