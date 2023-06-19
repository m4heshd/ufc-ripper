// Modules
const fs = require('fs-extra');
const path = require('path');
const {spawn, execSync} = require('child_process');
const {platform} = require('os');
const kill = require('tree-kill');
const clr = require('ansi-colors');
const {getConfig, incFileNumber, decFileNumber} = require('./config-util');
const {sendVODDownload, emitError, emitDownloadProgress, sendDLCancel} = require('./io-util');
const {createUFCRError} = require('./error-util');
const {processYTDLPOutput} = require('./txt-util');

const downloads = {}; // yt-dlp child processes
const failedDownloads = [];
const binPath = path.resolve('.', 'bin');
const bins = {
    atomicParsley: path.join(binPath, {win32: 'AtomicParsley.exe', linux: 'AtomicParsley'}[platform()]),
    ffmpeg: path.join(binPath, {win32: 'ffmpeg.exe', linux: 'ffmpeg'}[platform()]),
    ffprobe: path.join(binPath, {win32: 'ffprobe.exe', linux: 'ffprobe'}[platform()]),
    ytDlp: path.join(binPath, {win32: 'yt-dlp.exe', linux: 'yt-dlp'}[platform()])
};

module.exports = {
    binPath,
    validateBins,
    openDLSession,
    cancelDLSession,
    openDLDir,
    setFileExecutable
};

function validateBins(cb) {
    try {
        cb({
            atomicParsley: fs.existsSync(bins.atomicParsley),
            ffmpeg: fs.existsSync(bins.ffmpeg),
            ffprobe: fs.existsSync(bins.ffprobe),
            ytDlp: fs.existsSync(bins.ytDlp)
        });
    } catch (error) {
        throw createUFCRError(error, 'Unable to validate helper tools');
    }
}

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
    const progressTemplate = JSON.stringify({
        status: '%(progress.status)s',
        progress: '%(progress._percent_str)s',
        size: '%(progress._total_bytes_estimate_str)s',
        speed: '%(progress._speed_str)s',
        eta: '%(progress._eta_str)s',
        vcodec: '%(info.vcodec)s'
    });
    const downloadConfig = {
        '--format': `${vidQuality}[height=${resolution}][fps=${framerate}][ext=${extension}]+${audQuality}`,
        '--merge-output-format': mergeExt,
        '--output': `${path.join(dlPath, `${fullTitle}.%(ext)s`)}`,
        '--progress-template': progressTemplate,
        '--ffmpeg-location': binPath
    };
    if (throttle) downloadConfig['--limit-rate'] = dlRate;

    // Fail action
    const failDL = (error, consoleMsg, userMsg) => {
        if (!failedDownloads.includes(qID)) {
            console.error(clr.redBright.bgBlack.bold(consoleMsg));
            failedDownloads.push(qID);
            decFileNumber();
            emitError(createUFCRError(error, `${userMsg}\nCheck the console for error information`));
            emitDownloadProgress(qID, {status: 'failed'});
        }
    };

    // Begin download process
    console.log(clr.yellowBright.bgBlack.bold.underline(`Downloading "${fullTitle}"`));
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
    const dlArgsAll = [
        ...Object.entries(downloadConfig).flat(),
        ...dlArgs,
        hls
    ];
    const dl = spawn(path.join(bins.ytDlp), dlArgsAll);

    downloads[qID] = dl;

    if (getConfig('verboseLogging')) console.log(`[yt-dlp-args] ${dlArgsAll.join(' ')}\n`);

    dl.on('error', (error) => {
        failDL(
            error,
            `Failed to start the download process - "${fullTitle}"`,
            'Failed to start the download process.'
        );
    });

    dl.on('close', (code) => {
        if (code === 0) {
            console.log(clr.greenBright.bgBlack.bold(`Completed download - "${fullTitle}"`));
            emitDownloadProgress(qID, {status: 'completed'});
        }
    });

    dl.stdout.on('data', (data) => {
        emitDownloadProgress(qID, processYTDLPOutput(data));
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
        sendDLCancel(VOD, cb);
    });
}

function openDLDir(cb) {
    try {
        if (platform() === 'win32') execSync(`start "" "${getConfig('dlPath')}"`);
        if (cb) cb();
    } catch (error) {
        throw createUFCRError(error, 'Unable to open the download directory');
    }
}

function setFileExecutable(file) {
    fs.chmodSync(file, 0o775);
}
