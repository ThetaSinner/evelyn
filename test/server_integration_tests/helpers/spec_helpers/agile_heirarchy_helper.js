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

module.exports = {
    createLink: createLink,
};

function createLink(token, projectId, from_type, from_id, to_type, to_id) {
    return httpHelper.post('/agile/heirarchy/link', {
        Token: token,
        ProjectId: projectId,
        LinkFromTypeName: from_type,
        LinkFromId: from_id,
        LinkToTypeName: to_type,
        LinkToId: to_id
    })
    .then(serverErrorHelper.newResponseHandler());
}
