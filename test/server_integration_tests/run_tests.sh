#!/bin/bash

# Ensure the server build is up-to-date.
cd ../../backend
cargo build

# Start the server.
./target/debug/evelyn -conf=../test/server_integration_tests/evelyn_test_conf.json &
server_pid=$!

cd ../test/server_integration_tests

# Install test dependencies.
npm install

# Run the tests.
./node_modules/.bin/mocha spec/ --opts mocha.opts
exit_code=$?

# Stop the server.
kill -15 $server_pid

# Propogate the exit code from the test result.
exit $exit_code
