# FRB bindings

From the package root, use Flutter 3.50.0 and FRB codegen 2.13.0, then run `tool/generate_bindings.sh`. The script runs codegen and restores the package-owned default Native Assets loader in `lib/src/rust/frb_generated.dart`. The postprocessor rejects unknown codegen output or a changed version. Run `flutter test test/native_asset_loader_test.dart` afterwards. Commit regenerated bridge output separately from the loader and test changes.
