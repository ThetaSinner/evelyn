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
var chaiSubset = require('chai-subset');
var moment = require('moment');

var httpHelper = require('../helpers/chai_http_request_helper.js');
var commonRequestsHelper = require('../helpers/common_requests_helper.js');

var simpleTaskHelper = require('../helpers/spec_helpers/simple_task_helper.js');

chai.use(chaiSubset);
var expect = chai.expect;

describe('Simple Task', function() {
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

    it('Creates a task', function() {
        return httpHelper.post(
            '/simpletask/create',
            {
                Token: token,
                Title: "Test Task",
                Description: "Descriptive",
                DueDate: date
            }
        )
        .then(function (response) {
            expect(response.Error).to.be.null;
            expect(response.TaskId).to.be.okay;
        });
    });

    describe("Lookup", function () {
        before(function () {
            var task_id;
            var simpletask = {
                Token : token,
                Title : "Test Task",
                Description : "Descriptive",
                DueDate : date
            };

            // Lookup fetches multiple tasks, create multiple tasks to test
            return commonRequestsHelper.chaiHttpPostPurgeDatabase()
            .then(function (response) {
                return simpleTaskHelper.createTasks(simpletask, 12)
            })
            .then(function (response) {
                expect(response.Error).to.be.null;

                return simpleTaskHelper.lookupTasks(token);
            })
            .then(function (response) {
                expect(response.Error).to.be.null;

                expect(response.SimpleTasks).to.be.an.array;
                expect(response.SimpleTasks).to.have.lengthOf(12);
                expect(response.SimpleTasks[0]).to.have.property('TaskId').that.is.a('string');

                return httpHelper.post(
                    '/simpletask/update',
                    {
                        Token: token,
                        TaskId: response.SimpleTasks[0].TaskId,
                        NewTitle: simpletask.Title,
                        NewDescription: simpletask.Description,
                        NewDueDate: simpletask.DueDate,
                        NewCompleted: true
                    }
                );
            })
            .then(function (response) {
                expect(response.Error).to.be.null;
            });
        });

        it('Fetches unlimited unfinished tasks', function() {
            return httpHelper.post(
                '/simpletask/lookup',
                {
                    Token: token,
                    ShowCompleted: false,
                    Limit: 0
                }
            )
            .then(function (response) {
                expect(response.Error).to.be.null;
                expect(response.SimpleTasks).to.be.an.array;
                expect(response.SimpleTasks).to.have.length.of.at.least(1);
                expect(response.SimpleTasks).to.not.containSubset([{Completed: true}]);
            });
        });

        it('Fetches 10 unfinished tasks', function() {
            return httpHelper.post(
                '/simpletask/lookup',
                {
                    Token: token,
                    ShowCompleted: false,
                    Limit: 10
                }
            )
            .then(function (response) {
                expect(response.Error).to.be.null;
                expect(response.SimpleTasks).to.have.length.of.at.least(1);
                expect(response.SimpleTasks).to.have.length.of.at.most(10);
                expect(response.SimpleTasks).to.not.containSubset([{Completed: true}]);
            });
        });

        it('Fetches unlimited tasks, including completed', function() {
            return httpHelper.post(
                '/simpletask/lookup',
                {
                    Token: token,
                    ShowCompleted: true,
                    Limit: 0
                }
            )
            .then(function (response) {
                expect(response.Error).to.be.null;
                expect(response.SimpleTasks).to.have.length.of.at.least(1);
                expect(response.SimpleTasks).to.containSubset([{Completed: true}]);
            });
        });

        it('Fetches 10 tasks, including completed', function() {
            return httpHelper.post(
                '/simpletask/lookup',
                {
                    Token: token,
                    ShowCompleted: true,
                    Limit: 10
                }
            )
            .then(function (response) {
                expect(response.Error).to.be.null;
                expect(response.SimpleTasks).to.have.length.of.at.least(1);
                expect(response.SimpleTasks).to.have.length.of.at.most(10);
                expect(response.SimpleTasks).to.containSubset([{Completed: true}]);
            });
        });
    });

    describe("Update", function () {
        var simpletask = {};

        before(function () {
            simpletask.title = "Test Task";
            simpletask.description = "Descriptive";
            simpletask.dueDate = date;
            simpletask.completed = false;

            return httpHelper.post(
                '/simpletask/create',
                {
                    Token : token,
                    Title : simpletask.title,
                    Description : simpletask.description,
                    DueDate : simpletask.dueDate,
                }
            )
            .then(function (response) {
                expect(response.Error).to.be.null;
                expect(response.TaskId).to.not.be.null;
                simpletask.taskId = response.TaskId;
            });
        });

        it('Changes title', function() {
            var newTitle = "This is a new title";
            return httpHelper.post(
                '/simpletask/update',
                {
                    Token: token,
                    TaskId: simpletask.taskId,
                    NewTitle: newTitle,
                    NewDescription: simpletask.description,
                    NewDueDate: simpletask.dueDate,
                    NewCompleted: simpletask.completed
                }
            ).then(
                function (response) {
                    expect(response.Error).to.be.null;
                    return httpHelper.post(
                        '/simpletask/lookup',
                        {
                            Token : token,
                            Limit : 0,
                            ShowCompleted : true
                        }
                    );
                }
            ).then(function (response) {
                expect(response.Error).to.be.null;
                expect(response.SimpleTasks).to.be.an.array;
                expect(response.SimpleTasks).to.have.length.of.at.least(1);
                expect(response.SimpleTasks).to.containSubset([{TaskId: simpletask.taskId, Title: newTitle}]);
            });
        });

        it('Changes description', function() {
            var newDescription = "New Description";
            return httpHelper.post(
                '/simpletask/update',
                {
                    Token: token,
                    TaskId: simpletask.taskId,
                    NewTitle: simpletask.title,
                    NewDescription: newDescription,
                    NewDueDate: simpletask.dueDate,
                    NewCompleted: simpletask.completed,
                }
            ).then(
                function (response) {
                    expect(response.Error).to.be.null;
                    return httpHelper.post(
                        '/simpletask/lookup',
                        {
                            Token : token,
                            Limit : 0,
                            ShowCompleted : true
                        }
                    );
                }
            ).then(function (response) {
                expect(response.Error).to.be.null;
                expect(response.SimpleTasks).to.be.an.array;
                expect(response.SimpleTasks).to.have.length.of.at.least(1);
                expect(response.SimpleTasks).to.containSubset([{TaskId: simpletask.taskId, Description: newDescription}]);
            });
        });

        it('Changes duedate', function() {
            var newDate = moment().add(3, 'days').toISOString();

            return httpHelper.post(
                '/simpletask/update',
                {
                    Token: token,
                    TaskId: simpletask.taskId,
                    NewTitle: simpletask.title,
                    NewDescription: simpletask.description,
                    NewDueDate: newDate,
                    NewCompleted: simpletask.completed,
                }
            ).then(function (response) {
                expect(response.Error).to.be.null;
                return httpHelper.post(
                    '/simpletask/lookup',
                    {
                        Token : token,
                        Limit : 0,
                        ShowCompleted : true
                    }
                );
            })
            .then(function (response) {
                expect(response.Error).to.be.null;
                expect(response.SimpleTasks).to.be.an.array;
                expect(response.SimpleTasks).to.have.length.of.at.least(1);
                expect(response.SimpleTasks).to.containSubset([{TaskId: simpletask.taskId, DueDate: newDate}]);
            });
        });

        it('Mark as complete', function() {
            return httpHelper.post(
                '/simpletask/update',
                {
                    Token: token,
                    TaskId: simpletask.taskId,
                    NewTitle: simpletask.title,
                    NewDescription: simpletask.description,
                    NewDueDate: simpletask.dueDate,
                    NewCompleted: true,
                }
            ).then(
                function (response) {
                    expect(response.Error).to.be.null;
                    return httpHelper.post(
                        '/simpletask/lookup',
                        {
                            Token : token,
                            Limit : 0,
                            ShowCompleted : true
                        }
                    );
                }
            ).then(function (response) {
                expect(response.Error).to.be.null;
                expect(response.SimpleTasks).to.be.an.array;
                expect(response.SimpleTasks).to.have.length.of.at.least(1);
                expect(response.SimpleTasks).to.containSubset([{TaskId: simpletask.taskId, Completed: true}]);
            });
        });
    });

    describe('Remove', function() {
        before(function() {
            return commonRequestsHelper.chaiHttpPostPurgeDatabaseArea('simpletask');
        });

        it('Removes a task', function() {
            var taskIdToRemove = null;

            return simpleTaskHelper.createTasks({
                Token : token,
                Title : "Test Task",
                Description : "Descriptive",
                DueDate : date
            }, 2)
            .then(function() {
                return simpleTaskHelper.lookupTasks(token);
            })
            .then(function(response) {
                expect(response.SimpleTasks).to.be.an.array;
                expect(response.SimpleTasks).to.have.lengthOf(2);

                taskIdToRemove = response.SimpleTasks[0].taskId;
                expect(response.SimpleTasks[1].TaskId).to.not.equal(taskIdToRemove);

                return httpHelper.post('/simpletask/remove', {
                    Token: token,
                    TaskId: response.SimpleTasks[0].TaskId
                });
            })
            .then(function(response) {
                expect(response.Error).to.be.null;

                return simpleTaskHelper.lookupTasks(token);
            })
            .then(function(response) {
                expect(response.SimpleTasks).to.be.an.array;
                expect(response.SimpleTasks).to.have.lengthOf(1);
                expect(response.SimpleTasks[0].TaskId).to.not.equal(taskIdToRemove);
            });
        });
    });
});
