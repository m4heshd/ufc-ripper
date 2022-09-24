module.exports = {
    sendError,
    sendVODMeta
};

function sendError(error, cb) {
    console.error(`${error}\n`);
    cb({error});
}

function sendVODMeta(res, cb) {
    if (res.data) {
        const {id, title, description, thumbnailUrl} = res.data;
        cb({
            id,
            title,
            desc: description,
            thumb: thumbnailUrl
        });
    } else {
        sendError('No data in the response', cb);
    }
}