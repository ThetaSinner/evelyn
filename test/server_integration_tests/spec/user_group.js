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

describe('User groups', function() {
    var token = null;

    before(function () {
        return httpHelper.chaiHttpPostPurgeDatabase()
        .then(function () {
            return httpHelper.createUserAndLogon();
        })
        .then(function (_token) {
            token = _token;
        });
    });

    beforeEach(function() {
        return httpHelper.chaiHttpPostPurgeDatabaseArea('usergroup');
    });

    it('Create a group', function() {
        return createUserGroup(token, "my dev team", "the description of the team");
    });

    it('Add a member', function() {
        return createUserGroup(token, "my dev team", "the description of the team")
        .then(function (response) {
            return addMember(token, response.UserGroupId, "some user id");
        });
    });
});
