// RUN: %COMPILE %s && %RUN | %CHECK %s
// A negated narrow constant is widened to DINT before the negation, so the folded value does not wrap.
VAR_GLOBAL CONSTANT
    cu : USINT := 200;
    ci : INT := -32768;
END_VAR

VAR_GLOBAL
    gu : DINT := -cu;
    gi : DINT := -ci;
END_VAR

PROGRAM prg
    VAR
        mu : DINT := -cu;
    END_VAR
    printf('member := -cu = %d$N', mu);
END_PROGRAM

FUNCTION main : DINT
    VAR CONSTANT
        lc : USINT := 200;
    END_VAR
    VAR
        lu : DINT := -cu;
        rd : DINT;
    END_VAR

    // Global and local initializers
    printf('global := -cu = %d$N', gu);     // CHECK: global := -cu = -200
    printf('global := -ci = %d$N', gi);     // CHECK-NEXT: global := -ci = 32768
    printf('local := -cu = %d$N', lu);      // CHECK-NEXT: local := -cu = -200
    prg();                                  // CHECK-NEXT: member := -cu = -200

    // Body assignments
    rd := -cu;
    printf('body -cu = %d$N', rd);          // CHECK-NEXT: body -cu = -200
    rd := -lc;
    printf('body -lc = %d$N', rd);          // CHECK-NEXT: body -lc = -200
END_FUNCTION
