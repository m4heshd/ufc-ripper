module.exports = {
    getVODIDFromURL
};

function getVODIDFromURL(url) {
    const id = url.match(/(?<=video\/)(.*)(?=\/)/);
    return id ? id[0] : null;
}
