FUNCTION main
VAR
    coordinateSet: CoordinateSet := [
        [0,1,2],
        [10,11,12],
        [20,21,22],
        [30,31,32]
    ];
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
    // CHECK: [0,1,2]
    // CHECK-NEXT: [10,11,12]
    // CHECK-NEXT: [20,21,22]
    // CHECK-NEXT: [30,31,32]
END_FUNCTION
