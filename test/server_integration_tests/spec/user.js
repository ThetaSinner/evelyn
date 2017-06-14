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

    describe('Search for users', function() {
        beforeEach(function () {
            return httpHelper.chaiHttpPostPurgeDatabaseArea('user');
        });

        it('Searches for a user', function() {
            return httpHelper.createUserAndLogon('user1')
            .then(function(token) {
                return httpHelper.searchForUsers(token, 'er');
            })
            .then(function(response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);

                var result = response.SearchResults[0];

                expect(result.UserName).to.equal('user1');
                expect(result.UserId).to.be.ok;
            });
        });

        it('Searches for a matching user when there are multiple users', function() {
            var token = null;

            return httpHelper.createUserAndLogon('user1')
            .then(function(_token) {
                token = _token;
                return httpHelper.createUserAndLogon('jimmy');
            })
            .then(function() {
                return httpHelper.searchForUsers(token, 'er');
            })
            .then(function(response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);

                var result = response.SearchResults[0];

                expect(result.UserName).to.equal('user1');
                expect(result.UserId).to.be.ok;
            })
            .then(function() {
                return httpHelper.searchForUsers(token, 'mm');
            })
            .then(function(response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(1);

                var result = response.SearchResults[0];

                expect(result.UserName).to.equal('jimmy');
                expect(result.UserId).to.be.ok;
            });
        });

        it('Searches and finds multiple users', function() {
            var token = null;

            return httpHelper.createUserAndLogon('user1')
            .then(function(_token) {
                token = _token;
                return httpHelper.createUserAndLogon('user2');
            })
            .then(function() {
                return httpHelper.createUserAndLogon('jimmy');
            })
            .then(function() {
                return httpHelper.searchForUsers(token, 'er');
            })
            .then(function(response) {
                expect(response.SearchResults).to.be.an.array;
                expect(response.SearchResults).to.have.lengthOf(2);

                var result1 = response.SearchResults[0];

                expect(result1.UserName).to.equal('user1');
                expect(result1.UserId).to.be.ok;

                var result2 = response.SearchResults[1];

                expect(result2.UserName).to.equal('user2');
                expect(result2.UserId).to.be.ok;
            });
        });
    });
});
