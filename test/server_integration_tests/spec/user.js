if (!global.Promise) {
    global.Promise = require('bluebird');
}

var expect = require('chai').expect;

var httpHelper = require('../helpers/chai-http-request-helper.js');

describe('User', function() {
    before(function () {
        return httpHelper.chaiHttpPostPurgeDatabase();
    });

    describe('Create user', function() {
        beforeEach(function () {
            return httpHelper.chaiHttpPostPurgeDatabaseArea('user');
        });

        it('Creates a new user', function() {
            var payload = {
                UserName: "Theta",
                EmailAddress: "ts@evelyn.com",
                Password: "asdf"
            };

            return httpHelper.chaiHttpPost(
                '/user/create',
                payload
            )
            .then(function (response) {
                expect(response.Error).to.be.null;
            });
        });

        it('Refuses to create a user if the user already exists', function() {
            var payload = {
                UserName: "Exist",
                EmailAddress: "iexist@evelyn.com",
                Password: "asdf"
            };

            return httpHelper.chaiHttpPost(
                '/user/create',
                payload
            )
            .then(function (response) {
                expect(response.Error).to.be.null;

                return httpHelper.chaiHttpPost(
                    '/user/create',
                    payload
                );
            })
            .then(function (response) {
                expect(response.Error).to.not.be.null;
                expect(response.Error.ErrorCode).to.equal("100202");
            });
        });
    });

    describe('Logon', function() {
        var createUserPayload = {
            UserName: "IAmCorrect",
            EmailAddress: "iamcorrect@evelyn.com",
            Password: "asdf"
        };

        beforeEach(function () {
            return httpHelper.chaiHttpPostPurgeDatabaseArea('user');
        });

        it('Rejects logon with incorrect email', function() {
            return httpHelper.chaiHttpPost(
                '/user/create',
                createUserPayload
            )
            .then(function (response) {
                expect(response.Error).to.be.null;

                return httpHelper.chaiHttpPost(
                    '/user/logon',
                    {
                        EmailAddress: "iamnotcorrect@evelyn.com",
                        Password: "asdf"
                    }
                );
            })
            .then(function (response) {
                expect(response.Error).to.not.be.null;
                expect(response.Error.ErrorCode).to.equal("100203");
            });
        });

        it('Rejects logon with incorrect password', function() {
            return httpHelper.chaiHttpPost(
                '/user/create',
                createUserPayload
            )
            .then(function (response) {
                expect(response.Error).to.be.null;

                return httpHelper.chaiHttpPost(
                    '/user/logon',
                    {
                        EmailAddress: "iamcorrect@evelyn.com",
                        Password: "wrongpassword"
                    }
                );
            })
            .then(function (response) {
                expect(response.Error).to.not.be.null;
                expect(response.Error.ErrorCode).to.equal("100203");
            });
        });

        it('Accepts correct logon and gives back a session token', function() {
            return httpHelper.chaiHttpPost(
                '/user/create',
                createUserPayload
            )
            .then(function (response) {
                expect(response.Error).to.be.null;

                return httpHelper.chaiHttpPost(
                    '/user/logon',
                    {
                        EmailAddress: "iamcorrect@evelyn.com",
                        Password: "asdf"
                    }
                );
            })
            .then(function (response) {
                expect(response.Error).to.be.null;
                expect(response.Token).to.be.ok;
            });
        });
    });
});
