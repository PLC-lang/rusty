FUNCTION main
VAR
    ciArray: ARRAY[0..2] OF ColourInfo := [
        (timesPicked := 5, primaryColour := red),
        (timesPicked := 4, primaryColour := green),
        (timesPicked := 6, primaryColour := blue)
    ];
END_VAR
    // Test Printer
    TestPrinter();

    // Print output
    PrintStatistics(3, ciArray);
END_FUNCTION
