// ---------------------------------------------------- //
// This file is auto-generated                          //
// Manual changes made to this file will be overwritten //
// ---------------------------------------------------- //

#ifndef SHAPE_DRAWER
#define SHAPE_DRAWER

#include <stdint.h>
#include <math.h>
#include <stdbool.h>
#include <dependencies.plc.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint64_t* __vtable;
    int32_t x;
    int32_t y;
} Rectangle_type;

void __Rectangle__init(Rectangle_type* self);

void Rectangle(Rectangle_type* self);

void Rectangle__FB_INIT(Rectangle_type* self);

void DrawRectangle(int32_t x, int32_t y);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* !SHAPE_DRAWER */
