describe('Todo List', function() {

  var list_id = null;

  it('Create', function() {
    return chai.request('http://localhost:8080')
    .post('/todolist/create')
    .send({
      Token: Token,
      Title: "Test Todo",
    })
    .then(function (res) {
      var obj = JSON.parse(res.text);
      expect(obj.Error).to.equal(null);
      expect(obj.TodoListId).to.not.equal(null);
      list_id = obj.TodoListId;
    })
    .catch(function (err) {
      throw Error(err.actual.ErrorMessage);
    })
  });

  it('Add List Item', function() {
    return chai.request('http://localhost:8080')
    .post('/todolist/additem')
    .send({
      Token: Token,
      TodoListId: list_id,
      TodoListItem: {
        Text: "Eggs",
        IsDone: false
      }
    })
    .then(function (res) {
      var obj = JSON.parse(res.text);
      expect(obj.Error).to.equal(null);
    })
    .catch(function (err) {
      throw Error(err.actual.ErrorMessage);
    })
  });

  it('Update List Item', function() {
    return chai.request('http://localhost:8080')
    .post('/todolist/updateitem')
    .send({
      Token: Token,
      TodoListId: list_id,
      ItemIndex: 0,
      IsDone: true
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
    it('Lookup Single', function() {
      return chai.request('http://localhost:8080')
      .post('/todolist/lookuplist')
      .send({
        Token: Token,
        TodoListId: list_id
      })
      .then(function (res) {
        var obj = JSON.parse(res.text);
        expect(obj.Error).to.equal(null);
      })
      .catch(function (err) {
        throw Error(err.actual.ErrorMessage);
      })
    });

    it('Lookup Multiple', function() {
      return chai.request('http://localhost:8080')
      .post('/todolist/lookuplists')
      .send({
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
