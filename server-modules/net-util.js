// Modules
const axios = require('axios');
const {getConfig, writeConfig} = require('./config-util');
const {getVODIDFromURL} = require('./txt-util');
const {createUFCRError} = require('./error-util');

module.exports = {
    fightPassLogin,
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

async function fightPassLogin(email, pass) {
    const config = {
        method: 'post',
        url: 'https://dce-frontoffice.imggaming.com/api/v2/login',
        headers: {
            ...getHeaders(),
            'Content-Type': 'application/json'
        },
        data: {
            id: email,
            secret: pass
        }
    };

    try {
        const {data} = await axios(config);

        if (!(data?.authorisationToken && data?.refreshToken)) throw createUFCRError('No auth returned');

        return writeConfig({
            user: email,
            authToken: data.authorisationToken,
            refreshToken: data.refreshToken
        }, false);
    } catch (error) {
        if (error.response?.status === 404) throw createUFCRError(error, 'Incorrect email or password');
        throw createUFCRError(error, 'An unknown authentication error occurred');
    }
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

    const {data} = await axios(config);

    if (data?.authorisationToken) {
        writeConfig({
            authToken: data.authorisationToken
        }, false);
    } else {
        throw createUFCRError('No auth returned');
    }
}

async function getVODMeta(url) {
    const id = getVODIDFromURL(url);

    if (!id) throw createUFCRError('Invalid URL');

    const runReq = async () => {
        const {data} = await axios({
            method: 'get',
            url: `https://dce-frontoffice.imggaming.com/api/v2/vod/${id}`,
            headers: getHeaders(getConfig('authToken'))
        });

        if (data) {
            const {id, title, description, thumbnailUrl} = data;

            return {
                id,
                title,
                desc: description,
                thumb: thumbnailUrl,
                vodURL: url
            };
        } else {
            throw createUFCRError('No data in the response');
        }
    };

    try {
        return await runReq();
    } catch (error) {
        if (error.response.status === 401) {
            await refreshAuth();
            return await runReq();
        } else {
            throw createUFCRError(error, 'An unknown error has occurred while trying to retrieve VOD metadata');
        }
    }
}

async function getVODStream(id) {
    let config = {
        method: 'get',
        url: `https://dce-frontoffice.imggaming.com/api/v3/stream/vod/${id}`,
        headers: getHeaders(getConfig('authToken'))
    };

    let {data} = await axios(config);

    if (data?.playerUrlCallback) {
        data = (await axios.get(data.playerUrlCallback)).data;

        if (data?.hls?.length) {
            return data.hls[0].url;
        } else {
            throw createUFCRError('No stream URL in the API response');
        }
    } else {
        throw createUFCRError('No playback URL in the API response');
    }
}
