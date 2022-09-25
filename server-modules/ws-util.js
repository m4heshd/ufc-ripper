// Modules
const {randomUUID} = require('crypto');

module.exports = {
    sendError,
    sendVODMeta,
    sendVODDownload
};

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
