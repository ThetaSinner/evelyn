#!/bin/bash

# Ensure the server build is up-to-date.
cd ../../backend
cargo build
cd ../test/server_integration_tests

# Start the server.
../../backend/target/debug/evelyn > /dev/null &
server_pid=$!

# Install test dependencies.
npm install

# Run the tests.
./node_modules/.bin/mocha-phantomjs ./index.html
exit_code=$?

# Stop the server.
kill -15 $server_pid

# Propogate the exit code from the test result.
exit $exit_code
