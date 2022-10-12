module.exports = {
    createUFCRError,
    getEnumerableError
};

function createUFCRError(error, userMsg) {
    const isNewError = typeof error === 'string';
    const newError = isNewError ? Error(error) : error;

    return Object.assign(newError, {
        userMsg,
        name: isNewError ? 'UFCRError' : newError.name
    });
}

function getEnumerableError(error) {
    let enumError = {};

    for (const prop of ['message', 'name', 'code', 'userMsg']) {
        if (error[prop]) enumError[prop] = error[prop];
    }

    return enumError;
}
