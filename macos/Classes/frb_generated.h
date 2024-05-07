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

typedef struct wire_cst_list_prim_u_8_strict {
  uint8_t *ptr;
  int32_t len;
} wire_cst_list_prim_u_8_strict;

typedef struct wire_cst_dummy {
  struct wire_cst_list_prim_u_8_strict *field;
} wire_cst_dummy;

void frbgen_bbqr_dart_wire_dummy_split(int64_t port_, struct wire_cst_list_prim_u_8_loose *data);

struct wire_cst_list_prim_u_8_loose *frbgen_bbqr_dart_cst_new_list_prim_u_8_loose(int32_t len);

struct wire_cst_list_prim_u_8_strict *frbgen_bbqr_dart_cst_new_list_prim_u_8_strict(int32_t len);
static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_list_prim_u_8_loose);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_cst_new_list_prim_u_8_strict);
    dummy_var ^= ((int64_t) (void*) frbgen_bbqr_dart_wire_dummy_split);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    return dummy_var;
}
