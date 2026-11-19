#include <stdint.h>
#include <stdio.h>

typedef struct {
    void *__vtable;
    int32_t handle;
} Channel_type;

static int32_t next_handle = 1;

void Channel(Channel_type *self) {}

void Channel__FB_INIT(Channel_type *self) {
    self->handle = next_handle++;
    printf("open %d\n", self->handle);
}

void Channel__FB_EXIT(Channel_type *self) { printf("close %d\n", self->handle); }
