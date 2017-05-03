describe('User', function() {
  describe('Create user', function() {
    it('Creates a new user', function() {
      return chaiHttpPost(
        '/user/create',
        {
          UserName: "Theta",
          EmailAddress: "ts@evelyn.com",
          Password: "asdf"
        },
        function (res) {
          var obj = JSON.parse(res.text);
          expect(obj.Error).to.be.null;
        }
      );
    });

    it('Refuses to create a user if the user already exists', function() {
      var payload = {
        UserName: "Exist",
        EmailAddress: "iexist@evelyn.com",
        Password: "asdf"
      };

      return chaiHttpPost(
        '/user/create',
        payload,
        function (res) {
          var obj = JSON.parse(res.text);
          expect(obj.Error).to.be.null;

          return chaiHttpPost(
            '/user/create',
            payload,
            function (res) {
              var obj = JSON.parse(res.text);
              expect(obj.Error).to.not.be.null;
              expect(obj.Error.ErrorCode).to.equal("100202");
          });
        });
    });
  });

  describe('Logon', function() {
    it('Logon Incorrect', function() {
      return chai.request('http://localhost:8080')
      .post('/user/logon')
      .send({
        EmailAddress: "ts@evelyn.com",
        Password: "thisisthewrongpassword"
      })
      .then(function (res) {
        var obj = JSON.parse(res.text);
        expect(obj.Error).to.not.equal(null);
      })
      .catch(function (err) {
        throw Error(err.actual.ErrorMessage);
      })
    });

    it('Logon Correct', function() {
      return chai.request('http://localhost:8080')
      .post('/user/logon')
      .send({
        EmailAddress: "ts@evelyn.com",
        Password: "asdf"
      })
      .then(function (res) {
        var obj = JSON.parse(res.text);
        expect(obj.Error).to.equal(null);
        expect(obj.Token).to.not.equal(null);
        Token = obj.Token;
      })
      .catch(function (err) {
        throw Error(err.actual.ErrorMessage);
      })
    });
  });
});
