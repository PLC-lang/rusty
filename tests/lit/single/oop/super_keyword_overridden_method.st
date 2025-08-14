// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb
VAR
    x: INT := 10;
END_VAR

METHOD foo : INT
    VAR_INPUT
        a: INT := 100;
    END_VAR

    foo := a + x;
END_METHOD 
END_FUNCTION_BLOCK

FUNCTION_BLOCK fb2 EXTENDS fb
// TODO: should shadowing parent variables with temp vars be allowed?
// uncommenting the following var-block will cause a compile error
// VAR_TEMP 
//     x: INT := 20;
// END_VAR
METHOD foo : INT
    VAR_INPUT
        a: INT := 1000;
    END_VAR

    foo := a - x;
END_METHOD

    printf('%d$N', super^.foo(super^.x));   // CHECK: 20
    printf('%d$N', super^.foo(x));          // CHECK: 20
    printf('%d$N', super^.foo());           // CHECK: 110
    printf('%d$N', foo(super^.x));          // CHECK: 0
    printf('%d$N', foo(x));                 // CHECK: 0
    printf('%d$N', foo());                  // CHECK: 990
END_FUNCTION_BLOCK

FUNCTION main 
VAR
    inst: fb2;
END_VAR
    inst();
END_FUNCTION