// RUN: %COMPILE %s && %RUN | %CHECK %s
// A unary minus on a constant is folded by the constant evaluator. The folded value takes the
// type of the negation, the widened DINT, and not the width of the constant it came from, so a
// DINT initialized with `-c` on a USINT constant of 200 holds -200 and not a wrapped 56. A
// member of a program is the case that used to abort codegen with a mismatched initializer.

VAR_GLOBAL CONSTANT
    c1 : USINT := 200;
    ci : INT := -32768;
    cd : DINT := -2147483648;
    cr : REAL := 16777217.0;
    c5 : INT := -5;
    // a use of a constant inlines its folded value, converted to the constant's type: an integer
    // folded into a real constant is a signed conversion, and NOT keeps the width of its operand
    cf : LREAL := -c1;
    cfr : REAL := -c1;
    cfi : LREAL := c5;
    cnot : DINT := -(NOT c1);
    // a constant built from an unsigned constant keeps its value, and a negation into a narrower
    // target is cut down to that target as the assignment in a body is
    cfu : LREAL := c1;
    clu : LINT := c1;
    cz : SINT := -c1;
    cud : UDINT := -cd;
    csn : SINT := NOT c1;
END_VAR

VAR_GLOBAL
    g : DINT := -c1;
    gl : LINT := -ci;
    // the folding wraps and rounds in the type of the negation, as the runtime does: the negation
    // of the smallest DINT is computed in DINT and wraps, and a REAL has no room for 16777217
    gw : LINT := -cd;
    gr : LREAL := -cr;
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
    printf('%lld %.1f$N', gw, gr);  // CHECK: -2147483648 -16777216.0
    printf('%.1f %.1f %.1f %d$N', cf, cfr, cfi, cnot);  // CHECK: -200.0 -200.0 -5.0 -55
    printf('%.1f %lld %d %u %d$N', cfu, clu, cz, cud, csn);  // CHECK: 200.0 200 56 2147483648 55
END_FUNCTION
