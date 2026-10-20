// RUN: %COMPILE %s && %RUN | %CHECK %s
// A unary minus on a constant is folded by the constant evaluator. The folded value takes the
// type of the negation, the widened DINT, and not the width of the constant it came from, so a
// DINT initialized with `-c` on a USINT constant of 200 holds -200 and not a wrapped 56. A
// member of a program is the case that used to abort codegen with a mismatched initializer.

TYPE SRG : DINT(-2147483648..2147483647); END_TYPE
TYPE SB : BYTE(0..255); END_TYPE
TYPE MyL : LREAL := 1.1 + 2.2; END_TYPE
TYPE MyJ : DINT := 1.99999999; END_TYPE
TYPE S : STRUCT x : LREAL; END_STRUCT END_TYPE
TYPE Outer : STRUCT inner : S; arr : ARRAY[0..1] OF LREAL; END_STRUCT END_TYPE

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
    // value that no REAL holds exactly is the same in the constant and in a copy of it; a plain
    // real literal is a REAL and rounds to 123456792 first, as the assignment in a body does; a
    // subrange converts like its base type, and a real beyond any integer width wraps to zero
    cexact : DINT := LREAL#123456789.0;
    ccopy : DINT := cexact;
    creal : DINT := 123456789.0;
    cmax : DINT := LREAL#2147483647.0;
    csub : SRG := 3.0e9;
    csubx : SRG := LREAL#123456789.0;
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
    // a typed REAL literal holds REAL precision, and a sign over a REAL binary expression is
    // computed in REAL
    treal : DINT := REAL#1.99999999;
    tlreal : DINT := LREAL#1.99999999;
    rsum : LREAL := -(REAL#16777216.0 + REAL#1.0);
    // a plain real literal is a REAL, so it is rounded through REAL before an integer target
    // truncates it, as the same assignment in a body does
    plain : DINT := 1.99999999;
    negplain : DINT := -(1.99999999);
    // a value the evaluator folded itself keeps its own precision: an LREAL result into an
    // integer target truncates the exact value, as the body does
    lc : LREAL := 1.99999999;
    neglc : DINT := -lc;
    lsum : LINT := LREAL#123456789.0 + 0.0;
    // a binary expression of REAL operands is folded in REAL, as the body computes it
    bsum : DINT := REAL#16777216.0 + REAL#1.0;
    bsuml : LREAL := REAL#16777216.0 + REAL#1.0;
    psum : DINT := 1.99999999 + 0.0;
    // a plain real literal has the type the resolver gives it: the target's float type in a float
    // initializer, otherwise REAL; so an LREAL sum of literals keeps its precision, a REAL one
    // computes in REAL, and an integer target sees REAL arithmetic as the body does
    litsum : LREAL := 1.1 + 2.2;
    litsumr : REAL := 1.1 + 2.2;
    widesum : DINT := REAL#1.0 + 16777217;
    diffsum : DINT := 2.9999999 - 1.0;
    hugesum : LREAL := 1.0e40 + 1.0;
    // the declared float type reaches every literal of the initializer: nested operands, both
    // spellings of an array literal, a type default and a struct member assignment
    nested : LREAL := 3.14159265358979 * 2.0 / 4.0;
    listed : ARRAY[0..1] OF LREAL := [1.1 + 2.2, 1.1 + 2.2];
    multiplied : ARRAY[0..1] OF LREAL := [2(1.1 + 2.2)];
    tdef : MyL;
    member : S := (x := 1.1 + 2.2);
    // the default of a type is its initializer as the type's own constant expression folds it,
    // and a member of struct or array type hands its type on to the literals inside
    idef : MyJ;
    outer : Outer := (inner := (x := 1.1 + 2.2), arr := [1.1 + 2.2, 1.1 + 2.2]);
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
    printf('%d %d %d %d %d %lld$N', cexact, ccopy, creal, cmax, csub, chuge);  // CHECK: 123456789 123456789 123456792 2147483647 -1294967296 0
    printf('%d %d$N', csubx, csubc);  // CHECK: 123456789 123456789
    printf('%d %d %d$N', xr, yr, bnot);  // CHECK: 16777216 16777216 55
    printf('%d %d$N', snot, kinf);  // CHECK: -201 0
    printf('%d %d %.1f$N', treal, tlreal, rsum);  // CHECK: 2 1 -16777216.0
    printf('%d %d$N', plain, negplain);  // CHECK: 2 -2
    printf('%d %lld$N', neglc, lsum);  // CHECK: -1 123456789
    printf('%d %.1f %d$N', bsum, bsuml, psum);  // CHECK: 16777216 16777216.0 2
    printf('%.16f %.10f %d %d %.3e$N', litsum, litsumr, widesum, diffsum, hugesum);  // CHECK: 3.3000000000000003 3.3000001907 16777216 2 1.000e+40
    printf('%.16f %.16f %.16f %.16f %.16f$N', nested, listed[1], multiplied[1], tdef, member.x);  // CHECK: 1.5707963267948950 3.3000000000000003 3.3000000000000003 3.3000000000000003 3.3000000000000003
    printf('%d %.16f %.16f$N', idef, outer.inner.x, outer.arr[1]);  // CHECK: 2 3.3000000000000003 3.3000000000000003
END_FUNCTION
