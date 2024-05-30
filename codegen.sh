#!/bin/bash
cd rust
cargo update
cd - || exit 1

# export CPATH="/opt/homebrew/opt/llvm/include"
flutter_rust_bridge_codegen generate
