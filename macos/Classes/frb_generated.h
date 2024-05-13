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
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_decode_error);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_encode_error);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_header_parse_error);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_box_autoadd_split_options);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_list_String);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_list_prim_u_8_loose);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLock_Joined);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLock_Split);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLock_Joined);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLock_Split);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_Joined_try_new_from_parts);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_Split_try_from_data);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
