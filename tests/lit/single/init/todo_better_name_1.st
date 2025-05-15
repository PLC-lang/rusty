// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    globalA: MyStruct2 := (a := (c := 5, b := 7), b := (a := 3, b := 2));
    globalB: MyStruct2 := (b := (a := 9));
END_VAR

TYPE MyStruct2: STRUCT
    a: MyStruct := (a := 5, b := 3);
    b: MyStruct := (c := 7);
END_STRUCT
END_TYPE

TYPE MyStruct: STRUCT
    a: DINT;
    b: DINT;
    c: DINT;
END_STRUCT
END_TYPE

FUNCTION main
    printf('%d$N', globalA.a.a); // CHECK: 5
    printf('%d$N', globalA.a.b); // CHECK: 7
    printf('%d$N', globalA.a.c); // CHECK: 5
    printf('%d$N', globalA.b.a); // CHECK: 3
    printf('%d$N', globalA.b.b); // CHECK: 2
    printf('%d$N', globalA.b.c); // CHECK: 7

    printf('%d$N', globalB.b.a); // CHECK: 9
    printf('%d$N', globalB.b.b); // CHECK: 0
    printf('%d$N', globalB.b.c); // CHECK: 7
END_FUNCTION