// RUN: %COMPILE %s && %RUN | %CHECK %s
// The step is evaluated once per iteration, so the counter value that the loop tests is the value that the body sees.
VAR_GLOBAL
    calls : DINT;
    step : DINT;
END_VAR

FUNCTION next_step : DINT
    calls := calls + 1;
    next_step := calls;
END_FUNCTION

FUNCTION end_changes_step : DINT
    step := step + 1;
    end_changes_step := 2;
END_FUNCTION

FUNCTION main : DINT
    VAR
        i : DINT;
    END_VAR

    // The direction test reads 1, then the passes read 2 and 3
    calls := 0;
    FOR i := 0 TO 2 BY next_step() DO
        printf('BY next_step(): i = %d$N', i);
    END_FOR
    printf('BY next_step(): after = %d, calls = %d$N', i, calls);
    // CHECK:      BY next_step(): i = 0
    // CHECK-NEXT: BY next_step(): i = 2
    // CHECK-NEXT: BY next_step(): after = 5, calls = 3

    // The end value changes the step after the step is read
    step := 1;
    FOR i := 0 TO end_changes_step() BY step DO
        printf('TO end_changes_step(): i = %d$N', i);
    END_FOR
    printf('TO end_changes_step(): after = %d$N', i);
    // CHECK-NEXT: TO end_changes_step(): i = 0
    // CHECK-NEXT: TO end_changes_step(): i = 2
    // CHECK-NEXT: TO end_changes_step(): after = 5
END_FUNCTION
