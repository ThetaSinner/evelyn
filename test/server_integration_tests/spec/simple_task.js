if (!global.Promise) {
    global.Promise = require('bluebird');
}

var chai = require('chai');
var expect = chai.expect;
var moment = require('moment');
var chaiSubset = require('chai-subset');

var httpHelper = require('../helpers/chai-http-request-helper.js');

chai.use(chaiSubset);

function createTasks(starter_task, number_to_create) {
    return httpHelper.chaiHttpPost(
        '/simpletask/create',
        starter_task
    ).then(function (response) {
        expect(response.Error).to.be.null;

        if (number_to_create <= 1) {
            return Promise.resolve(response);
        }
        else {
            return createTasks(starter_task, number_to_create - 1);
        }
    });
}

function lookupTasks(token) {
    return httpHelper.chaiHttpPost(
        '/simpletask/lookup',
        {
            Token: token,
            Limit: 0,
            ShowCompleted: false
        }
    );
}

describe('Simple Task', function() {
    var date = new Date ().toISOString();
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

    it('Creates a task', function() {
        return httpHelper.chaiHttpPost(
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
            return httpHelper.chaiHttpPostPurgeDatabase()
            .then(function (response) {
                return createTasks(simpletask, 12)
            })
            .then(function (response) {
                expect(response.Error).to.be.null;

                return lookupTasks(token);
            })
            .then(function (response) {
                expect(response.Error).to.be.null;

                expect(response.SimpleTasks).to.be.an.array;
                expect(response.SimpleTasks).to.have.lengthOf(12);
                expect(response.SimpleTasks[0]).to.have.property('TaskId').that.is.a('string');

                return httpHelper.chaiHttpPost(
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
            return httpHelper.chaiHttpPost(
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
            return httpHelper.chaiHttpPost(
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
            return httpHelper.chaiHttpPost(
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
            return httpHelper.chaiHttpPost(
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

            return httpHelper.chaiHttpPost(
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
            return httpHelper.chaiHttpPost(
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
                    return httpHelper.chaiHttpPost(
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
            return httpHelper.chaiHttpPost(
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
                    return httpHelper.chaiHttpPost(
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

            return httpHelper.chaiHttpPost(
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
                return httpHelper.chaiHttpPost(
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
            return httpHelper.chaiHttpPost(
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
                    return httpHelper.chaiHttpPost(
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
            return httpHelper.chaiHttpPostPurgeDatabaseArea('simpletask');
        });

        it('removes a task', function() {
            var taskIdToRemove = null;

            return createTasks({
                Token : token,
                Title : "Test Task",
                Description : "Descriptive",
                DueDate : date
            }, 2)
            .then(function() {
                return lookupTasks(token);
            })
            .then(function(response) {
                expect(response.SimpleTasks).to.be.an.array;
                expect(response.SimpleTasks).to.have.lengthOf(2);

                taskIdToRemove = response.SimpleTasks[0].taskId;
                expect(response.SimpleTasks[1].TaskId).to.not.equal(taskIdToRemove);

                return httpHelper.chaiHttpPost('/simpletask/remove', {
                    Token: token,
                    TaskId: response.SimpleTasks[0].TaskId
                });
            })
            .then(function(response) {
                expect(response.Error).to.be.null;

                return lookupTasks(token);
            })
            .then(function(response) {
                expect(response.SimpleTasks).to.be.an.array;
                expect(response.SimpleTasks).to.have.lengthOf(1);
                expect(response.SimpleTasks[0].TaskId).to.not.equal(taskIdToRemove);
            });
        });
    });
});
