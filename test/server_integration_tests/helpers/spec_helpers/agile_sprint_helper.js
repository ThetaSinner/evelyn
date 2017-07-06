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

var httpHelper = require('../chai_http_request_helper');
var serverErrorHelper = require('../server_error_helper.js')
var _ = require('lodash');
var moment = require('moment');

module.exports = {
    createSprint: createSprint,
};

function createSprint(token, projectId, title, otherProperties) {
    return httpHelper.post('/agile/sprint/create', {
        Token: token,
        ProjectId: projectId,
        Title: title,
        StartDate: _.get(otherProperties, 'startDate', moment().utc().subtract(1, 'days')),
        EndDate: _.get(otherProperties, 'endDate', moment().utc().add(1, 'days'))
    })
    .then(serverErrorHelper.newResponseHandler());
}
