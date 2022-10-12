// Modules
const path = require('path');
const {spawn} = require('child_process');
const clr = require('ansi-colors');
const {getConfig, writeConfig} = require('./config-util');
const {sendVODDownload, emitError, emitDownloadProgress} = require('./io-util');
const {createUFCRError} = require('./error-util');

module.exports = {
    openDLSession
};

function openDLSession(VOD, cb) {
    const {qID, title, hls, vodURL} = VOD;
    const {
        vidQuality,
        audQuality,
        resolution,
        framerate,
        extension,
        dl_path,
        numberFiles,
        curNumber,
        dl_args
    } = getConfig();
    const fullTitle = `${numberFiles ? `${curNumber}. ` : ''}${title}`;
    const failDL = (error, consoleMsg, userMsg) => {
        console.error(clr.redBright.bgBlack.bold(consoleMsg));
        writeConfig({curNumber: curNumber - 1});
        emitError(createUFCRError(error, userMsg));
        emitDownloadProgress(qID, {status: 'failed'});
    };

    console.log(clr.yellowBright.bgBlack.bold.underline(`Downloading "${title}"`));
    console.log(clr.dim(`${vodURL}\n`));

    writeConfig({curNumber: curNumber + 1});
    sendVODDownload({
        ...VOD,
        title: fullTitle,
        status: 'downloading'
    }, cb);

    const dl = spawn('.\\bin\\yt-dlp.exe', [
        `-f "${vidQuality}[height=${resolution}][fps=${framerate}][ext=${extension}]+${audQuality}"`,
        `-o "${path.join(dl_path, `${fullTitle}.%(ext)s`)}"`,
        ...dl_args,
        hls
    ], {
        shell: true
    });

    dl.on('error', (error) => {
        failDL(
            error,
            `Failed to start the download process - "${title}"`,
            'Failed to start the download process.\nCheck the console for error information'
        );
    });

    dl.on('close', (code) => {
        if (code === 0) {
            console.log(clr.greenBright.bgBlack.bold(`Completed download - "${title}"`));
            emitDownloadProgress(qID, {status: 'completed'});
        }
    });

    dl.stderr.on('data', (data) => {
        failDL(
            data.toString(),
            `Download failed - "${title}"`,
            'A download has unexpectedly ended with an error.\nCheck the console for error information'
        );
    });
}
