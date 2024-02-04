// Modules
const project = require('./../package.json');
const semverGt = require('semver/functions/gt');
const {getAppUpdateMeta} = require('./net-util');

module.exports = {
    getAppMetadata,
    checkAppUpdates
};

// Returns all the application metadata including version information
function getAppMetadata() {
    return project;
}

// Checks if updates are available for the app and returns new version's data
async function checkAppUpdates() {
    const update = await getAppUpdateMeta();

    if (semverGt(update.version, project.version)) {
        return {
            updatable: true,
            version: update.version,
            url: `${project.homepage}/releases/latest`
        };
    }

    return {
        updatable: false
    };
}