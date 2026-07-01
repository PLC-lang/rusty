// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression: nested loops inside an action must re-initialize their inner temporaries (#1784).

PROGRAM mainProg
VAR
    acc : DINT;
    i : DINT;
    j : DINT;
END_VAR
    foo();
END_PROGRAM

ACTIONS
    ACTION foo
        acc := 0;
        FOR i := 1 TO 3 DO
            FOR j := 1 TO 3 DO
                acc := acc + 1;
            END_FOR
        END_FOR
        printf('nested=%d$N', acc); // CHECK: nested=9
    END_ACTION
END_ACTIONS

FUNCTION main : DINT
    mainProg();
    main := 0;
END_FUNCTION
