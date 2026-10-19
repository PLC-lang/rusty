// RUN: %COMPILE %s && %RUN | %CHECK %s
// A unary minus on a constant is folded by the constant evaluator. The folded value takes the
// type of the negation, the widened DINT, and not the width of the constant it came from, so a
// DINT initialized with `-c` on a USINT constant of 200 holds -200 and not a wrapped 56. A
// member of a program is the case that used to abort codegen with a mismatched initializer.

TYPE SRG : DINT(-2147483648..2147483647); END_TYPE
TYPE SB : BYTE(0..255); END_TYPE

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
    // a reference to an integer constant is read in the constant's type through any chain
    cb : SINT := c1;
    cc : DINT := cb;
    cfrac : DINT := 1.5;
    cnf : DINT := -cfrac;
    // a body read of a constant uses its folded value and does not re-derive it from the source
    // initializer, so a chain from an integer constant initialized with an out-of-range real reads
    // the same value the static data holds
    cmid : DINT := 3.0e9;
    cmidl : LINT := cmid;
    // the converted integer is generated as an integer, not through the real it came from, so a
    // value that no REAL holds exactly is the same in the constant and in a copy of it; a subrange
    // converts like its base type, and a real beyond any integer width wraps to zero
    cexact : DINT := 123456789.0;
    ccopy : DINT := cexact;
    cmax : DINT := 2147483647.0;
    csub : SRG := 3.0e9;
    csubx : SRG := 123456789.0;
    csubc : DINT := csubx;
    chuge : LINT := 1.0e40;
    // a reference to a REAL constant is read with REAL precision, and a binary operand has the
    // promoted type of its operands, so NOT over two BYTEs is computed in BYTE
    xr : DINT := cr;
    yr : DINT := xr;
    bnot : DINT := NOT (BYTE#200 AND BYTE#255);
    // a subrange over a bit string has integer nature, so its binary expressions are computed in
    // UDINT like the resolver does; a REAL beyond f32 range counts as zero for an integer
    csb : SB := 200;
    snot : DINT := NOT (csb AND BYTE#255);
    rover : REAL := 3.5e38;
    kinf : DINT := rover;
END_VAR

VAR_GLOBAL
    // an integer constant initialized with an out-of-range real holds the truncated value cut
    // down to its type, and every read of it, static or in a body, sees that same value
    gmid : LINT := cmid;
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
        lmid : LINT := cmid;
        dmid : DINT;
    END_VAR
    prg();
    printf('%d %lld$N', g, gl);     // CHECK: -200 32768
    printf('%d %d$N', l, n);        // CHECK: -200 -201
    printf('%lld %.1f$N', gw, gr);  // CHECK: -2147483648 -16777216.0
    printf('%.1f %.1f %.1f %d$N', cf, cfr, cfi, cnot);  // CHECK: -200.0 -200.0 -5.0 -55
    printf('%.1f %lld %d %u %d$N', cfu, clu, cz, cud, csn);  // CHECK: 200.0 200 56 2147483648 55
    printf('%d %d %d$N', cb, cc, cnf);  // CHECK: -56 -56 -1
    printf('%lld$N', cmidl);  // CHECK: -1294967296
    dmid := cmid;
    printf('%d %lld %lld$N', dmid, lmid, gmid);  // CHECK: -1294967296 -1294967296 -1294967296
    printf('%d %d %d %d %lld$N', cexact, ccopy, cmax, csub, chuge);  // CHECK: 123456789 123456789 2147483647 -1294967296 0
    printf('%d %d$N', csubx, csubc);  // CHECK: 123456789 123456789
    printf('%d %d %d$N', xr, yr, bnot);  // CHECK: 16777216 16777216 55
    printf('%d %d$N', snot, kinf);  // CHECK: -201 0
END_FUNCTION
