// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: An interface output may be left out of a call. The call discards the output instead of
// wrapping the empty argument in a fat pointer.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
        printf('FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

VAR_GLOBAL
    instance : FbA;
END_VAR

FUNCTION consumer : DINT
    VAR_OUTPUT
        out1 : IA;
        out2 : IA;
    END_VAR

    out1 := instance;
    out2 := instance;
    consumer := 1;
END_FUNCTION

FUNCTION main
    VAR
        refIA : IA;
        result : DINT;
    END_VAR

    // CHECK: main: result = 1
    result := consumer(out1 =>, out2 =>);
    printf('main: result = %d$N', result);

    // CHECK-NEXT: FbA::foo
    // CHECK-NEXT: main: result = 1
    result := consumer(out1 => refIA, out2 =>);
    refIA.foo();
    printf('main: result = %d$N', result);
END_FUNCTION
