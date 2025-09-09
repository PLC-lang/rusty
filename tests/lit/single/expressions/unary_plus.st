// RUN: (%COMPILE %s && %RUN) | %CHECK %s

VAR_GLOBAL CONSTANT g1 : INT := +5; END_VAR

PROGRAM prog
VAR
    x : INT := +g1;
END_VAR
    // CHECK: 5
    printf('%d$N', x);
    +x;
    x := +x + 4;
    // CHECK: 9
    printf('%d$N', x);
    x := +-4 + 5;
    // CHECK: 1
    printf('%d$N', x);
    +-x;
    x := +foo(+x);
    // CHECK: 3
    printf('%d$N', x);
END_PROGRAM

FUNCTION foo : INT
VAR_INPUT
    x : INT;
END_VAR
    foo := +x + +2 - +(+x - +1);
END_FUNCTION

FUNCTION main
    prog();
END_FUNCTION
