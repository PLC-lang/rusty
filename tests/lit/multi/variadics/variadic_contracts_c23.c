#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>

/* C23 relaxes variadic parameter lists: a variadic function needs no named parameter and
   `va_start` takes no second argument. Both forms are unreachable before C23, so this file is
   compiled separately from variadic_contracts.c. */

/* Terminator-directed with no named parameter at all. The implicit terminator is the only
   argument a call without variadic arguments carries. */
void OnlyVariadic(...) {
    va_list args;
    va_start(args);
    printf("onlyvariadic:");
    for (const char *it = va_arg(args, const char *); it != NULL; it = va_arg(args, const char *)) {
        printf(" %s", it);
    }
    printf("\n");
    va_end(args);
}

/* Reports how many arguments precede the first null, without a named parameter to anchor. */
int32_t OnlyVariadicCount(...) {
    va_list args;
    int32_t count = 0;
    va_start(args);
    while (va_arg(args, const void *) != NULL) {
        count++;
    }
    va_end(args);
    return count;
}
