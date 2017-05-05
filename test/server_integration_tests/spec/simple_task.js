if (!global.Promise) {
  global.Promise = require('bluebird');
}

var expect = require('chai').expect;

var httpHelper = require('../helpers/chai-http-request-helper.js');

describe('Simple Task', function() {
  var date = new Date ().toISOString();
  var token = null;

  before(function () {
    return httpHelper.chaiHttpPostPurgeDatabase().then(function () {
        return httpHelper.createUserAndLogon().then(function (_token) {
            token = _token;
          });
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
      },
      function (response) {
        expect(response.Error).to.be.null;
      }
    );
  });

  // TODO asserts for the tests below aren't checking enough.

  describe("Lookup", function () {
    it('Lookup - No Limit, No Completed', function() {
      return httpHelper.chaiHttpPost(
        '/simpletask/lookup',
        {
          Token: token,
          ShowCompleted: false,
          Limit: 0
        },
        function (response) {
          expect(response.Error).to.be.null;
        }
      );
    });

    it('Lookup - Limit, No Completed', function() {
      return httpHelper.chaiHttpPost(
        '/simpletask/lookup',
        {
          Token: token,
          ShowCompleted: false,
          Limit: 10
        },
        function (response) {
          expect(response.Error).to.be.null;
        }
      );
    });

    it('Lookup - No Limit, Inc Completed', function() {
      return httpHelper.chaiHttpPost(
        '/simpletask/lookup',
        {
          Token: token,
          ShowCompleted: true,
          Limit: 0
        },
        function (response) {
          expect(response.Error).to.be.null;
        }
      );
    });

    it('Lookup - Limit, Inc Completed', function() {
      return httpHelper.chaiHttpPost(
        '/simpletask/lookup',
        {
          Token: token,
          ShowCompleted: true,
          Limit: 10
        },
        function (response) {
          expect(response.Error).to.be.null;
        }
      );
    });
  });
});
