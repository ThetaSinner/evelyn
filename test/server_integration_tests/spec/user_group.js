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

var userGroupHelper = require('../helpers/spec_helpers/user_group_helper.js');

describe('User Groups', function() {
    var token1 = null;
    var token2 = null;

    before(function () {
        return commonRequestsHelper.chaiHttpPostPurgeDatabase()
        .then(function () {
            return commonRequestsHelper.createUserAndLogon('user1');
        })
        .then(function (_token) {
            token1 = _token;
        })
        .then(function () {
            return commonRequestsHelper.createUserAndLogon('user2');
        })
        .then(function (_token) {
            token2 = _token;
        });
    });

    beforeEach(function() {
        return commonRequestsHelper.chaiHttpPostPurgeDatabaseArea('usergroup');
    });

    it('Create a group', function() {
        return userGroupHelper.createUserGroup(token1, "my dev team", "the description of the team");
    });

    it('Remove a group', function() {
        var groupID = null;
        return userGroupHelper.createUserGroup(token1, "my dev team", "the description of the team")
        .then(function (response) {
            groupID = response.UserGroupId;
            return userGroupHelper.lookupGroups(token1);
        })
        .then(function (response) {
            expect(response.UserGroups).to.have.lengthOf(1);
            return userGroupHelper.removeUserGroup(token1, groupID);
        })
        .then(function (response) {
            return userGroupHelper.lookupGroups(token1);
        })
        .then(function (response) {
            expect(response.UserGroups).to.have.lengthOf(0);
        });
    });

    describe('Members', function() {
        it('Add a member', function() {
            return userGroupHelper.createUserGroup(token1, "my dev team", "the description of the team")
            .then(function (response) {
                return userGroupHelper.addMember(token1, response.UserGroupId, "some user id");
            });
        });

        it('Remove a member', function() {
            var groupId = null;
            var userId = null;

            return userGroupHelper.createUserGroup(token1, "my dev team", "the description of the team")
            .then(function (response) {
                groupId = response.UserGroupId;
                return commonRequestsHelper.searchForUsers(token1, 'user2');
            })
            .then(function (response) {
                userId = response.SearchResults[0].UserId;
                return userGroupHelper.addMember(token1, groupId, userId);
            })
            .then(function (response) {
                return userGroupHelper.lookupGroup(token1, groupId);
            })
            .then(function (response) {
                expect(response.UserGroup.Members).to.be.an.array;
                expect(response.UserGroup.Members).to.have.lengthOf(1);
                return userGroupHelper.removeMember(token1, groupId, userId);
            })
            .then(function (response) {
                return userGroupHelper.lookupGroup(token1, groupId);
            })
            .then(function (response) {
                expect(response.UserGroup.Members).to.be.an.array;
                expect(response.UserGroup.Members).to.have.lengthOf(0);
            })
            ;
        });

        it('Adding same member twice only adds once', function() {
            var groupId = null;
            var userId = null;

            return userGroupHelper.createUserGroup(token1, 'my dev team', 'the description of the team')
            .then(function (response) {
                groupId = response.UserGroupId;
                return commonRequestsHelper.searchForUsers(token1, 'user1');
            })
            .then(function (response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);
                userId = response.SearchResults[0].UserId;

                return userGroupHelper.addMember(token1, groupId, userId);
            })
            .then(function (response) {
                return userGroupHelper.addMember(token1, groupId, userId);
            })
            .then(function (response) {
                return userGroupHelper.lookupGroup(token1, groupId);
            })
            .then(function (response) {
                expect(response.UserGroup).to.be.ok;

                var userGroup = response.UserGroup;
                expect(userGroup.Name).to.equal('my dev team');
                expect(userGroup.Description).to.equal('the description of the team');

                expect(userGroup.Members).to.be.ok;

                var members = userGroup.Members;
                expect(members).to.be.an.array;
                expect(members).to.have.lengthOf(1);

                var member1 = members[0];
                expect(member1).to.be.ok;
                expect(member1.UserId).to.equal(userId);
                expect(member1.UserName).to.equal('user1');
            });
        });
    });

    describe('Lookup', function() {
        it('Lookup group previews', function() {
            return userGroupHelper.createUserGroup(token1, 'my dev team', 'the description of the team')
            .then(function (response) {
                return userGroupHelper.addMember(token1, response.UserGroupId, 'some user id');
            })
            .then(function (response) {
                return userGroupHelper.createUserGroup(token1, 'my other dev team', 'some other description');
            })
            .then(function (response) {
                return userGroupHelper.lookupGroups(token1);
            })
            .then(function (response) {
                expect(response.UserGroups).to.be.ok;
                var userGroups = response.UserGroups;
                expect(userGroups).to.be.an.array;
                expect(userGroups).to.have.lengthOf(2);

                var group1 = userGroups[0];
                expect(group1.Name).to.equal('my dev team');
                expect(group1.Description).to.equal('the description of the team');

                var group2 = userGroups[1];
                expect(group2.Name).to.equal('my other dev team');
                expect(group2.Description).to.equal('some other description');
            });
        });

        it('Lookup group with member', function() {
            var groupId = null;
            var userId = null;

            return userGroupHelper.createUserGroup(token1, 'my dev team', 'the description of the team')
            .then(function (response) {
                groupId = response.UserGroupId;
                return commonRequestsHelper.searchForUsers(token1, 'user1');
            })
            .then(function (response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);
                userId = response.SearchResults[0].UserId;

                return userGroupHelper.addMember(token1, groupId, userId);
            })
            .then(function (response) {
                return userGroupHelper.lookupGroup(token1, groupId);
            })
            .then(function (response) {
                expect(response.UserGroup).to.be.ok;

                var userGroup = response.UserGroup;
                expect(userGroup.Name).to.equal('my dev team');
                expect(userGroup.Description).to.equal('the description of the team');

                expect(userGroup.Members).to.be.ok;

                var members = userGroup.Members;
                expect(members).to.be.an.array;
                expect(members).to.have.lengthOf(1);

                var member1 = members[0];
                expect(member1).to.be.ok;
                expect(member1.UserId).to.equal(userId);
                expect(member1.UserName).to.equal('user1');
            });
        });

        it('Lookup group with invalid member', function() {
            var groupId = null;
            var userId = null;

            return userGroupHelper.createUserGroup(token1, 'my dev team', 'the description of the team')
            .then(function (response) {
                groupId = response.UserGroupId;

                return userGroupHelper.addMember(token1, groupId, 'invalid user id');
            })
            .then(function (response) {
                return userGroupHelper.lookupGroup(token1, groupId);
            })
            .then(function (response) {
                expect(response.UserGroup).to.be.ok;

                var userGroup = response.UserGroup;
                expect(userGroup.Name).to.equal('my dev team');
                expect(userGroup.Description).to.equal('the description of the team');

                expect(userGroup.Members).to.be.ok;

                var members = userGroup.Members;
                expect(members).to.be.an.array;
                expect(members).to.have.lengthOf(1);

                var member1 = members[0];
                expect(member1).to.be.null;
            });
        });

        it('Lookup groups restricted to created by', function() {
            return userGroupHelper.createUserGroup(token1, 'my dev team', 'the description of the team')
            .then(function (response) {
                groupId1 = response.UserGroupId;
                return userGroupHelper.createUserGroup(token2, 'other dev team', 'description');
            })
            .then(function (response) {
                return userGroupHelper.lookupGroups(token1);
            })
            .then(function (response) {
                expect(response.UserGroups).to.be.ok;
                var userGroups = response.UserGroups;
                expect(userGroups).to.be.an.array;
                expect(userGroups).to.have.lengthOf(1);

                var group = userGroups[0];
                expect(group.Name).to.equal('my dev team');
                expect(group.Description).to.equal('the description of the team');
            })
            .then(function (response) {
                return userGroupHelper.lookupGroups(token2);
            })
            .then(function (response) {
                expect(response.UserGroups).to.be.ok;
                var userGroups = response.UserGroups;
                expect(userGroups).to.be.an.array;
                expect(userGroups).to.have.lengthOf(1);

                var group = userGroups[0];
                expect(group.Name).to.equal('other dev team');
                expect(group.Description).to.equal('description');
            });
        });

        it('Lookup groups restricted to membership', function() {
            var group2Id = null;

            return userGroupHelper.createUserGroup(token1, 'my dev team', 'the description of the team')
            .then(function (response) {

                return userGroupHelper.createUserGroup(token1, 'other dev team', 'description');
            })
            .then(function (response) {
                expect(response.UserGroupId).to.be.ok;
                group2Id = response.UserGroupId;

                return commonRequestsHelper.searchForUsers(token1, 'user2');
            })
            .then(function (response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);
                var user2Id = response.SearchResults[0].UserId;

                return userGroupHelper.addMember(token1, group2Id, user2Id)
            })
            .then(function (response) {

                return userGroupHelper.lookupGroups(token2);
            })
            .then(function (response) {
                expect(response.UserGroups).to.be.ok;
                var userGroups = response.UserGroups;
                expect(userGroups).to.be.an.array;
                expect(userGroups).to.have.lengthOf(1);

                var group = userGroups[0];
                expect(group.Name).to.equal('other dev team');
                expect(group.Description).to.equal('description');
            });
        });
    });
});
