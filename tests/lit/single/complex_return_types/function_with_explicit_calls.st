// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION foo : STRING
        VAR_INPUT
            x : DINT;
        END_VAR
        foo := 'hello';
END_FUNCTION
FUNCTION main : DINT
VAR
    res : STRING;
END_VAR
    res := foo(x := 1);
    printf('%s$N', ADR(res)); // CHECK: hello
END_FUNCTION
