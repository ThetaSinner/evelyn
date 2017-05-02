describe('create user', function () {
  it('should create a user', function () {
    return chai.request('http://localhost:8080')
      .post('/user/create')
      .send({
        UserName: "Theta",
        EmailAddress: "ts@evelyn.com",
        Password: "asdf"
      })
      .then(function (response) {
        expect(response).to.have.status(200);

        expect(response.text).to.equal("{\"Error\":null}");
      })
      .catch(function (err) {
        throw err;
      });
  });
});
