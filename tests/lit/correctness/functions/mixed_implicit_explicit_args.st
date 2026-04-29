// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// Verify that mixing positional and named arguments compiles and produces
// correct results at runtime.

FUNCTION add3 : INT
VAR_INPUT
    a : INT;
    b : INT;
    c : INT;
END_VAR
    add3 := a * 100 + b * 10 + c;
END_FUNCTION

FUNCTION_BLOCK Counter
VAR_INPUT
    step  : INT;
    start : INT;
END_VAR
VAR_OUTPUT
    val : INT;
END_VAR
    val := start + step;
END_FUNCTION_BLOCK

FUNCTION main
VAR
    r1, r2, r3, r4, r5 : INT;
    fb : Counter;
END_VAR
    // positional before named
    r1 := add3(1, b := 20, c := 3);   // a=1  b=20 c=3  -> 123  wait, 1*100+20*10+3 = 303
    // named before positional
    r2 := add3(a := 10, 2, c := 3);   // a=10 b=2  c=3  -> 1023
    // named non-first param, rest positional
    r3 := add3(b := 20, 1, 3);        // a=1  b=20 c=3  -> 303
    // only one named, two positional
    r4 := add3(10, 2, c := 30);       // a=10 b=2  c=30 -> 1050
    // function block: positional input + named output
    fb(1, start := 10, val => r5);    // step=1 start=10 -> val=11

    printf('r1=%d$N', r1); // CHECK: r1=303
    printf('r2=%d$N', r2); // CHECK: r2=1023
    printf('r3=%d$N', r3); // CHECK: r3=303
    printf('r4=%d$N', r4); // CHECK: r4=1050
    printf('r5=%d$N', r5); // CHECK: r5=11
END_FUNCTION
