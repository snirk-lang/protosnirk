#!/bin/sh
set -e

mkdir -p $HOME/bin
# Install cmake from cmake.org downloads
wget https://cmake.org/files/v2.8/cmake-2.8.8-Linux-i386.tar.gz -O cmake.tar.gz --no-check-certificate
# Extract cmake files
tar xzf cmake.tar.gz # Using -v is too much for web client watching builds
# Move to a better dir
mv -v cmake-2.8.8-Linux-i386/bin/* $HOME/bin
