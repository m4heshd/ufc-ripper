// Modules
const path = require('path');
const {exec} = require('child_process');
const clr = require('ansi-colors');
const {getConfig, writeConfig} = require('./config-util');
const {sendError, sendVODDownload} = require('./ws-util');

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
    const yt_dlp_cmd = 'start .\\bin\\yt-dlp.exe ' +
        `-f "${vidQuality}[height=${resolution}][fps=${framerate}][ext=${extension}]+${audQuality}" ` +
        `-o "${path.join(dl_path, `${fullTitle}.%(ext)s`)}" ` +
        `${dl_args.join(' ')} ` +
        `${hls}`;

    console.log(clr.yellowBright.bgBlack.bold.underline(`Downloading "${title}"`));
    console.log(clr.dim(`${vodURL}\n`));

    writeConfig({curNumber: curNumber + 1});
    sendVODDownload({
        ...VOD,
        title: fullTitle
    }, cb);

    exec(yt_dlp_cmd, (err, stdout, stderr) => {
        if (err) {
            writeConfig({curNumber: curNumber - 1});
            return cb ? sendError(err, cb) : null;
        }
        console.log(clr.greenBright.bgBlack.bold(`Completed download - "${title}"`));
        console.log(stdout);
    });
}
