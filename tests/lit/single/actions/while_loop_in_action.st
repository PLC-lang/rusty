// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression: a WHILE loop inside an action must resolve against the parent POU's members (#1784).

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
        i := 0;
        WHILE i < 5 DO
            acc := acc + 1;
            i := i + 1;
        END_WHILE
        printf('while=%d$N', acc); // CHECK: while=5
    END_ACTION
END_ACTIONS

FUNCTION main : DINT
    mainProg();
    main := 0;
END_FUNCTION
