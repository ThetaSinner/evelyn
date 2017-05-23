if (!global.Promise) {
    global.Promise = require('bluebird');
}

var _ = require('lodash');

function wrapChaiHttpError(err) {
    var message = "ChaiHttpErrorWrapper: ";
    if (!_.isUndefined(err.message)) {
        message += err.message + "\r\n";
    }

    if (_.isObject(err.actual)) {
        message += JSON.stringify(err.actual) + "\r\n";
    }

    return new Error(message);
}

module.exports = {
    wrapChaiHttpError: wrapChaiHttpError,
};
