// Evelyn: Your personal assistant, project manager and calendar
// Copyright (C) 2017 Gregory Jensen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

if (!global.Promise) {
    global.Promise = require('bluebird');
}

var _ = require('lodash');

module.exports = {
    newResponseHandler: newResponseHandler
};

function newResponseHandler(opts) {
    var params = {
        silent: false
    };

    if (_.isObject(opts)) {
        if (_.isBoolean(opts.expectServerError)) {
            params.silent = true;
        }
    }

    return checkResponseForServerErrors.bind(null, params);
}

function checkResponseForServerErrors(params, response) {
    if (_.isNull(response.Error)) {
        return Promise.resolve(response);
    }

    if (!params.silent) {
        console.log('Server message: [', response.Error.ErrorMessage, ']');
    }
    return Promise.reject(response);
}


