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

var httpHelper = require('../helpers/chai_http_request_helper.js');
var commonRequestsHelper = require('../helpers/common_requests_helper.js');
var serverErrorHelper = require('../helpers/server_error_helper.js');

var userGroupHelper = require('../helpers/spec_helpers/user_group_helper.js');
var agileProjectHelper = require('../helpers/spec_helpers/agile_project_helper.js');
var agileTaskHelper = require('../helpers/spec_helpers/agile_task_helper.js');

describe('Agile: Task', function() {
    var token = null;
    var projectId = null;

    before(function () {
        return commonRequestsHelper.chaiHttpPostPurgeDatabase()
        .then(function () {
            return commonRequestsHelper.createUserAndLogon('user');
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
        return commonRequestsHelper.chaiHttpPostPurgeDatabaseArea('agile_task');
    });

    it('Creates a task', function() {
        return agileTaskHelper.createTask(token, projectId, 'starter_ref');
    });

    describe('Lookup task', function() {
        it('Looks up a task', function() {
            var taskId = null;

            return agileTaskHelper.createTask(token, projectId, 'starter_ref')
            .then(function (response) {
                expect(response.TaskId).to.be.ok;
                taskId = response.TaskId;
                return agileTaskHelper.lookupTask(token, projectId, response.TaskId);
            })
            .then(function (response) {
                let task = response.Task;
                expect(task.TaskId).to.equal(taskId);
                expect(task.ProjectId).to.equal(projectId);
                expect(task.Title).to.equal('title_starter_ref');
                expect(task.Description).to.equal('description_starter_ref');
                expect(task.OriginalEstimate).to.equal('1h');
                expect(task.ModifiedByUser).to.be.ok;
                expect(task.ModifiedByUser.UserName).to.equal('user');
                expect(task.ModifiedByUser.UserId).to.be.a.string;
                expect(task.DateModified).to.be.ok; // TODO assert date?
                expect(task.Assignment).to.be.null;
            });
        });
    });
});
