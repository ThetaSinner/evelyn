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

var todoListHelper = require('../helpers/spec_helpers/todo_list_helper.js');

describe('Todo List', function() {
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

    beforeEach(function() {
        return commonRequestsHelper.chaiHttpPostPurgeDatabaseArea('todolist');
    });

    it('Create', function() {
        return todoListHelper.createTodoList(token, "Test Title");
    });

    it('Add List Item', function() {
        return todoListHelper.createTodoList(token, "Add List Item")
        .then(function (response) {
            return todoListHelper.addItem(token, response.TodoListId, {
                Text: "Eggs",
                IsDone: false
            });
        });
    });

    it('Mark item done', function() {
        var todo_list_id = null;

        return todoListHelper.createTodoList(token, "Mark item done")
        .then(function (response) {
            todo_list_id = response.TodoListId;

            return todoListHelper.addItem(token, todo_list_id, {
                Text: "Eggs",
                IsDone: false
            });
        })
        .then(function () {
            return todoListHelper.updateItem({
                Token: token,
                TodoListId: todo_list_id,
                ItemIndex: 0,
                IsDone: true
            });
        });
    });

    describe("Lookup", function () {
        it('Lookup previews', function() {
            var todo_list_id_1 = null;
            var todo_list_id_2 = null;

            return todoListHelper.createTodoList(token, "Lookup previews 1")
            .then(function (response) {
                todo_list_id_1 = response.TodoListId;
                return todoListHelper.createTodoList(token, "Lookup previews 2");
            })
            .then(function (response) {
                todo_list_id_2 = response.TodoListId;

                return todoListHelper.lookupPreviews(token);
            })
            .then(function (response) {
                expect(response.Error).to.be.null;
                expect(response.TodoLists).to.have.lengthOf(2);

                var todo_lists = response.TodoLists;
                expect(todo_lists[0]).to.have.property('Title')
                .that.is.a('string')
                .that.equals("Lookup previews 1");

                expect(todo_lists[0]).to.have.property('TodoListId')
                .that.is.a('string')
                .that.equals(todo_list_id_1);

                expect(todo_lists[1]).to.have.property('Title')
                .that.is.a('string')
                .that.equals("Lookup previews 2");

                expect(todo_lists[1]).to.have.property('TodoListId')
                .that.is.a('string')
                .that.equals(todo_list_id_2);
            });
        });

        it('Lookup a todo list', function() {
            return todoListHelper.createTodoList(token, "Lookup a todo list")
            .then(function (response) {
                var todo_list_id = response.TodoListId;

                return todoListHelper.lookupList(token, todo_list_id);
            })
            .then(function (response) {
                expect(response.Error).to.be.null;
                expect(response.TodoList).to.not.be.null;

                var todo_list = response.TodoList;
                expect(todo_list.Title).to.be.a('string').that.equals('Lookup a todo list');
                expect(todo_list.TodoListItems).to.be.an.array;
                expect(todo_list.TodoListItems).to.have.lengthOf(0);
            });
        });
    });
});
