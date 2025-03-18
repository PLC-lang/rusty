// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb
VAR
    x: INT := 42;
END_VAR

METHOD foo : INT
    VAR_INPUT
        a: INT;
    END_VAR

    foo := a + x;
END_METHOD 
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb2 EXTENDS fb
METHOD foo : INT
    VAR_INPUT
        a: INT := 69;
    END_VAR

    foo := a - x;
END_METHOD

    printf('%d$N', super^.foo(super^.x));   // CHECK: 84
    printf('%d$N', super^.foo(x));          // CHECK: 84
    // FIXME: need to check VAR_INPUT default initializer for "super-qualified" method calls. this returns 0 currently
    printf('%d$N', super^.foo());           // CHECK: 42
    printf('%d$N', foo(super^.x));          // CHECK: 0
    printf('%d$N', foo(x));                 // CHECK: 0
    printf('%d$N', foo());                  // CHECK: 27 
END_FUNCTION_BLOCK

FUNCTION main 
VAR
    inst: fb2;
END_VAR
    inst();
END_FUNCTION