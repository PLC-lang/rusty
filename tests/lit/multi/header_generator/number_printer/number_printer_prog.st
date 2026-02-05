FUNCTION main
VAR
    // No variables
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
END_FUNCTION
