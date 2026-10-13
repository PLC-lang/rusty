// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: Surplus positional arguments bind to a sized variadic interface parameter, so a concrete
// instance is wrapped in a temporary fat pointer exactly like for a fixed interface parameter. The
// variadic is passed by reference, so it takes the exact interface and an interface variable passes
// through unchanged.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

INTERFACE IB EXTENDS IA
    METHOD bar
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
        printf('FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS IB
    METHOD foo
        printf('FbB::foo$N');
    END_METHOD

    METHOD bar
        printf('FbB::bar$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION consumer : DINT
    VAR_INPUT
        first : IA;
    END_VAR
    VAR_INPUT {ref}
        rest : {sized} IA...;
    END_VAR

    first.foo();
    consumer := 1;
END_FUNCTION

FUNCTION main
    VAR
        instanceA : FbA;
        instanceB : FbB;
        refIA : IA;
        result : DINT;
    END_VAR

    refIA := instanceB;

    // CHECK: FbA::foo
    // CHECK-NEXT: main: result = 1
    result := consumer(instanceA, instanceB, refIA);
    printf('main: result = %d$N', result);

    // CHECK-NEXT: FbB::foo
    // CHECK-NEXT: main: result = 1
    result := consumer(refIA, instanceA, instanceB);
    printf('main: result = %d$N', result);
END_FUNCTION
