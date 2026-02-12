#include "shape_drawer.h"
#include <stdio.h>

void __Shape__init(Shape_type* self) {}

void Shape(Shape_type* self) {}

void Shape__Draw(Shape_type* self) {}

void __Rectangle__init(Rectangle_type* self) {}

void Rectangle(Rectangle_type* self) {}

void Rectangle__FB_INIT(Rectangle_type* self) {
    self->x = 6;
    self->y = 3;
}

void Rectangle__Draw(Rectangle_type* self) {
    DrawRectangle(self->x, self->y);
}

void __Square__init(Square_type* self) {}

void Square(Square_type* self) {}

void Square__FB_INIT(Square_type* self) {
    self->x = 4;
}

void Square__Draw(Square_type* self) {
    DrawRectangle(self->x, self->x);
}

void Square__DrawAction(Square_type* self) {
    Square__Draw(self);
}

int32_t Square____get_size(Square_type* self) {
    return self->x;
}

void Square____set_size(Square_type* self, int32_t size) {
    self->x = size;
}

void DrawRectangle(int32_t x, int32_t y) {
    for (int i = 0; i < y; i++) {
        for (int j = 0; j < x; j++) {
            printf("*");
        }
        printf("\n");
    }

    printf("\n");
}
