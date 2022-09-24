// Modules
const axios = require('axios');
const {getConfig, writeConfig} = require('./config-util');
const {getVODIDFromURL} = require('./txt-util');

module.exports = {
    refreshAuth,
    getVODMeta,
    getVODStream
};

function getHeaders(auth) {
    const headers = {
        'Realm': 'dce.ufc',
        'x-app-var': '6.0.0',
        'x-api-key': getConfig('apiKey'),
        'app': 'dice'
    };

    return auth ? {...headers, 'Authorization': `Bearer ${auth}`} : headers;
}

async function refreshAuth() {
    const config = {
        method: 'post',
        url: 'https://dce-frontoffice.imggaming.com/api/v2/token/refresh',
        headers: {
            ...getHeaders(getConfig('authToken')),
            'Content-Type': 'application/json'
        },
        data: {
            refreshToken: getConfig('refreshToken')
        }
    };

    const res = await axios(config);

    if (res.data.authorisationToken) {
        writeConfig({
            authToken: res.data.authorisationToken
        });
    } else {
        throw 'No auth returned';
    }
}

async function getVODMeta(url) {
    const id = getVODIDFromURL(url);

    if (!id) throw 'Invalid URL';

    const runReq = async () => {
        const res = await axios({
            method: 'get',
            url: `https://dce-frontoffice.imggaming.com/api/v2/vod/${id}`,
            headers: getHeaders(getConfig('authToken'))
        });

        if (res.data) {
            const {id, title, description, thumbnailUrl} = res.data;

            return {
                id,
                title,
                desc: description,
                thumb: thumbnailUrl,
                vodURL: url
            };
        } else {
            throw('No data in the response');
        }
    };

    try {
        return await runReq();
    } catch (error) {
        if (error.response.status === 401) {
            await refreshAuth();
            return await runReq();
        } else {
            throw error;
        }
    }
}

async function getVODStream(id) {
    let config = {
        method: 'get',
        url: `https://dce-frontoffice.imggaming.com/api/v3/stream/vod/${id}`,
        headers: getHeaders(getConfig('authToken'))
    };

    let res = await axios(config);

    if (res.data?.playerUrlCallback) {
        res = await axios.get(res.data.playerUrlCallback);

        if (res.data?.hls?.length) {
            return res.data.hls[0].url;
        } else {
            throw 'No stream URL';
        }
    } else {
        throw 'No playback URL';
    }
}
