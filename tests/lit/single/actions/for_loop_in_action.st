// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression: a FOR loop inside an action must resolve its desugared temporaries (#1784).

PROGRAM mainProg
VAR
    acc : DINT;
    i : DINT;
END_VAR
    foo();
END_PROGRAM

ACTIONS
    ACTION foo
        acc := 0;
        FOR i := 1 TO 10 DO
            acc := acc + i;
        END_FOR
        printf('for=%d$N', acc); // CHECK: for=55
    END_ACTION
END_ACTIONS

FUNCTION main : DINT
    mainProg();
    main := 0;
END_FUNCTION
