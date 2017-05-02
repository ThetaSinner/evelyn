describe('User', function() {
  describe('Create', function() {
    it('Create', function() {
      return chai.request('http://localhost:8080')
      .post('/user/create')
      .send({
        UserName: "Theta",
        EmailAddress: "ts@evelyn.com",
        Password: "asdf"
      })
      .then(function (res) {
        var obj = JSON.parse(res.text);
        expect(obj.Error).to.equal(null);
      })
      .catch(function (err) {
        throw Error(err.actual.ErrorMessage);
      })
    });

    it('Already Exists', function() {
      return chai.request('http://localhost:8080')
      .post('/user/create')
      .send({
        UserName: "Theta",
        EmailAddress: "ts@evelyn.com",
        Password: "asdf"
      })
      .then(function (res) {
        var obj = JSON.parse(res.text);
        expect(obj.Error).to.not.equal(null);
      })
      .catch(function (err) {
        throw Error(err.actual.ErrorMessage);
      })
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
