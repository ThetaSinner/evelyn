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

var httpHelper = require('../helpers/chai_http_request_helper.js');
var commonRequestsHelper = require('../helpers/common_requests_helper.js');

describe('Calendar', function() {
    var date = new Date ().toISOString();
    var token = null;

    before(function () {
        return commonRequestsHelper.chaiHttpPostPurgeDatabase()
        .then(function () {
            return commonRequestsHelper.createUserAndLogon();
        })
        .then(function (_token) {
            token = _token;
        });
    });

    it('Add Event', function() {
        return httpHelper.post(
            '/calendar/addevent',
            {
                Token: token,
                Title: "The Great Testing Event",
                EventBegin: date,
                EventEnd: date
            }
        )
        .then(function (response) {
            expect(response.Error).to.be.null;
        });
    });

});
