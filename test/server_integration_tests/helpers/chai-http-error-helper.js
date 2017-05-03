function wrapChaiHttpError(err) {
  var message = "";
  if (!_.isUndefined(err.message)) {
    message += err.message + "\r\n";
  }

  if (_.isObject(err.actual)) {
    message += JSON.stringify(err.actual) + "\r\n";
  }

  return Error(message);
}
