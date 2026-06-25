// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Regression: a REPEAT loop inside an action must resolve its `__ran_once_*` temporary (#1784).

PROGRAM mainProg
VAR
    acc : DINT;
END_VAR
    foo();
END_PROGRAM

ACTIONS
    ACTION foo
        acc := 0;
        REPEAT
            acc := acc + 1;
        UNTIL acc >= 3
        END_REPEAT
        printf('repeat=%d$N', acc); // CHECK: repeat=3
    END_ACTION
END_ACTIONS

FUNCTION main : DINT
    mainProg();
    main := 0;
END_FUNCTION
