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

var expect = require('chai').expect;

var httpHelper = require('../chai_http_request_helper');
var serverErrorHelper = require('../server_error_helper.js')
var _ = require('lodash');

module.exports = {
    createUserGroup: createUserGroup,
    addMember: addMember,
    lookupGroups: lookupGroups,
    lookupGroup: lookupGroup
};

function createUserGroup(token, name, description) {
    return httpHelper.post('/usergroup/create', {
        Token: token,
        Name: name,
        Description: description
    })
    .then(serverErrorHelper.newResponseHandler())
    .then(function (response) {
        expect(response.UserGroupId).to.not.be.null;
        return Promise.resolve(response);
    });
}

function addMember(token, user_group_id, user_id) {
    return new Promise(function (resolve, reject) {
        httpHelper.post(
            '/usergroup/member/add',
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
        httpHelper.post(
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
        httpHelper.post(
            '/usergroup/lookup',
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
