// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb
    METHOD foo : STRING
        VAR_INPUT
            x : DINT;
        END_VAR
        foo := 'hello';
    END_METHOD
END_FUNCTION_BLOCK
FUNCTION main : DINT
VAR
    my_fb : fb;
    res : STRING;
END_VAR
    res := my_fb.foo(x := 1);
    printf('%s$N', ADR(res)); // CHECK: hello
END_FUNCTION
