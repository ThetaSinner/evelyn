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
var agileProjectHelper = require('../helpers/spec_helpers/agile_project_helper.js');
var userGroupHelper = require('../helpers/spec_helpers/user_group_helper.js');

describe('Agile: Project', function() {
    var tokenProjectOwner = null;
    var tokenUser = null;
    var tokenGroupUser = null;

    before(function () {
        return commonRequestsHelper.chaiHttpPostPurgeDatabase()
        .then(function () {
            return commonRequestsHelper.createUserAndLogon('projectOwner');
        })
        .then(function (_token) {
            tokenProjectOwner = _token;
        })
        .then(function () {
            return commonRequestsHelper.createUserAndLogon('user');
        })
        .then(function (_token) {
            tokenUser = _token;
        })
        .then(function () {
            return commonRequestsHelper.createUserAndLogon('groupUser');
        })
        .then(function (_token) {
            tokenGroupUser = _token;
        });
    });

    beforeEach(function() {
        return commonRequestsHelper.chaiHttpPostPurgeDatabaseArea('agile_project');
    });

    it('Creates a project', function() {
        return agileProjectHelper.createProject(tokenProjectOwner, 'starter_ref');
    });

    it('Adds a user contributor to a project', function() {
        var projectId = null;

        return agileProjectHelper.createProject(tokenProjectOwner, 'starter_ref')
        .then(function(response) {
            projectId = response.ProjectId;

            return commonRequestsHelper.searchForUsers(tokenProjectOwner, 'user');
        })
        .then(function(response) {
            expect(response.SearchResults).to.be.an.array;
            expect(response.SearchResults).to.have.lengthOf(1);
            var userId = response.SearchResults[0].UserId;

            return agileProjectHelper.addUserContributor(tokenProjectOwner, projectId, userId);
        });
    });

    it('Adds a user group contributor to a project', function() {
        var projectId = null;

        return agileProjectHelper.createProject(tokenProjectOwner, 'starter_ref')
        .then(function(response) {
            projectId = response.ProjectId;

            return userGroupHelper.createUserGroup(tokenProjectOwner, 'dev team', 'dev team desc');
        })
        .then(function(response) {
            var userGroupId = response.UserGroupId;
            
            return agileProjectHelper.addUserGroupContributor(tokenProjectOwner, projectId, userGroupId);
        });
    });

    describe('Lookup project previews', function() {
        it('Looks up a project preview', function() {
            return agileProjectHelper.createProject(tokenProjectOwner, 'starter_ref')
            .then(function(response) {
                return agileProjectHelper.createProject(tokenProjectOwner, 'sinker_ref');
            })
            .then(function(response) {
                return agileProjectHelper.lookupProjectPreviews(tokenProjectOwner);
            })
            .then(function(response) {
                expect(response.Projects).to.be.an.array;
                expect(response.Projects).to.have.lengthOf(2);

                var project_1 = response.Projects[0];
                expect(project_1.ProjectId).to.be.ok;
                expect(project_1.Name).to.equal('name_starter_ref');
                expect(project_1.ShortName).to.equal('short_name_starter_ref');
                expect(project_1.Description).to.equal('description_starter_ref');

                var project_2 = response.Projects[1];
                expect(project_2.ProjectId).to.be.ok;
                expect(project_2.Name).to.equal('name_sinker_ref');
                expect(project_2.ShortName).to.equal('short_name_sinker_ref');
                expect(project_2.Description).to.equal('description_sinker_ref');
            });
        });

        it('Allows user contributor to see preview', function() {
            var projectId = null;

            return agileProjectHelper.createProject(tokenProjectOwner, 'starter_ref')
            .then(function(response) {
                return agileProjectHelper.createProject(tokenProjectOwner, 'sinker_ref');
            })
            .then(function(response) {
                projectId = response.ProjectId;

                return commonRequestsHelper.searchForUsers(tokenProjectOwner, 'user');
            })
            .then(function(response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);
                var userId = response.SearchResults[0].UserId;

                return agileProjectHelper.addUserContributor(tokenProjectOwner, projectId, userId);
            })
            .then(function() {
                return agileProjectHelper.lookupProjectPreviews(tokenUser);
            })
            .then(function(response) {
                expect(response.Projects).to.be.an.array;
                expect(response.Projects).to.have.lengthOf(1);

                var project = response.Projects[0];
                expect(project.ProjectId).to.be.ok;
                expect(project.Name).to.equal('name_sinker_ref');
                expect(project.ShortName).to.equal('short_name_sinker_ref');
                expect(project.Description).to.equal('description_sinker_ref');
            });
        });

        it('Allows user group contributor to see preview', function() {
            var projectId = null;
            var userGroupId = null;

            return agileProjectHelper.createProject(tokenProjectOwner, 'starter_ref')
            .then(function(response) {
                return agileProjectHelper.createProject(tokenProjectOwner, 'sinker_ref');
            })
            .then(function(response) {
                projectId = response.ProjectId;

                return userGroupHelper.createUserGroup(tokenProjectOwner, 'group name', 'group description');
            })
            .then(function(response) {
                userGroupId = response.UserGroupId;

                return commonRequestsHelper.searchForUsers(tokenProjectOwner, 'user');
            })
            .then(function(response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);
                var userId = response.SearchResults[0].UserId;

                return userGroupHelper.addMember(tokenProjectOwner, userGroupId, userId);
            })
            .then(function() {
                return agileProjectHelper.addUserGroupContributor(tokenProjectOwner, projectId, userGroupId);
            })
            .then(function() {
                return agileProjectHelper.lookupProjectPreviews(tokenUser);
            })
            .then(function(response) {
                expect(response.Projects).to.be.an.array;
                expect(response.Projects).to.have.lengthOf(1);

                var project = response.Projects[0];
                expect(project.ProjectId).to.be.ok;
                expect(project.Name).to.equal('name_sinker_ref');
                expect(project.ShortName).to.equal('short_name_sinker_ref');
                expect(project.Description).to.equal('description_sinker_ref');
            });
        });
    });

    describe('Lookup project', function() {
        it('Looks up a project', function() {
            return agileProjectHelper.createProject(tokenProjectOwner, 'starter_ref')
            .then(function(response) {
                return agileProjectHelper.lookupProject(tokenProjectOwner, response.ProjectId);
            })
            .then(function(response) {
                expect(response.Project).to.be.ok;

                var project = response.Project;
                expect(project.ProjectId).to.be.ok;
                expect(project.Name).to.equal('name_starter_ref');
                expect(project.ShortName).to.equal('short_name_starter_ref');
                expect(project.Description).to.equal('description_starter_ref');
                expect(project.UserContributors).to.be.an.array;
                expect(project.UserContributors).to.be.empty;
                expect(project.UserGroupContributors).to.be.an.array;
                expect(project.UserGroupContributors).to.be.empty;
            });
        });

        it('Allows user contributor to see project', function() {
            var projectId1 = null;
            var projectId2 = null;
            var userId = null;

            return agileProjectHelper.createProject(tokenProjectOwner, 'starter_ref')
            .then(function(response) {
                projectId1 = response.ProjectId;

                return agileProjectHelper.createProject(tokenProjectOwner, 'sinker_ref');
            })
            .then(function(response) {
                projectId2 = response.ProjectId;

                return commonRequestsHelper.searchForUsers(tokenProjectOwner, 'user');
            })
            .then(function(response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);
                userId = response.SearchResults[0].UserId;

                return agileProjectHelper.addUserContributor(tokenProjectOwner, projectId1, userId);
            })
            .then(function() {
                return agileProjectHelper.lookupProject(tokenUser, projectId2, true);
            })
            .catch(function(response) {
                // Failed to lookup agile project
                expect(response.Error.ErrorCode).to.equal('1006005');

                return Promise.resolve();
            })
            .then(function() {
                return agileProjectHelper.lookupProject(tokenUser, projectId1);
            })
            .then(function(response) {
                expect(response.Project).to.be.ok;

                var project = response.Project;
                expect(project.ProjectId).to.be.ok;
                expect(project.Name).to.equal('name_starter_ref');
                expect(project.ShortName).to.equal('short_name_starter_ref');
                expect(project.Description).to.equal('description_starter_ref');
                expect(project.UserContributors).to.be.an.array;
                expect(project.UserContributors).to.have.lengthOf(1);
                expect(project.UserContributors[0].UserId).to.equal(userId);
                expect(project.UserGroupContributors).to.be.an.array;
                expect(project.UserGroupContributors).to.be.empty;
            });
        });

        it('Allows user group contributor to see project', function() {
            var projectId1 = null;
            var projectId2 = null;
            var userGroupId = null;
            var userId = null;

            return agileProjectHelper.createProject(tokenProjectOwner, 'starter_ref')
            .then(function(response) {
                projectId1 = response.ProjectId;

                return agileProjectHelper.createProject(tokenProjectOwner, 'sinker_ref');
            })
            .then(function(response) {
                projectId2 = response.ProjectId;

                return userGroupHelper.createUserGroup(tokenProjectOwner, 'group name', 'group description');
            })
            .then(function(response) {
                userGroupId = response.UserGroupId;

                return commonRequestsHelper.searchForUsers(tokenProjectOwner, 'user');
            })
            .then(function(response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);
                userId = response.SearchResults[0].UserId;

                return userGroupHelper.addMember(tokenProjectOwner, userGroupId, userId);
            })
            .then(function() {
                return agileProjectHelper.addUserGroupContributor(tokenProjectOwner, projectId2, userGroupId);
            })
            .then(function() {
                return agileProjectHelper.lookupProject(tokenUser, projectId1, true);
            })
            .catch(function(response) {
                // Failed to lookup agile project
                expect(response.Error.ErrorCode).to.equal('1006005');

                return Promise.resolve();
            })
            .then(function() {
                return agileProjectHelper.lookupProject(tokenUser, projectId2);
            })
            .then(function(response) {
                expect(response.Project).to.be.ok;

                var project = response.Project;
                expect(project.ProjectId).to.be.ok;
                expect(project.Name).to.equal('name_sinker_ref');
                expect(project.ShortName).to.equal('short_name_sinker_ref');
                expect(project.Description).to.equal('description_sinker_ref');
                expect(project.UserContributors).to.be.an.array;
                expect(project.UserContributors).to.be.empty;
                expect(project.UserGroupContributors).to.be.an.array;
                expect(project.UserGroupContributors).to.have.lengthOf(1);
                expect(project.UserGroupContributors[0].UserGroupId).to.equal(userGroupId);
            });
        });
    });
});
