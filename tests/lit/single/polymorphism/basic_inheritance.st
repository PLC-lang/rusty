// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    VAR
        nameA: STRING := 'A';
    END_VAR

    METHOD getName: STRING
        getName := nameA;
    END_METHOD

    METHOD printName
        printf('name = %s$N', ADR(getName()));
    END_METHOD

    METHOD printAncestors
        printf('ancestor = None$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    VAR
        nameB: STRING := 'B';
    END_VAR

    // OVERRIDE
    METHOD getName: STRING
        getName := nameB;
    END_METHOD

    METHOD printAncestors
        printf('ancestor = %s$N', ADR(SUPER^.getName()));
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK C EXTENDS B
    VAR
        nameC: STRING := 'C';
    END_VAR

    // OVERRIDE
    METHOD getName: STRING
        getName := nameC;
    END_METHOD

    METHOD printAncestors
        SUPER^.printAncestors();
        printf('ancestor = %s$N', ADR(SUPER^.getName()));
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK DD EXTENDS C
    VAR
        nameD: STRING := 'D';
    END_VAR

    // OVERRIDE
    METHOD getName: STRING
        getName := nameD;
    END_METHOD

    METHOD printAncestors
        SUPER^.printAncestors();
        printf('ancestor = %s$N', ADR(SUPER^.getName()));
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION printRefA
    VAR_INPUT
        refInstance: POINTER TO A;
    END_VAR

    refInstance^.printName();
    refInstance^.printAncestors();
END_FUNCTION

FUNCTION main
    VAR
        instanceA: A;
        instanceB: B;
        instanceC: C;
        instanceD: DD;

        refInstanceA: POINTER TO A;
        refInstanceB: POINTER TO B;
        refInstanceC: POINTER TO C;
        refInstanceD: POINTER TO DD;
    END_VAR

    // CHECK: name = A
    // CHECK: ancestor = None
    printRefA(ADR(instanceA));
    printf('$N');

    // CHECK: name = B
    // CHECK: ancestor = A
    printRefA(ADR(instanceB));
    printf('$N');

    // CHECK: name = C
    // CHECK: ancestor = A
    // CHECK: ancestor = B
    printRefA(ADR(instanceC));
    printf('$N');

    // CHECK: name = D
    // CHECK: ancestor = A
    // CHECK: ancestor = B
    // CHECK: ancestor = C
    printRefA(ADR(instanceD));
END_FUNCTION