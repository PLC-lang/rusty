// RUN: %COMPILE %s && %RUN | %CHECK %s
// The operand of a unary minus is widened to DINT before the negation, the same as the
// operands of a binary expression. Without the widening `-a` on a USINT is computed in
// 8 bits and wraps, and `-i` on the smallest INT stays the smallest INT.

TYPE SRU64 : ULINT(0..10); END_TYPE

VAR_GLOBAL CONSTANT
    ca : USINT := 200;
    csu : SRU64 := 10;
END_VAR

FUNCTION main : DINT
    VAR
        a : USINT := 200;
        i : INT := -32768;
        b : BYTE := 200;
        s : SINT := -128;
        u : UDINT := 3000000000;
        ul : ULINT := 1;
        li : LINT := -9223372036854775808;
        d : DINT;
        l : LINT;
        s2 : SINT;
        su : SRU64 := 10;
        lr : LREAL;
    END_VAR

    d := -a;
    printf('%d$N', d);          // CHECK: -200
    d := -i;
    printf('%d$N', d);          // CHECK: 32768
    d := 0 - a;
    printf('%d$N', d);          // CHECK: -200
    d := -b;
    printf('%d$N', d);          // CHECK: -200
    d := -s;
    printf('%d$N', d);          // CHECK: 128
    d := -ca;
    printf('%d$N', d);          // CHECK: -200
    d := -(a + 1);
    printf('%d$N', d);          // CHECK: -201
    d := -(-a);
    printf('%d$N', d);          // CHECK: 200

    // the negation of a value that is already DINT or wider keeps its width, as `0 - x` does
    l := -u;
    printf('%lld$N', l);        // CHECK: 1294967296
    l := -ul;
    printf('%lld$N', l);        // CHECK: -1
    l := -li;
    printf('%lld$N', l);        // CHECK: -9223372036854775808

    // the result moves to the target afterwards, so a narrow target still cuts it down
    s2 := -s;
    printf('%d$N', s2);         // CHECK: -128

    // a subrange of an unsigned 64 bit type is negated in LINT, as the plain type is, for a
    // variable and for a folded constant alike
    lr := -su;
    printf('%.1f$N', lr);       // CHECK: -10.0
    lr := -csu;
    printf('%.1f$N', lr);       // CHECK: -10.0
END_FUNCTION
