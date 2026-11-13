import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart'
    show ExternalLibrary;

ExternalLibrary nativeAssetLibrary() => _NativeAssetLibrary();

class _NativeAssetLibrary implements ExternalLibrary {
  @override
  final ffi.DynamicLibrary ffiDynamicLibrary = ffi.DynamicLibrary.codeAsset(
    'package:bbqr/src/rust/frb_generated.io.dart',
  );

  @override
  final String debugInfo = 'Native Assets: bbqr';
}
