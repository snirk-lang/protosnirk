#!/bin/sh
set -e

if [ -d $HOME/llvm-4.0.0 ]
    echo "LLVM already installed (cache)";
    exit;
fi

# Download llvm.tar.gz from llvm-sys's bitbucket
wget https://bitbucket.org/tari/llvm-sys.rs/downloads/llvm-4.0.0.linux.tar.xz -O llvm-4.0.0.tar.xz
# Extract files (no `v` arg to reduce web client spam)
tar xfJ llvm-4.0.0.tar.xz
# Move llvm files over
rm -v llvm-4.0.0.tar.xz
mv -v llvm-4.0.0 $HOME
