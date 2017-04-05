#!/bin/sh
echo "Changing working directory"
cd $TRAVIS_BUILD_DIR/backend
echo "Running tests"
sh ./scripts/run_tests.sh