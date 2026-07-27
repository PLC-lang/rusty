#include "library_string_overflow.h"

#include <stdint.h>
#include <string.h>

ErrorCode Config_GetString(char *key, char *value) {
    (void)key;

    // Fill the whole destination buffer as an external library would for STRING[1023].
    memset(value, 'A', 1023);
    value[1023] = '\0';

    return 0;
}
