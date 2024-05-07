#!/bin/bash
VERSION=$1
mkdir -p target/unittest.libbbqr.$VERSION
cargo build --release
OS=$(uname -s)
if [ "$OS" = "Linux" ]; then
    cp target/release/libbbqr.so target/unittest.libbbqr.$VERSION
    cp target/release/libbbqr.so ../build/unit_test_assets/lib
elif [ "$OS" = "Darwin" ]; then
    cp target/release/libbbqr.dylib target/unittest.libbbqr.$VERSION
else
    echo "Unsupported OS: $OS"
    exit 1
fi
