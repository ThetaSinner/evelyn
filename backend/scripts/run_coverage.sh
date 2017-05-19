#!/bin/bash

cargo build -j8
./target/debug/evelyn >/dev/null &
server_pid=$!

cargo test --no-run -j8
for file in target/debug/*test*; do mkdir -p "target/cov/$(basename $file)"; kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"; done;

echo "Killing server"
kill -15 $server_pid
