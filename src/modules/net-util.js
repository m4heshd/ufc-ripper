// Modules
const axios = require('axios');
const {getConfig, writeConfig} = require('./config-util');

module.exports = {
    refreshAuth
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
