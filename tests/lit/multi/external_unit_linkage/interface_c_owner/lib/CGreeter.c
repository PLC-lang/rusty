#include <stdint.h>
#include <string.h>

typedef struct {
    void *__vtable;
    int32_t calls;
} CGreeter_type;

void CGreeter(CGreeter_type *self) {}

int32_t CGreeter__greet(CGreeter_type *self, const char *name, const int32_t *weights) {
    self->calls += 1;
    return (int32_t)strlen(name) * 100 + weights[0] * 10 + weights[1] + self->calls * 1000;
}
