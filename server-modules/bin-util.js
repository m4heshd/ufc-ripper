// Modules
const path = require('path');
const {spawn} = require('child_process');
const clr = require('ansi-colors');
const {getConfig, writeConfig} = require('./config-util');
const {sendVODDownload, emitError} = require('./io-util');
const {createUFCRError} = require('./error-util');

module.exports = {
    openDLSession
};

function openDLSession(VOD, cb) {
    const {title, hls, vodURL} = VOD;
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

    console.log(clr.yellowBright.bgBlack.bold.underline(`Downloading "${title}"`));
    console.log(clr.dim(`${vodURL}\n`));

    writeConfig({curNumber: curNumber + 1});
    sendVODDownload({
        ...VOD,
        title: fullTitle
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
        writeConfig({curNumber: curNumber - 1});
        console.log(clr.redBright.bgBlack.bold(`Failed to start the download process - "${title}"`));
        emitError(createUFCRError(error, 'Failed to start the download process.\nCheck the console for error information'));
    });

    dl.on('close', (code) => {
        if (code === 0) return console.log(clr.greenBright.bgBlack.bold(`Completed download - "${title}"`));
    });

    dl.stderr.on('data', (data) => {
        writeConfig({curNumber: curNumber - 1});
        console.log(clr.redBright.bgBlack.bold(`Download failed - "${title}"`));
        emitError(createUFCRError(data.toString(), 'A download has unexpectedly ended with an error.\nCheck the console for error information'));
    });
}
