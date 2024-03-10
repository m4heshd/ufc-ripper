// Modules
const {homedir} = require('node:os');
const path = require('node:path');
const fs = require('fs-extra');

let config = {
    openInBrowser: true,
    port: 8383,
    verboseLogging: false,
    apiKey: '857a1e5d-e35e-4fdf-805b-a87b6f8364bf',
    searchAPIKey: 'e55ccb3db0399eabe2bfc37a0314c346',
    region: 'dce.ufc',
    user: '',
    refreshToken: '',
    authToken: '',
    searchTitleOnly: false,
    showThumb: true,
    showDuration: true,
    showDesc: true,
    resolution: '720',
    mergeExt: 'mp4',
    vidQuality: 'worstvideo',
    audQuality: 'bestaudio',
    dlPath: '',
    numberFiles: true,
    curNumber: 1,
    multiFrag: true,
    concurFrags: 64,
    throttle: false,
    dlRate: '100K',
    cusFormat: false,
    formatID: '',
    metadata: false,
    useProxy: false,
    proxyConfig: {
        protocol: 'http',
        host: '0.0.0.0',
        port: 1111,
        auth: {
            username: '',
            password: ''
        }
    },
    dlArgs: [
        '--no-warnings',
        '--no-mtime',
        '--output-na-placeholder',
        '""',
        '--no-cache-dir',
        '--ignore-config',
        '--no-check-certificate'
    ]
};

module.exports = {
    config,
    readConfig,
    writeConfig,
    getConfig,
    incFileNumber
};

function readConfig(key) {
    try {
        config = {
            ...config,
            ...fs.readJSONSync(path.join('config', 'config.json'))
        };
    } catch (error) {
        if (error.code !== 'ENOENT') throw error;
        writeConfig({}, false);
    }

    if (__isContainer()) {
        config = writeConfig({
            dlPath: '/downloads'
        }, false);
    } else if (config.dlPath === '') {
        config = writeConfig({
            dlPath: path.join(homedir(), 'Downloads')
        }, false);
    }

    return getConfig(key);
}

function writeConfig(newConfig = {}, emitUpdate = true) {
    config = {
        ...config,
        ...newConfig
    };

    fs.writeJSONSync(path.join('config', 'config.json'), config, {
        spaces: 2
    });
    if (emitUpdate) require('./io-util').emitConfigUpdate();

    return config;
}

function getConfig(key) {
    return key ? config[key] : config;
}

function incFileNumber(step = 1) {
    return config.numberFiles ? writeConfig({curNumber: config.curNumber + step}) : config;
}
