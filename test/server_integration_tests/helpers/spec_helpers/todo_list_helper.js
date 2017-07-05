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
    createTodoList: createTodoList,
    addItem: addItem,
    updateItem: updateItem,
    lookupPreviews: lookupPreviews,
    lookupList: lookupList
};

function createTodoList(token, title) {
    return httpHelper.post('/todolist/create', {
        Token: token,
        Title: title
    })
    .then(serverErrorHelper.newResponseHandler());
}

function addItem(token, todo_list_id, item) {
    return httpHelper.post('/todolist/item/add', {
        Token: token,
        TodoListId: todo_list_id,
        TodoListItem: item
    })
    .then(serverErrorHelper.newResponseHandler());
}

function updateItem(request) {
    return httpHelper.post('/todolist/item/update', request)
    .then(serverErrorHelper.newResponseHandler());
}

function lookupPreviews(token) {
    return httpHelper.post('/todolist/lookuplists', {
        Token: token
    })
    .then(serverErrorHelper.newResponseHandler());
}

function lookupList(token, todo_list_id) {
    return httpHelper.post('/todolist/lookup', {
        Token: token,
        TodoListId: todo_list_id
    })
    .then(serverErrorHelper.newResponseHandler());
}
