// RUN: %COMPILE %s && %RUN | %CHECK %s
// A unary minus on a constant is folded by the constant evaluator. The folded value takes the
// type of the negation, the widened DINT, and not the width of the constant it came from, so a
// DINT initialized with `-c` on a USINT constant of 200 holds -200 and not a wrapped 56. A
// member of a program is the case that used to abort codegen with a mismatched initializer.

VAR_GLOBAL CONSTANT
    c1 : USINT := 200;
    ci : INT := -32768;
END_VAR

VAR_GLOBAL
    g : DINT := -c1;
    gl : LINT := -ci;
END_VAR

PROGRAM prg
    VAR
        y : DINT := -c1;
        z : DINT := -ci;
    END_VAR
    printf('%d %d$N', y, z);        // CHECK: -200 32768
END_PROGRAM

FUNCTION main : DINT
    VAR
        l : DINT := -c1;
        n : DINT := -(c1 + 1);
    END_VAR
    prg();
    printf('%d %lld$N', g, gl);     // CHECK: -200 32768
    printf('%d %d$N', l, n);        // CHECK: -200 -201
END_FUNCTION
