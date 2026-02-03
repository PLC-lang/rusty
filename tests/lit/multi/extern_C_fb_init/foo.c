#include <stdio.h>

typedef struct {
    void* __vtable;
    int a;
    int b;
} fbInstance;

void myFunctionBlock(fbInstance* fb_instance) {}
void myFunctionBlock__FB_INIT(fbInstance* fb_instance) {
    fb_instance->a = 1;
    fb_instance->b = 2;
    printf("myFunctionBlock initialized with a = %d, b = %d\n", fb_instance->a, fb_instance->b);
}
