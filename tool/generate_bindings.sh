#!/bin/sh
set -eu
cd "$(dirname "$0")/.."
flutter_rust_bridge_codegen generate
python3 tool/patch_native_asset_loader.py
