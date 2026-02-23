FUNCTION main
VAR
    coordinateSet: CoordinateSet := [
        [0,1,2],
        [10,11,12],
        [20,21,22],
        [30,31,32]
    ];
    multiDemCoordinateSet: MultiDemCoordinateSet := [
        40,41,42,
        50,51,52,
        60,61,62,
        70,71,72
    ];
    addedNumbers : INT;
    addIntFb : AddInt_FB;
END_VAR
    // Print output
    PrintNumber(42);    // CHECK: The number you asked for: 42

    // Print enum values
    PrintAllEnumValues();
    // CHECK: Enum value 'A': 2
    // CHECK: Enum value 'B': 0
    // CHECK: Enum value 'C': 1
    // CHECK: Enum value 'D': 1000
    // CHECK: Enum value 'E': 1001

    // Print co-ordinate set
    PrintCoordinateSet(coordinateSet);
    // CHECK: [0, 1, 2]
    // CHECK-NEXT: [10, 11, 12]
    // CHECK-NEXT: [20, 21, 22]
    // CHECK-NEXT: [30, 31, 32]

    // Print multi dimensional co-ordinate set
    PrintMultiDemCoordinateSet(multiDemCoordinateSet);
    // CHECK: [40, 41, 42]
    // CHECK-NEXT: [50, 51, 52]
    // CHECK-NEXT: [60, 61, 62]
    // CHECK-NEXT: [70, 71, 72]

    // Add numbers together
    IF AddInt(4, 5, result => addedNumbers) = 0 THEN
        PrintNumber(addedNumbers);  // CHECK: The number you asked for: 9
    END_IF

    // Add numbers together using function block
    addIntFb();
    PrintNumber(addIntFb.result);  // CHECK: The number you asked for: 17
END_FUNCTION
