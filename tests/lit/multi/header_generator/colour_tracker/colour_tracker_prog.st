FUNCTION main
VAR
    ciArray: ARRAY[0..2] OF ColourInfo := [
        (timesPicked := 5, primaryColour := red),
        (timesPicked := 4, primaryColour := green),
        (timesPicked := 6, primaryColour := blue)
    ];
END_VAR
    // Test Printer
    TestPrinter();      // CHECK: Testing...

    // Print output
    PrintStatistics(3, ciArray);

    // CHECK: Red, Times Picked: 5
    // CHECK-NEXT: Green, Times Picked: 4
    // CHECK-NEXT: Blue, Times Picked: 6
    // CHECK-NEXT: Global Count: 3

END_FUNCTION
