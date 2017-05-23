if (!global.Promise) {
    global.Promise = require('bluebird');
}

var expect = require('chai').expect;

var httpHelper = require('../helpers/chai-http-request-helper.js');

describe('Calendar', function() {
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

    it('Add Event', function() {
        return httpHelper.chaiHttpPost(
            '/calendar/addevent',
            {
                Token: token,
                Title: "The Great Testing Event",
                EventBegin: date,
                EventEnd: date
            }
        )
        .then(function (response) {
            expect(response.Error).to.be.null;
        });
    });

});
