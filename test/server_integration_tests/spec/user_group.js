if (!global.Promise) {
    global.Promise = require('bluebird');
}

var expect = require('chai').expect;
var _ = require('lodash');

var httpHelper = require('../helpers/chai-http-request-helper.js');

function createUserGroup(token, name, description) {
    return new Promise(function (resolve, reject) {
        httpHelper.chaiHttpPost(
            '/usergroup/create',
            {
                Token: token,
                Name: name,
                Description: description
            }
        )
        .then(function (response) {
            expect(response.Error).to.be.null;
            expect(response.UserGroupId).to.not.be.null;
            resolve(response);
        })
        .catch(function (e) {
            reject(e);
        });
    });
}

function addMember(token, user_group_id, user_id) {
    return new Promise(function (resolve, reject) {
        httpHelper.chaiHttpPost(
            '/usergroup/addmember',
            {
                Token: token,
                UserGroupId: user_group_id,
                Member: {
                    UserId: user_id
                }
            }
        )
        .then(function (response) {
            expect(response.Error).to.be.null;
            resolve(response);
        })
        .catch(function (e) {
            reject(e);
        });
    });
}

function lookupGroups(token) {
    return new Promise(function (resolve, reject) {
        httpHelper.chaiHttpPost(
            '/usergroup/lookupgroups',
            {
                Token: token
            }
        )
        .then(function (response) {
            expect(response.Error).to.be.null;
            resolve(response);
        })
        .catch(function (e) {
            reject(e);
        });
    });
}

function lookupGroup(token, groupId) {
    return new Promise(function (resolve, reject) {
        httpHelper.chaiHttpPost(
            '/usergroup/lookupgroup',
            {
                Token: token,
                UserGroupId: groupId
            }
        )
        .then(function (response) {
            expect(response.Error).to.be.null;
            resolve(response);
        })
        .catch(function (e) {
            reject(e);
        });
    });
}

describe('User groups', function() {
    var token1 = null;
    var token2 = null;

    before(function () {
        return httpHelper.chaiHttpPostPurgeDatabase()
        .then(function () {
            return httpHelper.createUserAndLogon('user1');
        })
        .then(function (_token) {
            token1 = _token;
        })
        .then(function () {
            return httpHelper.createUserAndLogon('user2');
        })
        .then(function (_token) {
            token2 = _token;
        });
    });

    beforeEach(function() {
        return httpHelper.chaiHttpPostPurgeDatabaseArea('usergroup');
    });

    it('Create a group', function() {
        return createUserGroup(token1, "my dev team", "the description of the team");
    });

    it('Add a member', function() {
        return createUserGroup(token1, "my dev team", "the description of the team")
        .then(function (response) {
            return addMember(token1, response.UserGroupId, "some user id");
        });
    });

    describe('Lookup', function() {
        it('Lookup group previews', function() {
            return createUserGroup(token1, 'my dev team', 'the description of the team')
            .then(function (response) {
                return addMember(token1, response.UserGroupId, 'some user id');
            })
            .then(function (response) {
                return createUserGroup(token1, 'my other dev team', 'some other description');
            })
            .then(function (response) {
                return lookupGroups(token1);
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

        it('Lookup group', function() {
            var groupId = null;

            return createUserGroup(token1, 'my dev team', 'the description of the team')
            .then(function (response) {
                groupId = response.UserGroupId;
                return addMember(token1, response.UserGroupId, 'some user id');
            })
            .then(function (response) {
                return lookupGroup(token1, groupId);
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
                expect(member1.UserId).to.equal('some user id');
            });
        });
    });
});
