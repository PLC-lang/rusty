// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE str : STRUCT
    a : DINT := 10; b : DINT := 20; c : DINT := 30; d : DINT;
END_STRUCT END_TYPE

FUNCTION_BLOCK fb
VAR
    a : str;
END_VAR
END_FUNCTION_BLOCK

FUNCTION other : DINT
VAR
    x : fb;
END_VAR
VAR_INPUT
    index : INT;
END_VAR

    IF index = 0 THEN
        other := x.a.a;
    ELSIF index = 1 THEN
        other := x.a.b;
    ELSIF index = 2 THEN
        other := x.a.c;
    ELSE
        other := x.a.d;
    END_IF
END_FUNCTION

FUNCTION main : DINT
VAR
    a : DINT;
    b : DINT;
    c : DINT;
    d : DINT;
END_VAR
    a := other(index := 0);
    b := other(index := 1);
    c := other(index := 2);
    d := other(index := 3);

    printf('a=%d$N', a); // CHECK: a=10
    printf('b=%d$N', b); // CHECK: b=20
    printf('c=%d$N', c); // CHECK: c=30
    printf('d=%d$N', d); // CHECK: d=0
END_FUNCTION
