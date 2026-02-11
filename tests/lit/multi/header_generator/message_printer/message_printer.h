// ---------------------------------------------------- //
// This file is auto-generated                          //
// Manual changes made to this file will be overwritten //
// ---------------------------------------------------- //

#ifndef _WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_MESSAGE_PRINTER_MESSAGE_PRINTER_H_
#define _WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_MESSAGE_PRINTER_MESSAGE_PRINTER_H_

#include <stdint.h>
#include <stdbool.h>
#include <math.h>
#include <time.h>
#include <dependencies.plc.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef char T_Message[256];

// message: maximum of 256 T_Message(s)
void PrintMessage(T_Message* message);

void ManualMultiMessagePrinter(int16_t count, ...);

void SizedMultiMessagePrinter(int32_t messages_count, T_Message* messages);

#ifdef __cplusplus
}
#endif /* __cplusplus */

#endif /* !_WORKSPACES_RUSTY_TESTS_LIT_MULTI_HEADER_GENERATOR_MESSAGE_PRINTER_MESSAGE_PRINTER_H_ */
