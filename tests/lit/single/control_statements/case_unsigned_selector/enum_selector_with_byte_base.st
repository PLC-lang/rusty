// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// An enum with the unsigned base type BYTE compares as unsigned in a CASE range, so the range
// Low..Mid covers the variants 10 and 150 on both sides of the midpoint 128.

TYPE Level : (Low := 10, Mid := 150, High := 200) BYTE; END_TYPE

FUNCTION classify : DINT
    VAR_INPUT
        x : Level;
    END_VAR
    CASE x OF
        Low..Mid: classify := 1;
        High:     classify := 2;
    ELSE
        classify := 0;
    END_CASE
END_FUNCTION

FUNCTION main
    printf('Low: %d$N', classify(Low));
    printf('Mid: %d$N', classify(Mid));
    printf('High: %d$N', classify(High));
    // CHECK:      Low: 1
    // CHECK-NEXT: Mid: 1
    // CHECK-NEXT: High: 2
END_FUNCTION
