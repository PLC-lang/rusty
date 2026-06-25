// RUN: (%COMPILE %s && %RUN) | %CHECK %s
//
// Regression + integration coverage for loops nested inside ACTIONS.
//
// A loop body is desugared (in the plc_lowering pass) into a `WHILE TRUE` loop
// with synthetic temporaries such as `__ran_once_*` and `__is_incrementing_*`.
// Those are realized as entry-block locals registered under the parent POU's
// *type name* (`mainProg`), whereas an action's call name is `mainProg.<action>`.
// If the resolver qualifies the temporaries with the call name instead of the
// type name they fail to resolve during codegen:
//
//     error: Could not resolve reference to __is_incrementing_0
//
// Each action below must therefore compile *and* produce the correct runtime
// result while reading and writing the parent program's members.

PROGRAM mainProg
VAR
    acc : DINT;
    i : DINT;
    j : DINT;
END_VAR
    runFor();
    runWhile();
    runRepeat();
    runNested();
END_PROGRAM

ACTIONS
    // Base case: the FOR loop from the original report. 1 + 2 + ... + 10 = 55.
    ACTION runFor
        acc := 0;
        FOR i := 1 TO 10 DO
            acc := acc + i;
        END_FOR
        printf('for=%d$N', acc); // CHECK: for=55
    END_ACTION

    // WHILE lowers to `WHILE TRUE` with a negated-condition guard (no temporaries),
    // but still reads/writes the parent's members. Runs the body five times.
    ACTION runWhile
        acc := 0;
        i := 0;
        WHILE i < 5 DO
            acc := acc + 1;
            i := i + 1;
        END_WHILE
        printf('while=%d$N', acc); // CHECK: while=5
    END_ACTION

    // REPEAT introduces a single `__ran_once_*` temporary gating the UNTIL check.
    ACTION runRepeat
        acc := 0;
        REPEAT
            acc := acc + 1;
        UNTIL acc >= 3
        END_REPEAT
        printf('repeat=%d$N', acc); // CHECK: repeat=3
    END_ACTION

    // Nested loops: the inner temporaries must be re-initialized on every outer
    // iteration. 3 * 3 = 9.
    ACTION runNested
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
