// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    METHOD foo
        printf('A::foo$N');
    END_METHOD

    METHOD bar
        printf('A::bar$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    METHOD foo
        printf('B::foo (overridden)$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION operateOnA
    VAR_INPUT
        in: POINTER TO A;
    END_VAR

    in^.foo();
    in^.bar();
END_FUNCTION

FUNCTION main
    VAR
        index: INT;
        instanceA: A;
        instanceB: B;
        instanceRefA: POINTER TO A;
        instanceArrayRefA: ARRAY[1..2] OF POINTER TO A;
    END_VAR

    // FIXME: Missing initialization, these should be part of the __init_<POU>; fix at a later time
    instanceA.__vtable := ADR(__vtable_A_instance);
    instanceB.__vtable := ADR(__vtable_B_instance);

    // ~~ Direct calls ~~
    // CHECK: A::foo
    // CHECK: A::bar
    // CHECK: B::foo (overridden)
    // CHECK: A::bar

    instanceRefA := ADR(instanceA);
    instanceRefA^.foo();
    instanceRefA^.bar();

    instanceRefA := ADR(instanceB);
    instanceRefA^.foo();
    instanceRefA^.bar();

    // ~~ Indirect calls through arrays ~~
    // CHECK: B::foo (overridden)
    // CHECK: A::bar
    // CHECK: A::foo
    // CHECK: A::bar

    instanceArrayRefA[1] := ADR(instanceB);
    instanceArrayRefA[2] := ADR(instanceA);
    FOR index := 1 TO 2 DO
        instanceArrayRefA[index]^.foo();
        instanceArrayRefA[index]^.bar();
    END_FOR

    // ~~ Indirect calls through functions ~~
    // CHECK: A::foo
    // CHECK: A::bar
    // CHECK: B::foo (overridden)
    // CHECK: A::bar

    instanceRefA := ADR(instanceA);
    operateOnA(instanceRefA);

    instanceRefA := ADR(instanceB);
    operateOnA(instanceRefA);

END_FUNCTION