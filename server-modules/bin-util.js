// Modules
const path = require('path');
const {spawn} = require('child_process');
const kill = require('tree-kill');
const clr = require('ansi-colors');
const {getConfig, incFileNumber, decFileNumber} = require('./config-util');
const {sendVODDownload, emitError, emitDownloadProgress} = require('./io-util');
const {createUFCRError} = require('./error-util');
const {processYTDLPOutput} = require('./txt-util');

// yt-dlp child processes
const downloads = {};

module.exports = {
    openDLSession,
    cancelDLSession
};

function openDLSession(VOD, cb) {
    const {qID, title, hls, vodURL} = VOD;
    const {
        vidQuality,
        audQuality,
        resolution,
        framerate,
        extension,
        mergeExt,
        dlPath,
        numberFiles,
        curNumber,
        throttle,
        dlRate,
        dlArgs
    } = getConfig();

    // Download configuration
    const fullTitle = `${numberFiles ? `${curNumber}. ` : ''}${title}`;
    const downloadConfig = {
        '--format': `"${vidQuality}[height=${resolution}][fps=${framerate}][ext=${extension}]+${audQuality}"`,
        '--merge-output-format': mergeExt,
        '--output': `"${path.join(dlPath, `${fullTitle}.%(ext)s`)}"`
    };
    if (throttle) downloadConfig['--limit-rate'] = dlRate;

    // Fail action
    const failDL = (error, consoleMsg, userMsg) => {
        console.error(clr.redBright.bgBlack.bold(consoleMsg));
        decFileNumber();
        emitError(createUFCRError(error, `${userMsg}\nCheck the console for error information`));
        emitDownloadProgress(qID, {status: 'failed'});
    };

    // Begin download process
    console.log(clr.yellowBright.bgBlack.bold.underline(`Downloading "${title}"`));
    console.log(clr.dim(`${vodURL}\n`));

    incFileNumber();
    sendVODDownload({
        ...VOD,
        title: fullTitle,
        task: 'prepare',
        status: 'downloading',
        progress: 0,
        size: 'N/A',
        speed: 'N/A',
        eta: 'N/A'
    }, cb);

    // Launch and handle yt-dlp process
    const dl = spawn('.\\bin\\yt-dlp.exe', [
        ...Object.entries(downloadConfig).flat(),
        ...dlArgs,
        hls
    ], {
        windowsVerbatimArguments: true
    });

    downloads[qID] = dl;

    dl.on('error', (error) => {
        failDL(
            error,
            `Failed to start the download process - "${title}"`,
            'Failed to start the download process.'
        );
    });

    dl.on('close', (code) => {
        if (code === 0) {
            console.log(clr.greenBright.bgBlack.bold(`Completed download - "${title}"`));
            emitDownloadProgress(qID, {status: 'completed'});
        }
    });

    dl.stdout.on('data', (data) => {
        const dlStats = processYTDLPOutput(data);
        if (dlStats) emitDownloadProgress(qID, dlStats);
    });

    dl.stderr.on('data', (data) => {
        failDL(
            data.toString(),
            `Download failed - "${title}"`,
            'A download has unexpectedly ended with an error.'
        );
    });
}

function cancelDLSession(VOD, cb) {
    const {qID, title} = VOD;

    if (!downloads[qID]) throw createUFCRError('Download process is not present');

    kill(downloads[qID].pid, 'SIGKILL', (error) => {
        if (error) throw createUFCRError(error, 'Unable to cancel the download');
        console.error(clr.redBright.bgBlack.bold(`Download cancelled by user - "${title}"`));
        if (cb) cb();
    });
}
