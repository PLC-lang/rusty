#include <stdio.h>

typedef struct {
    int a;
    int b;
} myFunctionBlock;

myFunctionBlock __myFunctionBlock__init = { 0 };

void myFunctionBlock__FB_INIT(myFunctionBlock* fb_instance) {
    fb_instance->a = 1;
    fb_instance->b = 2;
    printf("myFunctionBlock initialized with a = %d, b = %d\n", fb_instance->a, fb_instance->b);
}