#!/bin/bash

# Note iberty, which used to be in binutils must be installed for --verify to work.
# Without verify kcov just SIGSEGVs
apt-get install -y binutils-dev libcurl4-openssl-dev libelf-dev libdw-dev cmake gcc pkg-config libiberty-dev

wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
tar xzf master.tar.gz
cd kcov-master
mkdir build
cd build
cmake ..
make
sudo make install

cd ../..
rm master.tar.gz
rm -rf kcov-master
