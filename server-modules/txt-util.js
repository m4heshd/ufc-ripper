// Modules
const {getConfig} = require('./config-util');

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

    if (outString.includes('Deleting')) type = 'cleanup';
    if (outString.includes('"status":"downloading"')) type = 'progress';

    switch (type) {
        case 'progress':
            try {
                const outObj = JSON.parse(outString.trim());

                dlStats.progress = Number(outObj.progress.trim().replace('%', '') || 0);
                dlStats.size = (outObj.size || '').trim();
                dlStats.speed = (outObj.speed || '').trim();
                dlStats.eta = (outObj.eta || '').trim();
                dlStats.task = outObj.videoExt === 'none' ? 'audio' : 'video';
            } catch (error) {
                console.error(
                    'Could not parse the progress output:\n',
                    outString,
                    `${getConfig('verboseLogging') ? error.stack : error}\n`
                );
            }
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
