#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
// EXTRA BEGIN
typedef struct DartCObject *WireSyncRust2DartDco;
typedef struct WireSyncRust2DartSse {
  uint8_t *ptr;
  int32_t len;
} WireSyncRust2DartSse;

typedef int64_t DartPort;
typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);
void store_dart_post_cobject(DartPostCObjectFnType ptr);
// EXTRA END
typedef struct _Dart_Handle* Dart_Handle;

typedef struct wire_cst_list_prim_u_8_strict {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_strict;

typedef struct wire_cst_list_prim_u_8_loose {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_loose;

typedef struct wire_cst_split_options {
  int32_t encoding;
  uintptr_t min_split_number;
  uintptr_t max_split_number;
  int32_t min_version;
  int32_t max_version;
} wire_cst_split_options;

typedef struct wire_cst_list_String {
  struct wire_cst_list_prim_u_8_strict **ptr;
  int32_t len;
} wire_cst_list_String;

typedef struct wire_cst_DecodeError_UnableToDecodeHex {
  uintptr_t field0;
  struct wire_cst_list_prim_u_8_strict *field1;
} wire_cst_DecodeError_UnableToDecodeHex;

typedef struct wire_cst_DecodeError_UnableToDecodeBase32 {
  uintptr_t field0;
  struct wire_cst_list_prim_u_8_strict *field1;
} wire_cst_DecodeError_UnableToDecodeBase32;

typedef struct wire_cst_DecodeError_UnableToInflateZlib {
  struct wire_cst_list_prim_u_8_strict *field0;
} wire_cst_DecodeError_UnableToInflateZlib;

typedef union DecodeErrorKind {
  struct wire_cst_DecodeError_UnableToDecodeHex UnableToDecodeHex;
  struct wire_cst_DecodeError_UnableToDecodeBase32 UnableToDecodeBase32;
  struct wire_cst_DecodeError_UnableToInflateZlib UnableToInflateZlib;
} DecodeErrorKind;

typedef struct wire_cst_decode_error {
  int32_t tag;
  union DecodeErrorKind kind;
} wire_cst_decode_error;

typedef struct wire_cst_EncodeError_CompressionError {
  struct wire_cst_list_prim_u_8_strict *field0;
} wire_cst_EncodeError_CompressionError;

typedef union EncodeErrorKind {
  struct wire_cst_EncodeError_CompressionError CompressionError;
} EncodeErrorKind;

typedef struct wire_cst_encode_error {
  int32_t tag;
  union EncodeErrorKind kind;
} wire_cst_encode_error;

typedef struct wire_cst_HeaderParseError_InvalidEncoding {
  struct wire_cst_list_prim_u_8_strict *field0;
} wire_cst_HeaderParseError_InvalidEncoding;

typedef struct wire_cst_HeaderParseError_InvalidFileType {
  struct wire_cst_list_prim_u_8_strict *field0;
} wire_cst_HeaderParseError_InvalidFileType;

typedef struct wire_cst_HeaderParseError_InvalidHeaderSize {
  uintptr_t field0;
} wire_cst_HeaderParseError_InvalidHeaderSize;

typedef struct wire_cst_HeaderParseError_InvalidHeaderParts {
  struct wire_cst_list_prim_u_8_strict *field0;
} wire_cst_HeaderParseError_InvalidHeaderParts;

typedef union HeaderParseErrorKind {
  struct wire_cst_HeaderParseError_InvalidEncoding InvalidEncoding;
  struct wire_cst_HeaderParseError_InvalidFileType InvalidFileType;
  struct wire_cst_HeaderParseError_InvalidHeaderSize InvalidHeaderSize;
  struct wire_cst_HeaderParseError_InvalidHeaderParts InvalidHeaderParts;
} HeaderParseErrorKind;

typedef struct wire_cst_header_parse_error {
  int32_t tag;
  union HeaderParseErrorKind kind;
} wire_cst_header_parse_error;

typedef struct wire_cst_JoinError_TooManyParts {
  uintptr_t field0;
  uintptr_t field1;
} wire_cst_JoinError_TooManyParts;

typedef struct wire_cst_JoinError_DuplicatePartWrongContent {
  uintptr_t field0;
} wire_cst_JoinError_DuplicatePartWrongContent;

typedef struct wire_cst_JoinError_PartWithNoData {
  uintptr_t field0;
} wire_cst_JoinError_PartWithNoData;

typedef struct wire_cst_JoinError_MissingPart {
  uintptr_t field0;
} wire_cst_JoinError_MissingPart;

typedef struct wire_cst_JoinError_HeaderParseError {
  struct wire_cst_header_parse_error *field0;
} wire_cst_JoinError_HeaderParseError;

typedef struct wire_cst_JoinError_DecodeError {
  struct wire_cst_decode_error *field0;
} wire_cst_JoinError_DecodeError;

typedef union JoinErrorKind {
  struct wire_cst_JoinError_TooManyParts TooManyParts;
  struct wire_cst_JoinError_DuplicatePartWrongContent DuplicatePartWrongContent;
  struct wire_cst_JoinError_PartWithNoData PartWithNoData;
  struct wire_cst_JoinError_MissingPart MissingPart;
  struct wire_cst_JoinError_HeaderParseError HeaderParseError;
  struct wire_cst_JoinError_DecodeError DecodeError;
} JoinErrorKind;

typedef struct wire_cst_join_error {
  int32_t tag;
  union JoinErrorKind kind;
} wire_cst_join_error;

typedef struct wire_cst_joined {
  int32_t encoding;
  int32_t file_type;
  struct wire_cst_list_prim_u_8_strict *data;
} wire_cst_joined;

typedef struct wire_cst_ContinuousJoinError_HeaderParseError {
  struct wire_cst_header_parse_error *field0;
} wire_cst_ContinuousJoinError_HeaderParseError;

typedef struct wire_cst_ContinuousJoinError_JoinError {
  struct wire_cst_join_error *field0;
} wire_cst_ContinuousJoinError_JoinError;

typedef struct wire_cst_ContinuousJoinError_DecodeError {
  struct wire_cst_decode_error *field0;
} wire_cst_ContinuousJoinError_DecodeError;

typedef union ContinuousJoinErrorKind {
  struct wire_cst_ContinuousJoinError_HeaderParseError HeaderParseError;
  struct wire_cst_ContinuousJoinError_JoinError JoinError;
  struct wire_cst_ContinuousJoinError_DecodeError DecodeError;
} ContinuousJoinErrorKind;

typedef struct wire_cst_continuous_join_error {
  int32_t tag;
  union ContinuousJoinErrorKind kind;
} wire_cst_continuous_join_error;

typedef struct wire_cst_JoinResult_InProgress {
  uintptr_t parts_left;
} wire_cst_JoinResult_InProgress;

typedef struct wire_cst_JoinResult_Complete {
  struct wire_cst_joined *joined;
} wire_cst_JoinResult_Complete;

typedef union JoinResultKind {
  struct wire_cst_JoinResult_InProgress InProgress;
  struct wire_cst_JoinResult_Complete Complete;
} JoinResultKind;

typedef struct wire_cst_join_result {
  int32_t tag;
  union JoinResultKind kind;
} wire_cst_join_result;

typedef struct wire_cst_SplitError_MaxSplitSizeTooLarge {
  uintptr_t field0;
} wire_cst_SplitError_MaxSplitSizeTooLarge;

typedef struct wire_cst_SplitError_EncodeError {
  struct wire_cst_encode_error *field0;
} wire_cst_SplitError_EncodeError;

typedef union SplitErrorKind {
  struct wire_cst_SplitError_MaxSplitSizeTooLarge MaxSplitSizeTooLarge;
  struct wire_cst_SplitError_EncodeError EncodeError;
} SplitErrorKind;

typedef struct wire_cst_split_error {
  int32_t tag;
  union SplitErrorKind kind;
} wire_cst_split_error;

WireSyncRust2DartDco frbgen_bbqr_dart_wire_ContinuousJoiner_add_part(uintptr_t that,
                                                                     struct wire_cst_list_prim_u_8_strict *part);

WireSyncRust2DartDco frbgen_bbqr_dart_wire_ContinuousJoiner_new(void);

WireSyncRust2DartDco frbgen_bbqr_dart_wire_Split_encoding(uintptr_t that);

WireSyncRust2DartDco frbgen_bbqr_dart_wire_Split_parts(uintptr_t that);

void frbgen_bbqr_dart_wire_Split_try_from_data(int64_t port_,
                                               struct wire_cst_list_prim_u_8_loose *data,
                                               int32_t file_type,
                                               struct wire_cst_split_options *options);

WireSyncRust2DartDco frbgen_bbqr_dart_wire_Split_version(uintptr_t that);

WireSyncRust2DartDco frbgen_bbqr_dart_wire_default_split_options(void);

WireSyncRust2DartDco frbgen_bbqr_dart_wire_joined_try_new_from_parts(struct wire_cst_list_String *parts);

void frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockContinuousJoiner(const void *ptr);

void frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockContinuousJoiner(const void *ptr);

void frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLock_Split(const void *ptr);

void frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLock_Split(const void *ptr);

struct wire_cst_decode_error *frbgen_bbqr_dart_cst_new_box_autoadd_decode_error(void);

struct wire_cst_encode_error *frbgen_bbqr_dart_cst_new_box_autoadd_encode_error(void);

struct wire_cst_header_parse_error *frbgen_bbqr_dart_cst_new_box_autoadd_header_parse_error(void);

struct wire_cst_join_error *frbgen_bbqr_dart_cst_new_box_autoadd_join_error(void);

struct wire_cst_joined *frbgen_bbqr_dart_cst_new_box_autoadd_joined(void);

struct wire_cst_split_options *frbgen_bbqr_dart_cst_new_box_autoadd_split_options(void);

struct wire_cst_list_String *frbgen_bbqr_dart_cst_new_list_String(int32_t len);

struct wire_cst_list_prim_u_8_loose *frbgen_bbqr_dart_cst_new_list_prim_u_8_loose(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_bbqr_dart_cst_new_list_prim_u_8_strict(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_decode_error);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_encode_error);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_header_parse_error);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_join_error);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_joined);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_split_options);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_list_String);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_list_prim_u_8_loose);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockContinuousJoiner);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLock_Split);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockContinuousJoiner);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLock_Split);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_ContinuousJoiner_add_part);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_ContinuousJoiner_new);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_Split_encoding);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_Split_parts);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_Split_try_from_data);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_Split_version);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_default_split_options);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_joined_try_new_from_parts);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
