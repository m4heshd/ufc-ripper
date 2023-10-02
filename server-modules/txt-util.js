// Modules
const {getConfig} = require('./config-util');

module.exports = {
    getVODIDFromURL,
    processYTDLPOutput
};

function getVODIDFromURL(url) {
    const id = new URL(url).pathname.match(/(?<=video\/).*$/);
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
                const outObj = JSON.parse(outString.trim().split('\r')[0]);

                dlStats.progress = Number(outObj.progress.trim().replace('%', '') || 0);
                dlStats.size = (outObj.size || '').trim();
                dlStats.speed = (outObj.speed || '').trim();
                dlStats.eta = (outObj.eta || '').trim();
                dlStats.task = outObj.vcodec === 'none' || outObj.vcodec === null ? 'audio' : 'video';
            } catch (error) {
                if (getConfig('verboseLogging'))
                    console.error(
                        'Could not parse the progress output:\n',
                        outString,
                        `${error.stack}\n`
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
