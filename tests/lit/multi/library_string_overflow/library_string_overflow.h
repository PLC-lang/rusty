// ---------------------------------------------------- //
// This file is auto-generated                          //
// Manual changes made to this file will be overwritten //
// ---------------------------------------------------- //

#ifndef TESTS_LIT_MULTI_LIBRARY_STRING_OVERFLOW_LIBRARY_STRING_OVERFLOW_H_
#define TESTS_LIT_MULTI_LIBRARY_STRING_OVERFLOW_LIBRARY_STRING_OVERFLOW_H_

#include <stdint.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>
#include <dependencies.plc.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef int32_t ErrorCode;

// Key: maximum of 1024 char(s)
// Value: maximum of 1024 char(s)
ErrorCode Config_GetString(char* Key, char* Value);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* !TESTS_LIT_MULTI_LIBRARY_STRING_OVERFLOW_LIBRARY_STRING_OVERFLOW_H_ */
