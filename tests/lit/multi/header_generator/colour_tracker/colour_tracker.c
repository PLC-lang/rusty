#include "colour_tracker.h"

int16_t globalCounter = 0;

void PrintStatistics(int32_t argumentCount, ColourInfo* colours) {
    for (int i = 0; i < argumentCount; i++) {
        ColourInfo colourInfo = colours[i];

        PrintColourInfo(&colourInfo);

        globalCounter++;
    }

    printf("Global Count: %d\n", globalCounter);
}

void TestPrinter() {
    printf("Testing...\n");
}

void PrintColourInfo(ColourInfo* colourInfo) {
    switch(colourInfo->primaryColour) {
        case red: printf("Red, Times Picked: %d\n", colourInfo->timesPicked); break;
        case green: printf("Green, Times Picked: %d\n", colourInfo->timesPicked); break;
        case blue: printf("Blue, Times Picked: %d\n", colourInfo->timesPicked); break;
        default: break;
    }
}
