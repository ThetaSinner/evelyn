describe('Simple Task', function() {
  var date = new Date ().toISOString();

  it('Create', function() {
    return chai.request('http://localhost:8080')
    .post('/simpletask/create')
    .send({
      Token: Token,
      Title: "Test Task",
      Description: "Descriptive",
      DueDate: date
    })
    .then(function (res) {
      var obj = JSON.parse(res.text);
      expect(obj.Error).to.equal(null);
    })
    .catch(function (err) {
      throw Error(err.actual.ErrorMessage);
    })
  });

  describe("Lookup", function () {
    it('Lookup - No Limit, No Completed', function() {
      return chai.request('http://localhost:8080')
      .post('/simpletask/lookup')
      .send({
        ShowCompleted: false,
        Limit: 0,
        Token: Token
      })
      .then(function (res) {
        var obj = JSON.parse(res.text);
        expect(obj.Error).to.equal(null);
      })
      .catch(function (err) {
        throw Error(err.actual.ErrorMessage);
      })
    });

    it('Lookup - Limit, No Completed', function() {
      return chai.request('http://localhost:8080')
      .post('/simpletask/lookup')
      .send({
        ShowCompleted: false,
        Limit: 10,
        Token: Token
      })
      .then(function (res) {
        var obj = JSON.parse(res.text);
        expect(obj.Error).to.equal(null);
      })
      .catch(function (err) {
        throw Error(err.actual.ErrorMessage);
      })
    });

    it('Lookup - No Limit, Inc Completed', function() {
      return chai.request('http://localhost:8080')
      .post('/simpletask/lookup')
      .send({
        ShowCompleted: true,
        Limit: 0,
        Token: Token
      })
      .then(function (res) {
        var obj = JSON.parse(res.text);
        expect(obj.Error).to.equal(null);
      })
      .catch(function (err) {
        throw Error(err.actual.ErrorMessage);
      })
    });

    it('Lookup - Limit, Inc Completed', function() {
      return chai.request('http://localhost:8080')
      .post('/simpletask/lookup')
      .send({
        ShowCompleted: true,
        Limit: 10,
        Token: Token
      })
      .then(function (res) {
        var obj = JSON.parse(res.text);
        expect(obj.Error).to.equal(null);
      })
      .catch(function (err) {
        throw Error(err.actual.ErrorMessage);
      })
    });
  });

});
