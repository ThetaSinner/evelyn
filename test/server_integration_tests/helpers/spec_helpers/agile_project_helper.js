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
    createProject: createProject,
    addUserContributor: addUserContributor,
    addUserGroupContributor: addUserGroupContributor,
    lookupProjectPreviews: lookupProjectPreviews,
    lookupProject: lookupProject,
};

function createProject(token, project_ref) {
    return httpHelper.post('/agile/project/create', {
        Token: token,
        Name: "name_" + project_ref,
        ShortName: "short_name_" + project_ref,
        Description: "description_" + project_ref
    })
    .then(serverErrorHelper.newResponseHandler());
}

function addUserContributor(token, projectId, userId) {
    return httpHelper.post('/agile/project/contributor/user/add', {
        Token: token,
        ProjectId: projectId,
        UserContributor: {
            UserId: userId
        }
    })
    .then(serverErrorHelper.newResponseHandler());
}

function addUserGroupContributor(token, projectId, userGroupId) {
    return httpHelper.post('/agile/project/contributor/usergroup/add', {
        Token: token,
        ProjectId: projectId,
        UserGroupContributor: {
            UserGroupId: userGroupId
        }
    })
    .then(serverErrorHelper.newResponseHandler());
}

function lookupProjectPreviews(token) {
    return httpHelper.post('/agile/project/lookup/contributingto', {
        Token: token
    })
    .then(serverErrorHelper.newResponseHandler());
}

function lookupProject(token, projectId, expectServerError) {
    return httpHelper.post('/agile/project/lookup', {
        Token: token,
        ProjectId: projectId
    })
    .then(serverErrorHelper.newResponseHandler({expectServerError: expectServerError}));
}
