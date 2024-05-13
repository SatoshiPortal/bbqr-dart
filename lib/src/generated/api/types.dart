// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.33.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'types.freezed.dart';

// The type `Split` is not used by any `pub` functions, thus it is ignored.

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<_Split>>
@sealed
class Split extends RustOpaque {
  Split.dcoDecode(List<dynamic> wire) : super.dcoDecode(wire, _kStaticData);

  Split.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        BbqrCore.instance.api.rust_arc_increment_strong_count_Split,
    rustArcDecrementStrongCount:
        BbqrCore.instance.api.rust_arc_decrement_strong_count_Split,
    rustArcDecrementStrongCountPtr:
        BbqrCore.instance.api.rust_arc_decrement_strong_count_SplitPtr,
  );

  static Future<Split> tryNewFromData(
          {required List<int> data,
          required FileType fileType,
          required SplitOptions options,
          dynamic hint}) =>
      BbqrCore.instance.api.splitTryNewFromData(
          data: data, fileType: fileType, options: options, hint: hint);
}

@freezed
sealed class EncodeError with _$EncodeError {
  const EncodeError._();

  /// No data to encode
  const factory EncodeError.empty() = EncodeError_Empty;

  /// Error while compressing data
  const factory EncodeError.compressionError(
    String field0,
  ) = EncodeError_CompressionError;
}

enum Encoding {
  hex,
  base32,
  zlib,
  ;
}

enum FileType {
  psbt,
  transaction,
  json,
  cbor,
  unicodeText,
  ;
}

@freezed
sealed class SplitError with _$SplitError implements FrbException {
  const SplitError._();

  /// No data found
  const factory SplitError.empty() = SplitError_Empty;

  /// Cannot make the data fit
  const factory SplitError.cannotFit() = SplitError_CannotFit;

  /// Max split size is too large
  const factory SplitError.maxSplitSizeTooLarge(
    int field0,
  ) = SplitError_MaxSplitSizeTooLarge;

  /// Min split size is too small
  const factory SplitError.minSplitTooSmall() = SplitError_MinSplitTooSmall;

  /// Invalid split min and max range, min is larger than max
  const factory SplitError.invalidSplitRange() = SplitError_InvalidSplitRange;

  /// Invalid version min and max range, min is larger than max
  const factory SplitError.invalidVersionRange() =
      SplitError_InvalidVersionRange;

  /// Error while encoding
  const factory SplitError.encodeError(
    EncodeError field0,
  ) = SplitError_EncodeError;
}

class SplitOptions {
  final Encoding encoding;
  final int minSplitNumber;
  final int maxSplitNumber;
  final Version minVersion;
  final Version maxVersion;

  const SplitOptions({
    required this.encoding,
    required this.minSplitNumber,
    required this.maxSplitNumber,
    required this.minVersion,
    required this.maxVersion,
  });

  @override
  int get hashCode =>
      encoding.hashCode ^
      minSplitNumber.hashCode ^
      maxSplitNumber.hashCode ^
      minVersion.hashCode ^
      maxVersion.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SplitOptions &&
          runtimeType == other.runtimeType &&
          encoding == other.encoding &&
          minSplitNumber == other.minSplitNumber &&
          maxSplitNumber == other.maxSplitNumber &&
          minVersion == other.minVersion &&
          maxVersion == other.maxVersion;
}

enum Version {
  /// Version n°01
  v01,

  /// Version n°02
  v02,

  /// Version n°03
  v03,

  /// Version n°04
  v04,

  /// Version n°05
  v05,

  /// Version n°06
  v06,

  /// Version n°07
  v07,

  /// Version n°08
  v08,

  /// Version n°09
  v09,

  /// Version n°10
  v10,

  /// Version n°11
  v11,

  /// Version n°12
  v12,

  /// Version n°13
  v13,

  /// Version n°14
  v14,

  /// Version n°15
  v15,

  /// Version n°16
  v16,

  /// Version n°17
  v17,

  /// Version n°18
  v18,

  /// Version n°19
  v19,

  /// Version n°20
  v20,

  /// Version n°21
  v21,

  /// Version n°22
  v22,

  /// Version n°23
  v23,

  /// Version n°24
  v24,

  /// Version n°25
  v25,

  /// Version n°26
  v26,

  /// Version n°27
  v27,

  /// Version n°28
  v28,

  /// Version n°29
  v29,

  /// Version n°30
  v30,

  /// Version n°31
  v31,

  /// Version n°32
  v32,

  /// Version n°33
  v33,

  /// Version n°34
  v34,

  /// Version n°35
  v35,

  /// Version n°36
  v36,

  /// Version n°37
  v37,

  /// Version n°38
  v38,

  /// Version n°39
  v39,

  /// Version n°40
  v40,
  ;
}
