#!/bin/bash

cargo build -j 8
cd ..
cp -r source source2
cd source
./target/debug/evelyn >/dev/null &
server_pid=$!

cd ../source2
cargo test  -j 8

kill -15 $server_pid
