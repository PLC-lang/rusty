// RUN: (%COMPILE %s && %RUN) | %CHECK %s

PROGRAM mainProg
VAR
    a, i, b : LINT;
    ret : DINT;
END_VAR

a := 16#FFFF_FFFF_FFFF_FFFF;
b := 16#FFFF_FFFF_FFFF_FFFF;
ret := 100;
FOR i := 1 TO 10 DO
    ret := ret + 1;
END_FOR
END_PROGRAM

FUNCTION main : DINT
    mainProg();

    printf('ret=%d$N', mainProg.ret); // CHECK: ret=110
    printf('a=%d$N', mainProg.a); // CHECK-NEXT: a=-1
    printf('b=%d$N', mainProg.b); // CHECK-NEXT: b=-1
    main := 0;
END_FUNCTION
