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

function lookupProjectPreviews(token) {
    return httpHelper.chaiHttpPost('/agile/project/lookup/contributingto', {
        Token: token
    })
    .then(function(response) {
        expect(response.Error).to.be.null;
        
        return Promise.resolve(response);
    });
}

function lookupProject(token, projectId) {
    return httpHelper.chaiHttpPost('/agile/project/lookup', {
        Token: token,
        ProjectId: projectId
    })
    .then(function(response) {
        if (response.Error === null) {
            return Promise.resolve(response);
        }
        else {
            return Promise.reject(response);
        }
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
            projectId = response.ProjectId;

            return httpHelper.searchForUsers(tokenProjectOwner, 'user');
        })
        .then(function(response) {
            expect(response.SearchResults).to.be.an.array;
            expect(response.SearchResults).to.have.lengthOf(1);
            var userId = response.SearchResults[0].UserId;

            return addUserContributor(tokenProjectOwner, projectId, userId);
        });
    });

    it('Adds a user group contributor to a project', function() {
        var projectId = null;

        return createProject(tokenProjectOwner, 'starter_ref')
        .then(function(response) {
            projectId = response.ProjectId;

            return userGroupHelper.createUserGroup(tokenProjectOwner, 'dev team', 'dev team desc');
        })
        .then(function(response) {
            var userGroupId = response.UserGroupId;
            
            return addUserGroupContributor(tokenProjectOwner, projectId, userGroupId);
        });
    });

    describe('Lookup project previews', function() {
        it('Looks up a project preview', function() {
            return createProject(tokenProjectOwner, 'starter_ref')
            .then(function(response) {
                return createProject(tokenProjectOwner, 'sinker_ref');
            })
            .then(function(response) {
                return lookupProjectPreviews(tokenProjectOwner);
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

            return createProject(tokenProjectOwner, 'starter_ref')
            .then(function(response) {
                return createProject(tokenProjectOwner, 'sinker_ref');
            })
            .then(function(response) {
                projectId = response.ProjectId;

                return httpHelper.searchForUsers(tokenProjectOwner, 'user');
            })
            .then(function(response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);
                var userId = response.SearchResults[0].UserId;

                return addUserContributor(tokenProjectOwner, projectId, userId);
            })
            .then(function() {
                return lookupProjectPreviews(tokenUser);
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

            return createProject(tokenProjectOwner, 'starter_ref')
            .then(function(response) {
                return createProject(tokenProjectOwner, 'sinker_ref');
            })
            .then(function(response) {
                projectId = response.ProjectId;

                return userGroupHelper.createUserGroup(tokenProjectOwner, 'group name', 'group description');
            })
            .then(function(response) {
                userGroupId = response.UserGroupId;

                return httpHelper.searchForUsers(tokenProjectOwner, 'user');
            })
            .then(function(response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);
                var userId = response.SearchResults[0].UserId;

                return userGroupHelper.addMember(tokenProjectOwner, userGroupId, userId);
            })
            .then(function() {
                return addUserGroupContributor(tokenProjectOwner, projectId, userGroupId);
            })
            .then(function() {
                return lookupProjectPreviews(tokenUser);
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
            return createProject(tokenProjectOwner, 'starter_ref')
            .then(function(response) {
                return lookupProject(tokenProjectOwner, response.ProjectId);
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

            return createProject(tokenProjectOwner, 'starter_ref')
            .then(function(response) {
                projectId1 = response.ProjectId;

                return createProject(tokenProjectOwner, 'sinker_ref');
            })
            .then(function(response) {
                projectId2 = response.ProjectId;

                return httpHelper.searchForUsers(tokenProjectOwner, 'user');
            })
            .then(function(response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);
                userId = response.SearchResults[0].UserId;

                return addUserContributor(tokenProjectOwner, projectId1, userId);
            })
            .then(function() {
                return lookupProject(tokenUser, projectId2);
            })
            .catch(function(response) {
                // Failed to lookup agile project
                expect(response.Error.ErrorCode).to.equal('1006005');

                return Promise.resolve();
            })
            .then(function() {
                return lookupProject(tokenUser, projectId1);
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

            return createProject(tokenProjectOwner, 'starter_ref')
            .then(function(response) {
                projectId1 = response.ProjectId;

                return createProject(tokenProjectOwner, 'sinker_ref');
            })
            .then(function(response) {
                projectId2 = response.ProjectId;

                return userGroupHelper.createUserGroup(tokenProjectOwner, 'group name', 'group description');
            })
            .then(function(response) {
                userGroupId = response.UserGroupId;

                return httpHelper.searchForUsers(tokenProjectOwner, 'user');
            })
            .then(function(response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);
                userId = response.SearchResults[0].UserId;

                return userGroupHelper.addMember(tokenProjectOwner, userGroupId, userId);
            })
            .then(function() {
                return addUserGroupContributor(tokenProjectOwner, projectId2, userGroupId);
            })
            .then(function() {
                return lookupProject(tokenUser, projectId1);
            })
            .catch(function(response) {
                // Failed to lookup agile project
                expect(response.Error.ErrorCode).to.equal('1006005');

                return Promise.resolve();
            })
            .then(function() {
                return lookupProject(tokenUser, projectId2);
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
