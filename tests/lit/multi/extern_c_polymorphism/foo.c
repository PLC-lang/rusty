#include <stdio.h>

typedef struct {
    void* __vtable;
    int id;
} ExternalFbInstance;

// Function block body (required for FBs)
void ExternalFb(ExternalFbInstance* self) {}

// FB_INIT: called by the generated constructor
void ExternalFb__FB_INIT(ExternalFbInstance* self) {
    self->id = 42;
}

// Method: implements IGreeter.greet
void ExternalFb__greet(ExternalFbInstance* self) {
    printf("ExternalFb::greet (id=%d)\n", self->id);
}
