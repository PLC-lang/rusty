#include <stdint.h>

typedef struct {
    void *__vtable;
    int16_t _value;
} FbExt_type;

void FbExt(FbExt_type *self) {}

int16_t FbExt____get_value(FbExt_type *self) { return self->_value; }

void FbExt____set_value(FbExt_type *self, int16_t value) { self->_value = value; }

int16_t FbExt__getValue(FbExt_type *self) { return self->_value; }
