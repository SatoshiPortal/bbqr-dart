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

typedef struct wire_cst_split_options {
  int32_t encoding;
  uintptr_t min_split_number;
  uintptr_t max_split_number;
  int32_t min_version;
  int32_t max_version;
} wire_cst_split_options;

void frbgen_bbqr_dart_wire_Split_try_new_from_data(int64_t port_,
                                                   uintptr_t data,
                                                   int32_t file_type,
                                                   struct wire_cst_split_options *options);

void frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerError(const void *ptr);

void frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerError(const void *ptr);

void frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInner_Split(const void *ptr);

void frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInner_Split(const void *ptr);

void frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInneru8(const void *ptr);

void frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInneru8(const void *ptr);

struct wire_cst_split_options *frbgen_bbqr_dart_cst_new_box_autoadd_split_options(void);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_split_options);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerError);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInner_Split);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInneru8);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerError);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInner_Split);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInneru8);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_Split_try_new_from_data);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
