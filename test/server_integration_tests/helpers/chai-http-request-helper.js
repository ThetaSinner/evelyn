function chaiHttpPost(action, payload, onSuccess) {
  return chai.request('http://localhost:8080')
  .post(action)
  .send(payload)
  .then(onSuccess)
  .catch(function (err) {
    throw wrapChaiHttpError(err);
  });
}
