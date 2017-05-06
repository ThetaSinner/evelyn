if (!global.Promise) {
  global.Promise = require('bluebird');
}

var chai = require('chai');
var expect = chai.expect;
var httpHelper = require('../helpers/chai-http-request-helper.js');
var moment = require('moment');
var chaiSubset = require('chai-subset');
chai.use(chaiSubset);

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

  it('Create', function() {
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
    });
  });

  // TODO asserts for the tests below aren't checking enough.
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
      // TODO write recursive function to generate this
      return httpHelper.chaiHttpPost('/simpletask/create', simpletask)
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.TaskId).to.not.be.null;
        task_id = response.TaskId;
        return httpHelper.chaiHttpPost('/simpletask/create', simpletask);
      })
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.TaskId).to.not.be.null;
        return httpHelper.chaiHttpPost('/simpletask/create', simpletask);
      })
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.TaskId).to.not.be.null;
        return httpHelper.chaiHttpPost('/simpletask/create', simpletask);
      })
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.TaskId).to.not.be.null;
        return httpHelper.chaiHttpPost('/simpletask/create', simpletask);
      })
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.TaskId).to.not.be.null;
        return httpHelper.chaiHttpPost('/simpletask/create', simpletask);
      })
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.TaskId).to.not.be.null;
        return httpHelper.chaiHttpPost('/simpletask/create', simpletask);
      })
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.TaskId).to.not.be.null;
        return httpHelper.chaiHttpPost('/simpletask/create', simpletask);
      })
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.TaskId).to.not.be.null;
        return httpHelper.chaiHttpPost('/simpletask/create', simpletask);
      })
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.TaskId).to.not.be.null;
        return httpHelper.chaiHttpPost('/simpletask/create', simpletask);
      })
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.TaskId).to.not.be.null;
        return httpHelper.chaiHttpPost('/simpletask/create', simpletask);
      })
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.TaskId).to.not.be.null;
        return httpHelper.chaiHttpPost('/simpletask/create', simpletask);
      })
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.TaskId).to.not.be.null;
        return httpHelper.chaiHttpPost(
          '/simpletask/update',
          {
            Token: token,
            TaskId: task_id,
            NewTitle: simpletask.Title,
            NewDescription: simpletask.Description,
            NewDueDate: simpletask.DueDate,
            NewCompleted: true
          }
        )})
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
          expect(response.SimpleTasks).to.not.containSubset([{completed: true}]);
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
          expect(response.SimpleTasks).to.not.containSubset([{completed: true}]);
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
          expect(response.SimpleTasks).to.containSubset([{completed: true}]);
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
          expect(response.SimpleTasks).to.containSubset([{completed: true}]);
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
          expect(response.SimpleTasks).to.containSubset([{taskId: simpletask.taskId, title: newTitle}]);
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
          expect(response.SimpleTasks).to.containSubset([{taskId: simpletask.taskId, description: newDescription}]);
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
          // TODO Dates don't seem to update
          expect(response.SimpleTasks).to.containSubset([{taskId: simpletask.taskId, dueDate: newDate}]);
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
          expect(response.SimpleTasks).to.containSubset([{taskId: simpletask.taskId, completed: true}]);
        });
      });
    });
  });
