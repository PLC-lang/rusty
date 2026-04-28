#include "message_printer.h"

void PrintMessage(T_Message* message) {
    printf("%s\n", message);
}

void ManualMultiMessagePrinter(int16_t count, ...) {
    va_list args;
    va_start(args, count);
    for (int i = 0; i < count; i++)
        printf("%s\n", va_arg(args, char*));
    va_end(args);
}

void SizedMultiMessagePrinter(int32_t messages_count, T_Message* messages) {
    for (int i = 0; i < messages_count; i++)
        printf("%s\n", messages + i);
}
