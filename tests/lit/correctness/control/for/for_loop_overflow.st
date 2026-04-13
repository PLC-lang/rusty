// RUN: (%COMPILE %s && %RUN) | %CHECK %s

PROGRAM mainProg
VAR
    i : INT;
    ret : DINT;
END_VAR

ret := 100;
FOR i := 1 TO 0 BY -2 DO
    ret := ret + 1;
END_FOR
END_PROGRAM

FUNCTION main : DINT
    mainProg();

    printf('ret=%d$N', mainProg.ret); // CHECK: ret=101
    printf('i=%d$N', mainProg.i); // CHECK-NEXT: i=-1
    main := 0;
END_FUNCTION
