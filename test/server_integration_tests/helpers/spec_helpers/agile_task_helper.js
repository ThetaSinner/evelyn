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

var httpHelper = require('../chai_http_request_helper');
var serverErrorHelper = require('../server_error_helper.js')
var _ = require('lodash');

module.exports = {
    createTask: createTask,
    lookupTask: lookupTask,
    lookupBacklog: lookupBacklog,
    updateTask: updateTask
};

function createTask(token, projectId, taskRef, otherProperties) {
    return httpHelper.post('/agile/task/create', {
        Token: token,
        ProjectId: projectId,
        Title: "title_" + taskRef,
        Description: "description_" + taskRef,
        OriginalEstimate: _.get(otherProperties, 'originalEstimate', '1h'),
    })
    .then(serverErrorHelper.newResponseHandler());
}

function lookupTask(token, projectId, taskId) {
    return httpHelper.post('/agile/task/lookup', {
        Token: token,
        ProjectId: projectId,
        TaskId: taskId
    })
    .then(serverErrorHelper.newResponseHandler());
}

function lookupBacklog(token, projectId) {
    return httpHelper.post('/agile/task/lookup/backlog', {
        Token: token,
        ProjectId: projectId
    })
    .then(serverErrorHelper.newResponseHandler());
}

function updateTask(token, projectId, taskId, updateProperties) {
    var payload = {
        Token: token,
        ProjectId: projectId,
        TaskId: taskId
    };

    if (_.has(updateProperties, 'title')) {
        payload.Title = updateProperties.title;
    }
    if (_.has(updateProperties, 'description')) {
        payload.Description = updateProperties.description;
    }
    if (_.has(updateProperties, 'originalEstimate')) {
        payload.OriginalEstimate = updateProperties.originalEstimate;
    }
    if (_.has(updateProperties, 'assignToUserId')) {
        payload.AssignToUserId = updateProperties.assignToUserId;
    }

    return httpHelper.post('/agile/task/update', payload)
    .then(serverErrorHelper.newResponseHandler());
}
