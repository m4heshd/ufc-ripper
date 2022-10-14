module.exports = {
    getVODIDFromURL,
    processYTDLPOutput
};

function getVODIDFromURL(url) {
    const id = url.match(/(?<=video\/)(.*)(?=\/)/);
    return id ? id[0] : null;
}

function processYTDLPOutput(output) {
    const outString = output.toString();
    let type = outString.match(/(?<=\[)(.*)(?=])/)?.[0];
    let dlStats = {};

    if (outString.includes('Destination:')) type = 'download:begin';
    if (outString.includes('Deleting')) type = 'cleanup';

    switch (type) {
        case 'download:begin':
            dlStats.task = outString.includes('.faudio-') ? 'audio' : 'video';
            break;
        case 'download':
            dlStats.progress = Number(outString.match(/\d+(?:\.\d+)?(?=%)/)?.[0] || 0);
            dlStats.size = outString.match(/(?<=~)(.*)(?= at)/)?.[0];
            dlStats.speed = outString.match(/\d+(?:\.\d+)?([a-zA-Z]+\/s)/)?.[0];
            dlStats.eta = outString.match(/(?<=ETA )(.*)(?= \()/)?.[0];
            break;
        case 'Merger':
            dlStats.task = 'merge';
            break;
        case 'Metadata':
            dlStats.task = 'meta';
            break;
        case 'cleanup':
            dlStats.task = 'cleanup';
            break;
        default:
            dlStats.task = 'prepare';
    }

    return dlStats;
}
