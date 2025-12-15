#include "shape_drawer.h"

void __Rectangle__init(Rectangle_type* self) {}

void Rectangle(Rectangle_type* self) {}

void Rectangle__FB_INIT(Rectangle_type* self) {
    self->x = 6;
    self->y = 3;

    DrawRectangle(self->x, self->y);
}

void DrawRectangle(int32_t x, int32_t y) {
    for (int i = 0; i < y; i++) {
        for (int j = 0; j < x; j++) {
            printf("*");
        }
        printf("\n");
    }
}
