// Modules
const fs = require('fs-extra');

let config = {};

module.exports = {
    config,
    readConfig,
    writeConfig,
    getConfig,
    incFileNumber,
    decFileNumber
};

function readConfig(key) {
    config = fs.readJSONSync('config.json');
    return getConfig(key);
}

function writeConfig(newConfig, emitUpdate = true) {
    config = {
        ...config,
        ...newConfig
    };

    fs.writeJSONSync('config.json', config, {
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

function decFileNumber(step = 1) {
    return config.numberFiles ? writeConfig({curNumber: config.curNumber - step}) : config;
}
