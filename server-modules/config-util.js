// Modules
const fs = require('fs-extra');

let config = {};

module.exports = {
    config,
    readConfig,
    writeConfig,
    getConfig
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
