// RUN: %COMPILE_IR %s | %CHECK_IR %s
//
// A unary minus on a constant is folded by the constant evaluator. The folded literal is typed
// by the negation, which is widened to DINT, and not by the constant it came from. Without this
// the initializer of a program member was an i8 inside an i32 slot, which LLVM rejects.

TYPE SRd : DINT(-2147483648..0); END_TYPE
TYPE ED : DINT (DA := -2147483648, DB := 0); END_TYPE

VAR_GLOBAL CONSTANT
    c1 : USINT := 200;
    ci : INT := -32768;
    cs : SINT := -128;
    cb : SINT := c1;
    cd : DINT := -2147483648;
    cr : REAL := 16777217.0;
    csrd : SRd := -2147483648;
    ced : ED := ED#DA;
END_VAR

VAR_GLOBAL
    g : DINT := -c1;
    gl : LINT := -ci;
    // the folding wraps and rounds in the type of the negation, as the runtime does; the double
    // is -16777216.0 in LLVM's hexadecimal notation
    gw : LINT := -cd;
    gr : LREAL := -cr;
    gp : LREAL := +cr;
    // a subrange and an enum are computed in their base type, a nested sign in the type of the
    // innermost operand
    gs : LINT := -csrd;
    ge : LINT := -ced;
    gn : LINT := -(-cd);
    // NOT keeps the width of its operand, and a real constant holds the negated value
    gnot : DINT := -(NOT c1);
    gnn : USINT := NOT (NOT c1);
    gnneg : DINT := NOT (-cs);
    // the operand is read in its own type, so the SINT holding the raw 200 is -56
    gnarrow : DINT := -cb;
    gf : LREAL := -c1;
    // a constant built from an unsigned constant keeps its value in the static data too, and a
    // negation into a narrower target is cut down to that target
    gfu : LREAL := c1;
    glu : LINT := c1;
    gz : SINT := -c1;
    gud : UDINT := -cd;
END_VAR

PROGRAM prg
    VAR
        y : DINT := -c1;
        z : DINT := -ci;
        // a unary plus hands its operand's literal on with its own id, so the member's hint applies
        w : INT := +(-ci);
    END_VAR
END_PROGRAM

// CHECK: @g = global i32 -200
// CHECK: @gl = global i64 32768
// CHECK: @gw = global i64 -2147483648
// CHECK: @gr = global double 0xC170000000000000
// CHECK: @gp = global double 0x4170000000000000
// CHECK: @gs = global i64 -2147483648
// CHECK: @ge = global i64 -2147483648
// CHECK: @gn = global i64 -2147483648
// CHECK: @gnot = global i32 -55
// CHECK: @gnn = global i8 -56
// CHECK: @gnneg = global i32 -129
// CHECK: @gnarrow = global i32 56
// CHECK: @gf = global double -2.000000e+02
// CHECK: @gfu = global double 2.000000e+02
// CHECK: @glu = global i64 200
// CHECK: @gz = global i8 56
// CHECK: @gud = global i32 -2147483648
// CHECK: @prg_instance = global %prg { i32 -200, i32 32768, i16 -32768 }
