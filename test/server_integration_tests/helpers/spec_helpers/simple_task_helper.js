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

var expect = require('chai').expect;

var httpHelper = require('../chai_http_request_helper');
var serverErrorHelper = require('../server_error_helper.js')
var _ = require('lodash');

module.exports = {
    createTasks: createTasks,
    lookupTasks: lookupTasks
};

function createTasks(starter_task, number_to_create) {
    return httpHelper.post(
        '/simpletask/create',
        starter_task
    ).then(function (response) {
        expect(response.Error).to.be.null;

        if (number_to_create <= 1) {
            return Promise.resolve(response);
        }
        else {
            return createTasks(starter_task, number_to_create - 1);
        }
    })
    .then(serverErrorHelper.newResponseHandler());
}

function lookupTasks(token) {
    return httpHelper.post(
        '/simpletask/lookup',
        {
            Token: token,
            Limit: 0,
            ShowCompleted: false
        }
    )
    .then(serverErrorHelper.newResponseHandler());
}
