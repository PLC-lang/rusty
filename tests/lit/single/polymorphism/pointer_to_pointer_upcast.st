// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    METHOD getName: STRING
        getName := 'A';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    METHOD getName: STRING
        getName := 'B';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK C EXTENDS B
    METHOD getName: STRING
        getName := 'C';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: A;
        instanceB: B;
        instanceC: C;

        ptrA: POINTER TO A;
        ptrB: POINTER TO B;
        ptrC: POINTER TO C;
    END_VAR

    // Pointer-to-pointer upcast: assign child pointer to base pointer
    ptrB := ADR(instanceB);
    ptrA := ptrB;
    // CHECK: name = B
    printf('name = %s$N', ADR(ptrA^.getName()));

    ptrC := ADR(instanceC);
    ptrA := ptrC;
    // CHECK: name = C
    printf('name = %s$N', ADR(ptrA^.getName()));

    // Transitive: ptrC -> ptrB -> ptrA
    ptrC := ADR(instanceC);
    ptrB := ptrC;
    ptrA := ptrB;
    // CHECK: name = C
    printf('name = %s$N', ADR(ptrA^.getName()));
END_FUNCTION
