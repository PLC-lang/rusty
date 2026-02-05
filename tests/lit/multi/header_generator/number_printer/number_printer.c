#include "number_printer.h"

void PrintNumber(int16_t valueToPrint) {
    printf("The number you asked for: %d\n", valueToPrint);
}

void PrintAllEnumValues() {
    printf("Enum value 'A': %d\n", A);
    printf("Enum value 'B': %d\n", B);
    printf("Enum value 'C': %d\n", C);
    printf("Enum value 'D': %d\n", D);
    printf("Enum value 'E': %d\n", E);
}

void PrintCoordinateSet(CoordinateSet* coordinateSet) {
    for (int i = 0; i < 4; i++) {
        printf("[");
        for (int j = 0; j < 3; j++) {
            printf("%s", coordinateSet + i + j);

            if (j < (3 - 1)) {
                printf(", ");
            }
        }
        printf("]\n");
    }
}
