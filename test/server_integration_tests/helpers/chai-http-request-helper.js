if (!global.Promise) {
    global.Promise = require('bluebird');
}

var chai = require('chai');
var chaiHttp = require('chai-http');
var _ = require('lodash');
var MongoClient = require('mongodb').MongoClient;

var httpErrorHelper = require('./chai-http-error-helper.js');

chai.use(chaiHttp);
var expect = chai.expect;

function hang(ms, forward) {
    return new Promise(function (resolve, reject) {
        setTimeout(function () {
            resolve(forward);
        }, ms);
    });
}

function chaiHttpPost(action, payload) {
    // For some reason .send() seems to sometimes send an empty payload
    // if you give it an object, which is documented to be allowed...
    if (_.isObject(payload)) {
        payload = JSON.stringify(payload);
    }

    // console.log("Will send", action, payload);

    return new Promise(function (resolve, reject) {
        return chai.request('localhost:8080')
        .post(action)
        .send(payload)
        .then(function (res) {
            expect(res).to.have.status(200);
            expect(res).to.be.json;
            resolve(res.body);
        })
        .catch(function (err) {
            reject(httpErrorHelper.wrapChaiHttpError(err));
        });
    });
}

function chaiHttpPostPurgeDatabase() {
    return new Promise(function (resolve, reject) {
        chaiHttpPost(
            '/purge',
            {
                Token: 'a temporary token',
                TargetType: 'database',
                Target: ''
            }
        )
        .then(function (response) {
            if (_.isObject(response.Error)) {
                console.log('Purge database error', response.Error.ErrorCode, response.Error.ErrorMessage);
            }

            expect(response.Error).to.be.null;
            resolve();
        }).catch(function (e) {
            reject(e);
        });
    });
}

function chaiHttpPostPurgeDatabaseArea(target) {
    return new Promise(function (resolve, reject) {
        chaiHttpPost(
            '/purge',
            {
                Token: 'a temporary token',
                TargetType: 'database_area',
                Target: target
            }
        )
        .then(function (response) {
            if (_.isObject(response.Error)) {
                console.log('Purge database area error', response.Error.ErrorCode, response.Error.ErrorMessage);
            }

            expect(response.Error).to.be.null;
            resolve();
        }).catch(function (e) {
            reject(e);
        })
    });
}

function createUserAndLogon() {
    return chaiHttpPost(
        '/user/create',
        {
            UserName: "Theta",
            EmailAddress: "ts@evelyn.com",
            Password: "asdf"
        }
    )
    .then(function (response) {
        expect(response.Error).to.be.null;

        return chaiHttpPost(
            '/user/logon',
            {
                EmailAddress: 'ts@evelyn.com',
                Password: 'asdf'
            }
        );
    })
    .then(function (response) {
        expect(response.Error).to.be.null;
        return Promise.resolve(response.Token);
    });
}

module.exports = {
    chaiHttpPost: chaiHttpPost,
    chaiHttpPostPurgeDatabase: chaiHttpPostPurgeDatabase,
    chaiHttpPostPurgeDatabaseArea: chaiHttpPostPurgeDatabaseArea,
    createUserAndLogon: createUserAndLogon
};
