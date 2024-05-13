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

typedef struct wire_cst_list_prim_u_8_strict {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_strict;

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

void frbgen_bbqr_dart_wire_Split_try_new_from_data(int64_t port_,
                                                   struct wire_cst_list_prim_u_8_loose *data,
                                                   int32_t file_type,
                                                   struct wire_cst_split_options *options);

void frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInner_Split(const void *ptr);

void frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInner_Split(const void *ptr);

struct wire_cst_encode_error *frbgen_bbqr_dart_cst_new_box_autoadd_encode_error(void);

struct wire_cst_split_options *frbgen_bbqr_dart_cst_new_box_autoadd_split_options(void);

struct wire_cst_list_prim_u_8_loose *frbgen_bbqr_dart_cst_new_list_prim_u_8_loose(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_bbqr_dart_cst_new_list_prim_u_8_strict(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_encode_error);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_split_options);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_list_prim_u_8_loose);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInner_Split);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInner_Split);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_Split_try_new_from_data);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
