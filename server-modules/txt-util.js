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
    const type = outString.match(/(?<=\[)(.*)(?=])/);

    if (type?.[0] !== 'download' || outString.includes('Destination:')) return null;

    return {
        progress: Number(outString.match(/\d+(?:\.\d+)?(?=%)/)?.[0] || 0),
        size: outString.match(/(?<=~)(.*)(?= at)/)?.[0],
        speed: outString.match(/\d+(?:\.\d+)?([a-zA-Z]+\/s)/)?.[0],
        eta: outString.match(/(?<=ETA )(.*)(?= \()/)?.[0]
    };
}
