#!/bin/sh
set -e

# Download llvm.tar.gz
wget http://releases.llvm.org/3.7.1/clang+llvm-3.7.1-x86_64-linux-gnu-ubuntu-14.04.tar.xz -O llvm.tar.xz
# Extract files (no `v` arg to reduce web client spam)
tar xfJ llvm.tar.xz
# Move llvm files over
mv -v clang+llvm-3.7.1-x86_64-linux-gnu-ubuntu-14.04 $HOME/clang
cp -v $HOME/clang/bin/* $HOME/bin
