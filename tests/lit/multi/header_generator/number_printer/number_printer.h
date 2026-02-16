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

typedef int16_t __CoordinateSet[3];

typedef int16_t MultiDemCoordinateSet[4][3];

typedef __CoordinateSet CoordinateSet[4];

typedef int32_t PartialEnumType;
#define PartialEnumType_A ((PartialEnumType)2)
#define PartialEnumType_B ((PartialEnumType)0)
#define PartialEnumType_C ((PartialEnumType)1)
#define PartialEnumType_D ((PartialEnumType)1000)
#define PartialEnumType_E ((PartialEnumType)1001)

typedef struct {
    uint64_t* __vtable;
    int16_t a;
    int16_t b;
    int16_t result;
} AddInt_FB_type;

void PrintNumber(int16_t valueToPrint);

void PrintAllEnumValues();

// coordinateSet: maximum of 4 CoordinateSet(s)
void PrintCoordinateSet(CoordinateSet* coordinateSet);

// coordinateSet: maximum of 4 MultiDemCoordinateSet(s)
void PrintMultiDemCoordinateSet(MultiDemCoordinateSet* coordinateSet);

int16_t AddInt(int16_t a, int16_t b, int16_t* result);

void __AddInt_FB__init(AddInt_FB_type* self);

void AddInt_FB(AddInt_FB_type* self);

void AddInt_FB__FB_INIT(AddInt_FB_type* self);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* !_WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_NUMBER_PRINTER_NUMBER_PRINTER_H_ */
