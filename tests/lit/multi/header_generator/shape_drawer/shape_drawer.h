// ---------------------------------------------------- //
// This file is auto-generated                          //
// Manual changes made to this file will be overwritten //
// ---------------------------------------------------- //

#ifndef _WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_SHAPE_DRAWER_SHAPE_DRAWER_H_
#define _WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_SHAPE_DRAWER_SHAPE_DRAWER_H_

#include <stdint.h>
#include <math.h>
#include <stdbool.h>
#include <time.h>
#include <dependencies.plc.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint64_t* __vtable;
} Shape_type;

typedef struct {
    Shape_type __Shape;
    int32_t x;
    int32_t y;
} Rectangle_type;

typedef struct {
    Shape_type __Shape;
    int32_t x;
} Square_type;

void __Shape__init(Shape_type* self);

void Shape(Shape_type* self);

void Shape__Draw(Shape_type* self);

void __Rectangle__init(Rectangle_type* self);

void Rectangle(Rectangle_type* self);

void Rectangle__FB_INIT(Rectangle_type* self);

void Rectangle__Draw(Rectangle_type* self);

void __Square__init(Square_type* self);

void Square(Square_type* self);

void Square__FB_INIT(Square_type* self);

void Square__Draw(Square_type* self);

void DrawRectangle(int32_t x, int32_t y);

int32_t Square____get_size(Square_type* self);

void Square____set_size(Square_type* self, int32_t size);

void Square__DrawAction(Square_type* self);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* !_WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_SHAPE_DRAWER_SHAPE_DRAWER_H_ */
