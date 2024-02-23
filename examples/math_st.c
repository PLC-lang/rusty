#include <math.h>
#include <stdio.h>

typedef struct LOG_interface {
    int x;
} LOG_interface;

typedef struct PRINT_interface {
    char text[81];
    int value;
} PRINT_interface;

int LOG(LOG_interface* param) {
    printf("Calling log with %d\n", param->x);
    int res =  (int) log10(param->x);
    printf("result :  %d\n", res);
    return res;
}

int PRINTF(PRINT_interface* param) {
    return printf(param->text, param->value);
}
