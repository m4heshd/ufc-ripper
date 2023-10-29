// Modules
const {homedir} = require('os');
const path = require('path');
const fs = require('fs-extra');

let config = {
    port: 8383,
    verboseLogging: false,
    apiKey: '857a1e5d-e35e-4fdf-805b-a87b6f8364bf',
    region: 'dce.ufc',
    user: '',
    refreshToken: '',
    authToken: '',
    showThumb: true,
    showDesc: true,
    resolution: '720',
    framerate: '30',
    extension: 'mp4',
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
        '--no-mtime',
        '--output-na-placeholder',
        '""',
        '--add-metadata',
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
