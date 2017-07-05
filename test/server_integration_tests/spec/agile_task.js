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

var expect = require('chai').expect;
var _ = require('lodash');

var httpHelper = require('../helpers/chai-http-request-helper.js');
var serverErrorHelper = require('../helpers/server-error-helper.js');

var userGroupHelper = require('./user_group.js');
var agileProjectHelper = require('./agile_project.js');

function createTask(token, projectId, taskRef, otherProperties) {
    return httpHelper.chaiHttpPost('/agile/task/create', {
        Token: token,
        ProjectId: projectId,
        Title: "title_" + taskRef,
        Description: "description_" + taskRef,
        OriginalEstimate: _.get(otherProperties, 'originalEstimate', '1h'),
    })
    .then(serverErrorHelper.checkResponseForServerErrors);
}

describe('Agile: Task', function() {
    var token = null;
    var projectId = null;

    before(function () {
        return httpHelper.chaiHttpPostPurgeDatabase()
        .then(function () {
            return httpHelper.createUserAndLogon('user');
        })
        .then(function (_token) {
            token = _token;
        })
        .then(function() {
            return agileProjectHelper.createProject(token, 'task_project');
        })
        .then(function(response) {
            projectId = response.ProjectId;  
        });
    });

    beforeEach(function() {
        return httpHelper.chaiHttpPostPurgeDatabaseArea('agile_task');
    });

    it('Creates a task', function() {
        return createTask(token, projectId, 'starter_ref');
    });
});
