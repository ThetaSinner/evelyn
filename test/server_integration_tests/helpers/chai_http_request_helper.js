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

var chai = require('chai');
var chaiHttp = require('chai-http');
var _ = require('lodash');

var httpErrorHelper = require('./chai_http_error_helper.js');

chai.use(chaiHttp);

module.exports = {
    post: chaiHttpPost
};

function chaiHttpPost(action, payload) {
    // For some reason .send() seems to sometimes send an empty payload
    // if you give it an object, which is documented to be allowed...
    if (_.isObject(payload)) {
        payload = JSON.stringify(payload);
    }

    return new Promise(function (resolve, reject) {
        return chai.request('localhost:8080')
        .post(action)
        .send(payload)
        .then(function (res) {
            chai.expect(res).to.have.status(200);
            chai.expect(res).to.be.json;
            resolve(res.body);
        })
        .catch(function (err) {
            reject(httpErrorHelper.wrapChaiHttpError(err));
        });
    });
}
