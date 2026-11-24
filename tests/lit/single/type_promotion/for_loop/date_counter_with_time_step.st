// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
// A DATE counter computes its next value as a DATE, so a TIME step is an error, as in `d + T#1d`.
FUNCTION main : DINT
    VAR
        d : DATE;
        n : DINT;
    END_VAR

    FOR d := D#2020-01-01 TO D#2020-01-05 BY T#1d DO
        n := n + 1;
    END_FOR
END_FUNCTION

// CHECK: error[E156]: Operator `+` is not defined for `DATE` and `TIME`
