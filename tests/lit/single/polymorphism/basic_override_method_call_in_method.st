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
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    VAR
        nameB: STRING := 'B';
    END_VAR

    // OVERRIDE
    METHOD getName: STRING
        getName := nameB;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA: A;
        instanceB: B;
        refInstanceA: POINTER TO A;
    END_VAR

    // CHECK: name = A
    refInstanceA := ADR(instanceA);
    refInstanceA^.printName();

    // CHECK: name = B
    refInstanceA := ADR(instanceB);
    refInstanceA^.printName();
END_FUNCTION