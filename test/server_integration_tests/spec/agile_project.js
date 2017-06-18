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
var userGroupHelper = require('./user_group.js');

function createProject(token, project_ref) {
    return httpHelper.chaiHttpPost('/agile/project/create', {
        Token: token,
        Name: "name_" + project_ref,
        ShortName: "short_name_" + project_ref,
        Description: "description_" + project_ref
    })
    .then(function(response) {
        expect(response.Error).to.be.null;
        
        return Promise.resolve(response);
    });
}

function addUserContributor(token, projectId, userId) {
    return httpHelper.chaiHttpPost('/agile/project/contributor/user/add', {
        Token: token,
        ProjectId: projectId,
        UserContributor: {
            UserId: userId
        }
    })
    .then(function(response) {
        expect(response.Error).to.be.null;
        
        return Promise.resolve(response);
    });
}

function addUserGroupContributor(token, projectId, userGroupId) {
    return httpHelper.chaiHttpPost('/agile/project/contributor/usergroup/add', {
        Token: token,
        ProjectId: projectId,
        UserGroupContributor: {
            UserGroupId: userGroupId
        }
    })
    .then(function(response) {
        expect(response.Error).to.be.null;
        
        return Promise.resolve(response);
    });
}

describe('Agile: Project', function() {
    var tokenProjectOwner = null;
    var tokenUser = null;
    var tokenGroupUser = null;

    before(function () {
        return httpHelper.chaiHttpPostPurgeDatabase()
        .then(function () {
            return httpHelper.createUserAndLogon('projectOwner');
        })
        .then(function (_token) {
            tokenProjectOwner = _token;
        })
        .then(function () {
            return httpHelper.createUserAndLogon('user');
        })
        .then(function (_token) {
            tokenUser = _token;
        })
        .then(function () {
            return httpHelper.createUserAndLogon('groupUser');
        })
        .then(function (_token) {
            tokenGroupUser = _token;
        });
    });

    beforeEach(function() {
        return httpHelper.chaiHttpPostPurgeDatabaseArea('agile_project');
    });

    it('Creates a project', function() {
        return createProject(tokenProjectOwner, 'starter_ref');
    });

    it('Adds a user contributor to a project', function() {
        var projectId = null;

        return createProject(tokenProjectOwner, 'starter_ref')
        .then(function(response) {
            expect(response.Error).to.be.null;
            projectId = response.ProjectId;

            return httpHelper.searchForUsers(tokenProjectOwner, 'user');
        })
        .then(function(response) {
            expect(response.Error).to.be.null;
            expect(response.SearchResults).to.be.an.array;
            expect(response.SearchResults).to.have.lengthOf(1);
            var userId = response.SearchResults[0].UserId;

            return addUserContributor(tokenProjectOwner, projectId, userId);
        })
        .then(function(response) {
            expect(response.Error).to.be.null;
        });
    });

    it('Adds a user group contributor to a project', function() {
        var projectId = null;

        return createProject(tokenProjectOwner, 'starter_ref')
        .then(function(response) {
            expect(response.Error).to.be.null;
            projectId = response.ProjectId;

            return userGroupHelper.createUserGroup(tokenProjectOwner, 'dev team', 'dev team desc');
        })
        .then(function(response) {
            expect(response.Error).to.be.null;
            var userGroupId = response.UserGroupId;
            
            return addUserGroupContributor(tokenProjectOwner, projectId, userGroupId);
        })
        .then(function(response) {
            expect(response.Error).to.be.null;
        });
    });
});