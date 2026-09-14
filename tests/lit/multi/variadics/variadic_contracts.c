#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>

/* Terminator-directed: walks the argument list until it finds a null. This is the shape that
   reads past the arguments when the caller leaves the list unterminated. */
void SentinelPrinter(const char *prefix, ...) {
    va_list args;
    va_start(args, prefix);
    printf("%s", prefix);
    for (const char *it = va_arg(args, const char *); it != NULL; it = va_arg(args, const char *)) {
        printf(" %s", it);
    }
    printf("\n");
    va_end(args);
}

/* Count-directed: reads exactly `count` arguments, so the appended terminator stays unread. */
void CountPrinter(int32_t count, ...) {
    va_list args;
    va_start(args, count);
    printf("count=%d", count);
    for (int32_t i = 0; i < count; i++) {
        printf(" %s", va_arg(args, const char *));
    }
    printf("\n");
    va_end(args);
}

/* Reports how many arguments precede the first null. Proves that exactly one terminator arrives,
   and that a terminator written by the caller is the one that ends the list. */
int32_t ArgsBeforeTerminator(const char *label, ...) {
    va_list args;
    int32_t count = 0;
    va_start(args, label);
    while (va_arg(args, const void *) != NULL) {
        count++;
    }
    va_end(args);
    return count;
}

/* Terminator-directed over a by-ref variadic block (`VAR_INPUT {ref} args : ...`): every argument
   arrives as a pointer and the terminator is a null of the same shape. */
void RefSentinelPrinter(const char *prefix, ...) {
    va_list args;
    va_start(args, prefix);
    printf("%s", prefix);
    for (const int32_t *it = va_arg(args, const int32_t *); it != NULL;
         it = va_arg(args, const int32_t *)) {
        printf(" %d", *it);
    }
    printf("\n");
    va_end(args);
}

/* Format-directed over mixed register classes: pointers, 64-bit integers and doubles. */
void FormatPrinter(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);
    vprintf(fmt, args);
    va_end(args);
}

/* Typed unsized variadic (`args : DINT...`): the callee signature is concrete, so this list
   carries no terminator and a wrong count would read past the arguments. */
void TypedCountPrinter(int32_t count, ...) {
    va_list args;
    va_start(args, count);
    printf("typed:");
    for (int32_t i = 0; i < count; i++) {
        printf(" %d", va_arg(args, int32_t));
    }
    printf("\n");
    va_end(args);
}

/* Sized variadic (`args : {sized} DINT...`): the compiler passes (count, array). */
void SizedPrinter(int32_t args_count, int32_t *args) {
    printf("sized:");
    for (int32_t i = 0; i < args_count; i++) {
        printf(" %d", args[i]);
    }
    printf("\n");
}
