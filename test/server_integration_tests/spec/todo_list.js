if (!global.Promise) {
    global.Promise = require('bluebird');
}

var expect = require('chai').expect;
var _ = require('lodash');

var httpHelper = require('../helpers/chai-http-request-helper.js');

function createTodoList(token, title) {
    return new Promise(function (resolve, reject) {
        httpHelper.chaiHttpPost(
            '/todolist/create',
            {
                Token: token,
                Title: title
            }
        )
        .then(function (response) {
            expect(response.Error).to.be.null;
            expect(response.TodoListId).to.not.be.null;
            resolve(response);
        })
        .catch(function (e) {
            reject(e);
        });
    });
}

function addItem(token, todo_list_id, item) {
    return new Promise(function (resolve, reject) {
        httpHelper.chaiHttpPost(
            '/todolist/item/add',
            {
                Token: token,
                TodoListId: todo_list_id,
                TodoListItem: item
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

function updateItem(request) {
    return new Promise(function (resolve, reject) {
        httpHelper.chaiHttpPost(
            '/todolist/item/update',
            request
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

function lookupPreviews(token) {
    return new Promise(function (resolve, reject) {
        httpHelper.chaiHttpPost(
            '/todolist/lookuplists',
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

function lookupList(token, todo_list_id) {
    return new Promise(function (resolve, reject) {
        httpHelper.chaiHttpPost(
            '/todolist/lookup',
            {
                Token: token,
                TodoListId: todo_list_id
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

describe('Todo List', function() {
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
        return httpHelper.chaiHttpPostPurgeDatabaseArea('todolist');
    });

    it('Create', function() {
        return createTodoList(token, "Test Title");
    });

    it('Add List Item', function() {
        return createTodoList(token, "Add List Item")
        .then(function (response) {
            return addItem(token, response.TodoListId, {
                Text: "Eggs",
                IsDone: false
            });
        });
    });

    it('Mark item done', function() {
        var todo_list_id = null;

        return createTodoList(token, "Mark item done")
        .then(function (response) {
            todo_list_id = response.TodoListId;

            return addItem(token, todo_list_id, {
                Text: "Eggs",
                IsDone: false
            });
        })
        .then(function (response) {
            return updateItem({
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

            return createTodoList(token, "Lookup previews 1")
            .then(function (response) {
                todo_list_id_1 = response.TodoListId;
                return createTodoList(token, "Lookup previews 2");
            })
            .then(function (response) {
                todo_list_id_2 = response.TodoListId;

                return lookupPreviews(token);
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
            return createTodoList(token, "Lookup a todo list")
            .then(function (response) {
                var todo_list_id = response.TodoListId;

                return lookupList(token, todo_list_id);
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
