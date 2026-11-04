// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: `REF=` binds the address of an interface variable through `REF_TO`, `POINTER TO` and
// `REFERENCE TO`, and calls reach the instance through each binding. A null binding `REF= 0` is
// accepted for each of them.

INTERFACE IA
    METHOD foo : DINT
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo : DINT
        foo := 7;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main : DINT
    VAR
        a : FbA;
        value : IA;
        p : REF_TO IA;
        q : POINTER TO IA;
        r : REFERENCE TO IA;
    END_VAR

    value := a;
    p REF= value;
    q REF= value;
    r REF= value;

    // CHECK: bound 7 7 7
    printf('bound %d %d %d$N', p^.foo(), q^.foo(), r.foo());

    p REF= 0;
    q REF= 0;
    r REF= 0;
END_FUNCTION
