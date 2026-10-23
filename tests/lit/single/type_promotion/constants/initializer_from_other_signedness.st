// RUN: %COMPILE %s && %RUN | %CHECK %s
// XFAIL: *
// A constant initialized from a constant of other signedness is extended by the signedness of the target, not of the source.
VAR_GLOBAL CONSTANT
    cu : USINT := 200;
    k : INT := cu;
    cs : SINT := -1;
    u : ULINT := cs;
    d : DINT := cs;
END_VAR

VAR_GLOBAL
    g : INT := cu;
END_VAR

FUNCTION main : DINT
    VAR CONSTANT
        lk : INT := cu;
    END_VAR
    VAR
        t : INT := USINT#200;
    END_VAR

    printf('variable INT := USINT 200 = %d$N', g);          // CHECK: variable INT := USINT 200 = 200
    printf('constant INT := USINT 200 = %d$N', k);          // CHECK-NEXT: constant INT := USINT 200 = 200
    printf('local constant INT := USINT 200 = %d$N', lk);   // CHECK-NEXT: local constant INT := USINT 200 = 200
    printf('INT := USINT#200 = %d$N', t);                   // CHECK-NEXT: INT := USINT#200 = 200
    printf('constant ULINT := SINT -1 = %llu$N', u);        // CHECK-NEXT: constant ULINT := SINT -1 = 18446744073709551615
    printf('constant DINT := SINT -1 = %d$N', d);           // CHECK-NEXT: constant DINT := SINT -1 = -1
END_FUNCTION
