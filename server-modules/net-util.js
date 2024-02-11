// Modules
const path = require('node:path');
const {platform} = require('node:os');
const fs = require('fs-extra');
const axios = require('axios');
const algoliasearch = require('algoliasearch');
const {getConfig, writeConfig} = require('./config-util');
const {getVODIDFromURL} = require('./txt-util');
const {createUFCRError} = require('./error-util');

module.exports = {
    getAppUpdateMeta,
    fightPassLogin,
    refreshAuth,
    getVODMeta,
    getVODStream,
    getVODSearchResults,
    downloadFile,
    getMediaToolsInfo,
    downloadMediaTools
};

function getHeaders(auth) {
    const headers = {
        'Realm': getConfig('region'),
        'x-api-key': getConfig('apiKey'),
        'x-app-var': '6.0.1.f8add0e',
        'app': 'dice'
    };

    return auth ? {...headers, 'Authorization': `Bearer ${auth}`} : headers;
}

function getProxyConfig() {
    return getConfig('useProxy') ? getConfig('proxyConfig') : undefined;
}

async function getAppUpdateMeta() {
    const config = {
        method: 'get',
        url: `${require('./app-util').getAppMetadata().homepage}/raw/master/package.json`,
    };

    try {
        const {data} = await axios(config);

        if (!data) throw createUFCRError('Update metadata missing');

        return data;
    } catch (error) {
        throw createUFCRError(error, 'Failed to check for app updates. Check the console for error information');
    }
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
        },
        proxy: getProxyConfig()
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
        },
        proxy: getProxyConfig()
    };

    try {
        const {data} = await axios(config);

        if (data?.authorisationToken) {
            writeConfig({
                authToken: data.authorisationToken
            }, false);
        } else {
            throw createUFCRError('No auth returned');
        }
    } catch (error) {
        if (error.response?.status === 404) {
            throw createUFCRError(error, 'Your Fight Pass login has expired. Please logout and login again');
        } else {
            throw createUFCRError(error, 'An unknown error has occurred while trying to refresh the authorization token');
        }
    }
}

async function getVODMeta(url) {
    const id = getVODIDFromURL(url);

    if (!id) throw createUFCRError('Invalid URL');

    const runReq = async () => {
        const {data} = await axios({
            method: 'get',
            url: `https://dce-frontoffice.imggaming.com/api/v2/vod/${id}`,
            headers: getHeaders(getConfig('authToken')),
            proxy: getProxyConfig()
        });

        if (data) {
            const {id, title, description, thumbnailUrl, accessLevel} = data;

            return {
                id,
                title: title.replace(':', ' -'),
                desc: description,
                thumb: thumbnailUrl,
                access: accessLevel !== 'DENIED',
                vodURL: url
            };
        } else {
            throw createUFCRError('No data in the response');
        }
    };

    try {
        return await runReq();
    } catch (error) {
        if (error.response?.status === 401) {
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
        headers: getHeaders(getConfig('authToken')),
        proxy: getProxyConfig()
    };

    let {data} = await axios(config);

    if (data?.playerUrlCallback) {
        data = (await axios({
            method: 'get',
            url: data.playerUrlCallback,
            proxy: getProxyConfig()
        })).data;

        if (data?.hls?.length) {
            return data.hls[0].url;
        } else {
            throw createUFCRError('No stream URL in the API response');
        }
    } else {
        throw createUFCRError('No playback URL in the API response');
    }
}

async function getVODSearchResults(query, page = 0) {
    const client = algoliasearch('H99XLDR8MJ', getConfig('searchAPIKey'));
    const index = client.initIndex('prod-dce.ufc-livestreaming-events');

    try {
        const data = await index.search(query, {
            facetFilters: 'type:VOD_VIDEO',
            hitsPerPage: 12,
            page,
            advancedSyntax: true,
            attributesToRetrieve: [
                'id',
                'description',
                'thumbnailUrl',
                'duration'
            ],
            restrictSearchableAttributes: getConfig('searchTitleOnly') ? ['name'] : undefined
        });

        if (data) {
            return data;
        } else {
            throw createUFCRError('No results were returned');
        }
    } catch (error) {
        throw createUFCRError(error, 'An error has occurred while trying to search. Check the console for error information');
    }
}

function downloadFile(url, savePath, onProgress) {
    return new Promise((resolve, reject) => {
        const config = {
            method: 'get',
            responseType: 'stream',
            url
        };

        axios(config)
            .then((res) => {
                const {data, headers} = res;
                const size = headers['content-length'];
                const dest = fs.createWriteStream(savePath);
                let downloaded = 0;
                let progress = 0;

                dest.on('finish', resolve);

                data
                    .on('error', reject)
                    .on('data', (chunk) => {
                        const newProgress = Math.floor(((downloaded += chunk.length) / size) * 100);
                        if (newProgress !== progress) {
                            progress = newProgress;
                            onProgress(progress);
                        }
                    })
                    .pipe(dest);
            })
            .catch(reject);
    });
}

async function getMediaToolsInfo() {
    const config = {
        method: 'get',
        url: 'https://raw.githubusercontent.com/m4heshd/media-tools/master/versions.json'
    };

    const {data} = await axios(config);
    const bins = data?.[platform()];

    if (bins) return bins;
    throw createUFCRError('No media tools info in the response or platform not supported');
}

async function downloadMediaTools(tools) {
    const toolsInfo = await getMediaToolsInfo();
    const {binPath, setFileExecutable} = require('./bin-util');
    const {emitMediaToolDLProgress} = require('./io-util');
    const onProgress = (tool, progress) => {
        emitMediaToolDLProgress(tool, {progress});
    };

    fs.ensureDirSync(binPath);

    for (const tool of tools) {
        if (getConfig('verboseLogging')) console.log(`Downloading ${tool}..`);

        const toolPath = path.join(binPath, toolsInfo[tool]?.filename);

        try {
            await downloadFile(
                toolsInfo[tool]?.download,
                toolPath,
                (progress) => onProgress(tool, progress)
            );
            setFileExecutable(toolPath);
        } catch (error) {
            throw createUFCRError(error, `Failed to download media tool: ${tool}`);
        }
    }
}
