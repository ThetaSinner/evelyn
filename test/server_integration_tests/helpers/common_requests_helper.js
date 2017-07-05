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

var httpHelper = require('./chai_http_request_helper.js');

module.exports = {
    chaiHttpPostPurgeDatabase: chaiHttpPostPurgeDatabase,
    chaiHttpPostPurgeDatabaseArea: chaiHttpPostPurgeDatabaseArea,
    createUserAndLogon: createUserAndLogon,
    searchForUsers: searchForUsers
};

function chaiHttpPostPurgeDatabase() {
    return new Promise(function (resolve, reject) {
        httpHelper.post(
            '/purge',
            {
                Token: 'a temporary token',
                TargetType: 'database',
                Target: ''
            }
        )
        .then(function (response) {
            if (_.isObject(response.Error)) {
                console.log('Purge database error', response.Error.ErrorCode, response.Error.ErrorMessage);
            }

            expect(response.Error).to.be.null;
            resolve();
        }).catch(function (e) {
            reject(e);
        });
    });
}

function chaiHttpPostPurgeDatabaseArea(target) {
    return new Promise(function (resolve, reject) {
        httpHelper.post(
            '/purge',
            {
                Token: 'a temporary token',
                TargetType: 'database_area',
                Target: target
            }
        )
        .then(function (response) {
            if (_.isObject(response.Error)) {
                console.log('Purge database area error', response.Error.ErrorCode, response.Error.ErrorMessage);
            }

            expect(response.Error).to.be.null;
            resolve();
        }).catch(function (e) {
            reject(e);
        })
    });
}

function createUserAndLogon(user_ref) {
    if (!_.isString(user_ref)) {
        user_ref = 'rupert';
    }

    return httpHelper.post(
        '/user/create',
        {
            UserName: user_ref,
            EmailAddress: user_ref + "@evelyn.com",
            Password: "asdf"
        }
    )
    .then(function (response) {
        expect(response.Error).to.be.null;

        return httpHelper.post(
            '/user/logon',
            {
                EmailAddress: user_ref + '@evelyn.com',
                Password: 'asdf'
            }
        );
    })
    .then(function (response) {
        expect(response.Error).to.be.null;
        return Promise.resolve(response.Token);
    });
}

function searchForUsers(token, user_ref) {
    if (!_.isString(user_ref)) {
        user_ref = 'rupert';
    }

    return httpHelper.post(
        '/user/search',
        {
            Token: token,
            Query: user_ref
        }
    )
    .then(function (response) {
        expect(response.Error).to.be.null;
        return Promise.resolve(response);
    });
}
