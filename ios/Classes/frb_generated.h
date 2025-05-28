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

typedef struct wire_cst_list_String {
  struct wire_cst_list_prim_u_8_strict **ptr;
  int32_t len;
} wire_cst_list_String;

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

typedef struct wire_cst_joined {
  int32_t encoding;
  int32_t file_type;
  struct wire_cst_list_prim_u_8_strict *data;
} wire_cst_joined;

typedef struct wire_cst_split {
  int32_t version;
  struct wire_cst_list_String *parts;
  int32_t encoding;
} wire_cst_split;

void frbgen_dart_bbqr_wire__bbqr__continuous_join__ContinuousJoiner_default(int64_t port_);

void frbgen_dart_bbqr_wire__bbqr__continuous_join__ContinuousJoiner_frb_override_add_part(int64_t port_,
                                                                                          uintptr_t that,
                                                                                          struct wire_cst_list_prim_u_8_strict *part);

void frbgen_dart_bbqr_wire__bbqr__continuous_join__ContinuousJoiner_new(int64_t port_);

void frbgen_dart_bbqr_wire__bbqr__encode__encoding_as_byte(int64_t port_, int32_t that);

void frbgen_dart_bbqr_wire__bbqr__encode__encoding_from_byte(int64_t port_, uint8_t byte);

void frbgen_dart_bbqr_wire__bbqr__encode__encoding_is_known_encoding(int64_t port_, uint8_t byte);

void frbgen_dart_bbqr_wire__bbqr__encode__encoding_split_mod(int64_t port_, int32_t that);

void frbgen_dart_bbqr_wire__bbqr__file_type__file_type_as_byte(int64_t port_, int32_t that);

void frbgen_dart_bbqr_wire__bbqr__file_type__file_type_from_byte(int64_t port_, uint8_t byte);

void frbgen_dart_bbqr_wire__bbqr__file_type__file_type_is_known_filetype(int64_t port_,
                                                                         uint8_t byte);

void frbgen_dart_bbqr_wire__bbqr__join__joined_frb_override_try_from_parts(int64_t port_,
                                                                           struct wire_cst_list_String *parts);

void frbgen_dart_bbqr_wire__bbqr__split__split_frb_override_try_from_data(int64_t port_,
                                                                          struct wire_cst_list_prim_u_8_loose *bytes,
                                                                          int32_t file_type,
                                                                          struct wire_cst_split_options *options);

void frbgen_dart_bbqr_wire__bbqr__split__split_options_default(int64_t port_);

void frbgen_dart_bbqr_wire__bbqr__qr__version_data_capacity(int64_t port_, int32_t that);

void frbgen_dart_bbqr_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerContinuousJoinResult(const void *ptr);

void frbgen_dart_bbqr_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerContinuousJoinResult(const void *ptr);

void frbgen_dart_bbqr_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerContinuousJoiner(const void *ptr);

void frbgen_dart_bbqr_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerContinuousJoiner(const void *ptr);

int32_t *frbgen_dart_bbqr_cst_new_box_autoadd_encoding(int32_t value);

int32_t *frbgen_dart_bbqr_cst_new_box_autoadd_file_type(int32_t value);

struct wire_cst_split_options *frbgen_dart_bbqr_cst_new_box_autoadd_split_options(void);

struct wire_cst_list_String *frbgen_dart_bbqr_cst_new_list_String(int32_t len);

struct wire_cst_list_prim_u_8_loose *frbgen_dart_bbqr_cst_new_list_prim_u_8_loose(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_dart_bbqr_cst_new_list_prim_u_8_strict(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_cst_new_box_autoadd_encoding);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_cst_new_box_autoadd_file_type);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_cst_new_box_autoadd_split_options);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_cst_new_list_String);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_cst_new_list_prim_u_8_loose);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerContinuousJoinResult);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerContinuousJoiner);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerContinuousJoinResult);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerContinuousJoiner);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__continuous_join__ContinuousJoiner_default);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__continuous_join__ContinuousJoiner_frb_override_add_part);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__continuous_join__ContinuousJoiner_new);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__encode__encoding_as_byte);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__encode__encoding_from_byte);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__encode__encoding_is_known_encoding);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__encode__encoding_split_mod);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__file_type__file_type_as_byte);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__file_type__file_type_from_byte);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__file_type__file_type_is_known_filetype);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__join__joined_frb_override_try_from_parts);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__qr__version_data_capacity);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__split__split_frb_override_try_from_data);
    dummy_var ^= ((int64_t) (void*) frbgen_dart_bbqr_wire__bbqr__split__split_options_default);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
