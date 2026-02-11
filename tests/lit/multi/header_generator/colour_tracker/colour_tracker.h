// ---------------------------------------------------- //
// This file is auto-generated                          //
// Manual changes made to this file will be overwritten //
// ---------------------------------------------------- //

#ifndef _WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_COLOUR_TRACKER_COLOUR_TRACKER_H_
#define _WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_COLOUR_TRACKER_COLOUR_TRACKER_H_

#include <stdint.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>
#include <dependencies.plc.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef int32_t RGB;
#define RGB_red ((RGB)0)
#define RGB_green ((RGB)1)
#define RGB_blue ((RGB)2)

typedef struct {
    int32_t timesPicked;
    RGB primaryColour;
} ColourInfo;

extern int16_t globalCounter;

// colours: maximum of 3 ColourInfo(s)
void PrintStatistics(int32_t argumentCount, ColourInfo* colours);

void TestPrinter();

void PrintColourInfo(ColourInfo* colourInfo);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* !_WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_COLOUR_TRACKER_COLOUR_TRACKER_H_ */
