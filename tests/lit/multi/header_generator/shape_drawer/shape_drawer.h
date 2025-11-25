// ---------------------------------------------------- //
// This file is auto-generated                          //
// Manual changes made to this file will be overwritten //
// ---------------------------------------------------- //

#ifndef SHAPE_DRAWER
#define SHAPE_DRAWER

#include <dependencies.plc.h>

typedef struct {
    uint64_t* __vtable;
    int32_t x;
    int32_t y;
} Rectangle_type;

extern Rectangle_type __Rectangle__init = { 0 };

void Rectangle(Rectangle_type* self);

void Rectangle__FB_INIT(Rectangle_type* self);

void DrawRectangle(int32_t x, int32_t y);

#endif /* !SHAPE_DRAWER */
