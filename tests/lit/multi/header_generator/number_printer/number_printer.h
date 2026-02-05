// ---------------------------------------------------- //
// This file is auto-generated                          //
// Manual changes made to this file will be overwritten //
// ---------------------------------------------------- //

#ifndef _WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_NUMBER_PRINTER_NUMBER_PRINTER_H_
#define _WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_NUMBER_PRINTER_NUMBER_PRINTER_H_

#include <stdint.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>
#include <dependencies.plc.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ePartialEnumType {
    A = 2,
    B = 0,
    C,
    D = 1000,
    E
} PartialEnumType;

void PrintNumber(int16_t valueToPrint);

void PrintAllEnumValues();

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* !_WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_NUMBER_PRINTER_NUMBER_PRINTER_H_ */
