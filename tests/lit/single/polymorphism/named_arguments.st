// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// !! INCOMPLETE !!
FUNCTION_BLOCK A
    VAR
        fooCounter: INT := 0;
    END_VAR

    METHOD foo: STRING
        fooCounter := fooCounter + 1;
        foo := CONCAT('called A::foo ', DINT_TO_STRING(fooCounter), ' times$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    VAR
        barCounter: INT := 0;
    END_VAR

    METHOD bar
        VAR_IN_OUT
            out: DINT;
        END_VAR

        barCounter := barCounter + 1;
        // out := CONCAT('called B::bar ', DINT_TO_STRING(barCounter), ' times$N');
        out := barCounter;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        result: DINT;

        instanceA: A;
        instanceB: B;
        refInstanceA: POINTER TO A;
        refInstanceB: POINTER TO B;
    END_VAR

    instanceB.bar(out => result);
    printf('result = %d$N', result);

    instanceA.__vtable := ADR(__vtable_A_instance);

    refInstanceA := ADR(instanceA);
    refInstanceA^.foo();
    printf('result = %d$N', result);

    refInstanceB := ADR(instanceB);
    refInstanceB^.foo();
    refInstanceB^.bar(out => result);
    printf('result = %d$N', result);
END_FUNCTION