// RUN: %COMPILE %s 2> %t.err && %RUN | %CHECK %s
// RUN: not grep -E "Implicit conversion from|Operand of type" %t.err
// Explicit conversions, standard date and time arithmetic, builtins, generics and compiler-generated code
// never report an implicit conversion.
INTERFACE Source
    METHOD next : UDINT END_METHOD
END_INTERFACE

FUNCTION_BLOCK Counter IMPLEMENTS Source
    VAR count : UDINT; END_VAR
    METHOD next : UDINT
        count := count + 1;
        next := count;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Wide EXTENDS Counter
END_FUNCTION_BLOCK

FUNCTION greeting : STRING
    greeting := 'hello';
END_FUNCTION

FUNCTION main
    VAR
        vSint : SINT := -3;
        vDint : DINT := 5;
        vLint : LINT := 6;
        vUdint : UDINT := 7;
        vUlint : ULINT := 8;
        vReal : REAL := 2.5;
        vTime : TIME := T#1s;
        vTod : TOD := TOD#01:00:00;
        vDt : DT := DT#2020-01-01-00:00:00;
        vDate : DATE := D#2020-01-02;
        counter : Counter;
        wide : Wide;
        source : Source;
        text : STRING;
    END_VAR

    // explicit conversions
    vUdint := TO_UDINT(vDint);
    vUlint := TO_ULINT(vLint);
    vReal := TO_REAL(vDint);
    vLint := TO_LINT(vReal);
    printf('%u %lu %f %ld$N', vUdint, vUlint, vReal, vLint);   // CHECK: 5 6 5.000000 5

    // counting idioms
    vUdint := vUdint + 1;
    vUdint := vUdint * 2;
    vUdint := vUdint - 1;
    printf('%u$N', vUdint);                                    // CHECK: 11

    // date and time arithmetic of the standard
    vTime := vTime + vTime;
    vTod := vTod + vTime;
    vDt := vDt + vTime;
    vTime := vDate - vDate;
    IF vTime = T#0s THEN
        printf('zero$N');                                      // CHECK: zero
    END_IF

    // generics and builtins
    vUdint := MAX(vUdint, vUdint);
    vUdint := SEL(TRUE, vUdint, 5);
    printf('%u$N', vUdint);                                    // CHECK: 5

    // generated code: constructors, vtables, aggregate returns, interface calls
    source := counter;
    vUdint := source.next();
    vUdint := wide.next();
    text := greeting();
    printf('%u %s$N', vUdint, text);                           // CHECK: 1 hello
END_FUNCTION
