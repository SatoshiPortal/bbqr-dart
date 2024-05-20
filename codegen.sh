#!/bin/bash
cd rust
cargo update
cd - || exit 1

# export CPATH="$(clang -v 2>&1 | grep "Selected GCC installation" | rev | cut -d' ' -f1 | rev)/include"
# export CPATH="/opt/homebrew/opt/llvm/include"
flutter_rust_bridge_codegen generate
