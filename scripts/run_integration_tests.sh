#!/bin/bash

cargo build
./target/debug/evelyn >/dev/null &
server_pid=$!

cargo test

kill -15 $server_pid
