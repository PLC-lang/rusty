// RUN: (%COMPILE %s && %RUN) | %CHECK %s
TYPE userType :
    DINT := 0;
END_TYPE

FUNCTION main : DINT
    VAR
        x : userType := 101;
        y : userType := 202;
        z : userType := 303;
        w : userType := 404;

        a : REFERENCE TO ARRAY [1..2] OF POINTER TO userType;
        b : ARRAY [1..2] OF POINTER TO userType;
        c : ARRAY [1..2] OF POINTER TO userType;
        d : ARRAY [1..2] OF POINTER TO userType;
    END_VAR

    // b starts as z/w, d starts as x/y
    b[1] := ADR(z);
    b[2] := ADR(w);
    c[1] := ADR(z);
    c[2] := ADR(w);
    d[1] := ADR(x);
    d[2] := ADR(y);

    // 1) REF= binds reference to target array variable (a -> d)
    a REF= d;

    // 2) b := a reads through reference and copies dereferenced array value
    b := a;

    // CHECK: b-after-b:=a 101 202
    printf('b-after-b:=a %d %d$N', b[1]^, b[2]^);

    // 3) a := c writes into referenced array (d := c)
    a := c;

    // CHECK: d-after-a:=c 303 404
    printf('d-after-a:=c %d %d$N', d[1]^, d[2]^);

    // Keep exact bug-report statements as compile-time/runtime no-op checks
    a REF= b;
    b := a;
    a := b;

    // CHECK: after-bug-sequence 101 202
    printf('after-bug-sequence %d %d$N', b[1]^, b[2]^);
END_FUNCTION
