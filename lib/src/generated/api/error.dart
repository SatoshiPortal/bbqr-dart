// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.31.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'error.freezed.dart';

@freezed
sealed class DecodeError with _$DecodeError {
  const DecodeError._();

  /// Unable to decode hex part
  const factory DecodeError.unableToDecodeHex(
    int field0,
    String field1,
  ) = DecodeError_UnableToDecodeHex;

  /// Unable to decode base32 part
  const factory DecodeError.unableToDecodeBase32(
    int field0,
    String field1,
  ) = DecodeError_UnableToDecodeBase32;

  /// Unable to decompress zlib data
  const factory DecodeError.unableToInflateZlib(
    String field0,
  ) = DecodeError_UnableToInflateZlib;
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

@freezed
sealed class HeaderParseError with _$HeaderParseError {
  const HeaderParseError._();

  /// No data found
  const factory HeaderParseError.empty() = HeaderParseError_Empty;

  /// Invalid encoding
  const factory HeaderParseError.invalidEncoding(
    String field0,
  ) = HeaderParseError_InvalidEncoding;

  /// Invalid file type
  const factory HeaderParseError.invalidFileType(
    String field0,
  ) = HeaderParseError_InvalidFileType;

  /// Invalid fixed header
  const factory HeaderParseError.invalidFixedHeader() =
      HeaderParseError_InvalidFixedHeader;

  /// Invalid header size, not long enough
  const factory HeaderParseError.invalidHeaderSize(
    int field0,
  ) = HeaderParseError_InvalidHeaderSize;

  /// Invalid header parts, not enough parts
  const factory HeaderParseError.invalidHeaderParts(
    String field0,
  ) = HeaderParseError_InvalidHeaderParts;
}

@freezed
sealed class JoinError with _$JoinError implements FrbException {
  const JoinError._();

  /// No data found
  const factory JoinError.empty() = JoinError_Empty;

  /// Conflicting/variable file type/encodings/sizes
  const factory JoinError.conflictingHeaders() = JoinError_ConflictingHeaders;

  /// Too many parts
  const factory JoinError.tooManyParts(
    int field0,
    int field1,
  ) = JoinError_TooManyParts;

  /// Duplicate part with different content
  const factory JoinError.duplicatePartWrongContent(
    int field0,
  ) = JoinError_DuplicatePartWrongContent;

  /// Part with no data
  const factory JoinError.partWithNoData(
    int field0,
  ) = JoinError_PartWithNoData;

  /// Missing part
  const factory JoinError.missingPart(
    int field0,
  ) = JoinError_MissingPart;

  /// Header parse error
  const factory JoinError.headerParseError(
    HeaderParseError field0,
  ) = JoinError_HeaderParseError;

  /// Decode error
  const factory JoinError.decodeError(
    DecodeError field0,
  ) = JoinError_DecodeError;
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
