#!/bin/bash

cargo build

# If cargo build fails then just exit, don't try to run tests.
if [ $? -ne 0 ]; then
  echo "Failed to build"
  exit 1
fi

# Start an Evelyn server instance and keep its process id.
./target/debug/evelyn >/dev/null &
server_pid=$!

cargo test

# Keep the exit code from cargo test.
test_exit_code=0
if [ $? -ne 0 ]; then
  test_exit_code=$?
fi

# Kill the Evelyn server.
kill -15 $server_pid

# Use the exit code from cargo test
exit $test_exit_code
