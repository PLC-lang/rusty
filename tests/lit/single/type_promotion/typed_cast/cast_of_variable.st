// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A typed cast of a variable reads the variable in the target type instead of converting its value.
FUNCTION main : DINT
    VAR
        small : INT := -300;
        i : INT := 5;
        d : DINT := 7;
        s : SINT := -1;
        rd : DINT;
        rl : LINT;
        rr : LREAL;
        r4 : REAL;
    END_VAR

    // Widening to a larger integer
    rd := DINT#small;
    printf('DINT#(INT -300) = %d$N', rd);                   // CHECK: DINT#(INT -300) = -300
    rl := LINT#small;
    printf('LINT#(INT -300) = %lld$N', rl);                 // CHECK-NEXT: LINT#(INT -300) = -300
    rd := DINT#s;
    printf('DINT#(SINT -1) = %d$N', rd);                    // CHECK-NEXT: DINT#(SINT -1) = -1

    // Integer to real
    rr := LREAL#i;
    printf('LREAL#(INT 5) = %f$N', rr);                     // CHECK-NEXT: LREAL#(INT 5) = 5.000000
    r4 := REAL#d;
    printf('REAL#(DINT 7) = %f$N', r4);                     // CHECK-NEXT: REAL#(DINT 7) = 7.000000
END_FUNCTION
