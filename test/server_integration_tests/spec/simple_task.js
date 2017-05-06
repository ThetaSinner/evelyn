if (!global.Promise) {
  global.Promise = require('bluebird');
}

var expect = require('chai').expect;

var httpHelper = require('../helpers/chai-http-request-helper.js');

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
      });
    });
  });

  describe("Update", function () {
    var simpletask;

    before(function () {
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
        return httpHelper.chaiHttpPost(
          '/simpletask/lookup',
          {
            Token: token,
            ShowCompleted: false,
            Limit: 10
          }
        )
      })
      .then(function (response) {
        expect(response.Error).to.be.null;
        expect(response.SimpleTasks).to.not.be.null;
        simpletask = response.SimpleTasks[0];

        expect(simpletask.taskId).to.not.be.null;
        expect(simpletask.title).to.not.be.null;
        expect(simpletask.description).to.not.be.null;
        expect(simpletask.dueDate).to.not.be.null;
        expect(simpletask.completed).to.not.be.null;
      });
    });

    it('Changes title', function() {
      return httpHelper.chaiHttpPost(
        '/simpletask/update',
        {
          Token: token,
          TaskId: simpletask.taskId,
          NewTitle: "This is a new title",
          NewDescription: simpletask.fescription,
          NewDueDate: simpletask.dueDate,
          NewCompleted: true,
        }
      ).then(
        function (response) {
          expect(response.Error).to.be.null;
        }
      );
    });

    it('Changes description', function() {
      return httpHelper.chaiHttpPost(
        '/simpletask/update',
        {
          Token: token,
          TaskId: simpletask.taskId,
          NewTitle: simpletask.title,
          NewDescription: "Describe this ...",
          NewDueDate: simpletask.dueDate,
          NewCompleted: simpletask.completed,
        }
      ).then(
        function (response) {
          expect(response.Error).to.be.null;
        }
      );
    });

    it('Changes duedate', function() {
      var newDate = new Date();
      newDate.setDate(newDate.getDate() + 1);

      return httpHelper.chaiHttpPost(
        '/simpletask/update',
        {
          Token: token,
          TaskId: simpletask.taskId,
          NewTitle: simpletask.title,
          NewDescription: simpletask.fescription,
          NewDueDate: newDate.toISOString(),
          NewCompleted: simpletask.completed,
        }
      ).then(
        function (response) {
          expect(response.Error).to.be.null;
        }
      );
    });

    it('Mark as complete', function() {
      return httpHelper.chaiHttpPost(
        '/simpletask/update',
        {
          Token: token,
          TaskId: simpletask.taskId,
          NewTitle: simpletask.title,
          NewDescription: simpletask.fescription,
          NewDueDate: simpletask.dueDate,
          NewCompleted: true,
        }
      ).then(
        function (response) {
          expect(response.Error).to.be.null;
        }
      );
    });
  });
});
