#include "number_printer.h"
#include <stdio.h>

void PrintNumber(int16_t valueToPrint) {
    printf("The number you asked for: %d\n", valueToPrint);
}

void PrintAllEnumValues() {
    printf("Enum value 'A': %d\n", PartialEnumType_A);
    printf("Enum value 'B': %d\n", PartialEnumType_B);
    printf("Enum value 'C': %d\n", PartialEnumType_C);
    printf("Enum value 'D': %d\n", PartialEnumType_D);
    printf("Enum value 'E': %d\n", PartialEnumType_E);
}

void PrintCoordinateSet(CoordinateSet* coordinateSet) {
    for (int i = 0; i < 4; i++) {
        printf("[");
        for (int j = 0; j < 3; j++) {
            printf("%d", *(*(*coordinateSet + i) + j));

            if (j < (3 - 1)) {
                printf(", ");
            }
        }
        printf("]\n");
    }
}

void PrintMultiDemCoordinateSet(MultiDemCoordinateSet* coordinateSet) {
    for (int i = 0; i < 4; i++) {
        printf("[");
        for (int j = 0; j < 3; j++) {
            printf("%d", *(*(*coordinateSet + i) + j));

            if (j < (3 - 1)) {
                printf(", ");
            }
        }
        printf("]\n");
    }
}
