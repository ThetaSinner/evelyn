if (!global.Promise) {
  global.Promise = require('bluebird');
}

var chai = require('chai');
var chaiHttp = require('chai-http');
var _ = require('lodash');

var httpErrorHelper = require('./chai-http-error-helper.js');

chai.use(chaiHttp);
var expect = chai.expect;

function chaiHttpPost(action, payload, onSuccess) {
  // For some reason .send() seems to sometimes send an empty payload
  // if you give it an object, which is documented to be allowed...
  if (_.isObject(payload)) {
    payload = JSON.stringify(payload);
  }

  // console.log("Will send", action, payload);

  return chai.request('localhost:8080')
  .post(action)
  .send(payload)
  .then(function (res) {
    expect(res).to.have.status(200);
    expect(res).to.be.json;
    return onSuccess(res.body);
  })
  .catch(function (err) {
    throw httpErrorHelper.wrapChaiHttpError(err);
  });
}

function chaiHttpPostPurgeDatabase() {
  return chaiHttpPost(
    '/purge',
    {
      Token: 'a temporary token',
      TargetType: 'database',
      Target: ''
    },
    function (response) {
      if (_.isObject(response.Error)) {
        console.log('Purge database error', response.Error.ErrorCode, response.Error.ErrorMessage);
      }

      expect(response.Error).to.be.null;
    }
  );
}

function chaiHttpPostPurgeDatabaseArea(target) {
  return chaiHttpPost(
    '/purge',
    {
      Token: 'a temporary token',
      TargetType: 'database_area',
      Target: target
    },
    function (response) {
      if (_.isObject(response.Error)) {
        console.log('Purge database area error', response.Error.ErrorCode, response.Error.ErrorMessage);
      }

      expect(response.Error).to.be.null;
    }
  );
}

function createUserAndLogon() {
  return new Promise(function (resolve, reject) {
    chaiHttpPost(
      '/user/create',
      {
        UserName: "Theta",
        EmailAddress: "ts@evelyn.com",
        Password: "asdf"
      },
      function (response) {
        expect(response.Error).to.be.null;

        chaiHttpPost(
          '/user/logon',
          {
            EmailAddress: 'ts@evelyn.com',
            Password: 'asdf'
          },
          function (response) {
            expect(response.Error).to.be.null;
            resolve(response.Token);
          }
        );
      }
    );
  });
}

module.exports = {
  chaiHttpPost: chaiHttpPost,
  chaiHttpPostPurgeDatabase: chaiHttpPostPurgeDatabase,
  chaiHttpPostPurgeDatabaseArea: chaiHttpPostPurgeDatabaseArea,
  createUserAndLogon: createUserAndLogon
};
