// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION_BLOCK A
    METHOD printName: STRING
        printf('FB A$N');
    END_METHOD

    METHOD A1
        printf('  A::A1$N');
    END_METHOD

    METHOD A2
        printf('  A::A2$N');
    END_METHOD

    METHOD A3
        printf('  A::A3$N');
    END_METHOD

    METHOD A4
        printf('  A::A4$N');
    END_METHOD

    METHOD callMethods
        A1();
        A2();
        A3();
        A4();
    END_METHOD
END_FUNCTION_BLOCK

// B overriddes A2, introduces B1, B2 and B3
FUNCTION_BLOCK B EXTENDS A
    METHOD printName: STRING
        printf('FB B$N');
    END_METHOD

    METHOD A2
        printf('  B::A2 (overridden)$N');
    END_METHOD

    METHOD B1
        printf('  B::B1$N');
    END_METHOD

    METHOD B2
        printf('  B::B2$N');
    END_METHOD

    METHOD B3
        printf('  B::B3$N');
    END_METHOD
END_FUNCTION_BLOCK

// C overriddes A3, B2, introduces C1 and C2
FUNCTION_BLOCK C EXTENDS B
    METHOD printName: STRING
        printf('FB C$N');
    END_METHOD

    METHOD A3
        printf('  C::A3 (overridden)$N');
    END_METHOD

    METHOD B2
        printf('  C::B2 (overridden)$N');
    END_METHOD

    METHOD C1
        printf('  C::C1$N');
    END_METHOD

    METHOD C2
        printf('  C::C2$N');
    END_METHOD
END_FUNCTION_BLOCK

// D overriddes A4, B3, C2, introduces D1
FUNCTION_BLOCK DD EXTENDS C
    METHOD printName: STRING
        printf('FB D$N');
    END_METHOD

    METHOD A4
        printf('  D::A4 (overridden)$N');
    END_METHOD

    METHOD B3
        printf('  D::B3 (overridden)$N');
    END_METHOD

    METHOD C2
        printf('  D::C2 (overridden)$N');
    END_METHOD

    METHOD D1
        printf('  D::D1$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        index: INT;

        instanceA: A;
        instanceB: B;
        instanceC: C;
        instanceD: DD;

        instanceRefArrayA: ARRAY[1..4] OF POINTER TO A;
        instanceRefArrayB: ARRAY[1..3] OF POINTER TO B;
        instanceRefArrayC: ARRAY[1..2] OF POINTER TO C;
        instanceRefArrayD: ARRAY[1..1] OF POINTER TO DD;
    END_VAR

    // FIXME:
    // TODOs
    // - initializer in variable declaration not working, e.g. `instanceRefArrayD: ARRAY[1..1] OF POINTER TO D := [ADR(instanceD)]`
    // - initializer in body not working, e.g. `instanceRefArrayD := [ADR(instanceD)]`
    instanceA.__vtable := ADR(__vtable_A_instance);
    instanceB.__vtable := ADR(__vtable_B_instance);
    instanceC.__vtable := ADR(__vtable_C_instance);
    instanceD.__vtable := ADR(__vtable_DD_instance);

    // Initialize the arrays
    instanceRefArrayA[1] := ADR(instanceA);
    instanceRefArrayA[2] := ADR(instanceB);
    instanceRefArrayA[3] := ADR(instanceC);
    instanceRefArrayA[4] := ADR(instanceD);

    instanceRefArrayB[1] := ADR(instanceB);
    instanceRefArrayB[2] := ADR(instanceC);
    instanceRefArrayB[3] := ADR(instanceD);

    instanceRefArrayC[1] := ADR(instanceC);
    instanceRefArrayC[2] := ADR(instanceD);

    instanceRefArrayD[1] := ADR(instanceD);


    // CHECK: -- FUNCTION_BLOCK A and its children --
    // CHECK: FB A
    // CHECK:   A::A1
    // CHECK:   A::A2
    // CHECK:   A::A3
    // CHECK:   A::A4
    // CHECK: FB B
    // CHECK:   A::A1
    // CHECK:   B::A2 (overridden)
    // CHECK:   A::A3
    // CHECK:   A::A4
    // CHECK: FB C
    // CHECK:   A::A1
    // CHECK:   B::A2 (overridden)
    // CHECK:   C::A3 (overridden)
    // CHECK:   A::A4
    // CHECK: FB D
    // CHECK:   A::A1
    // CHECK:   B::A2 (overridden)
    // CHECK:   C::A3 (overridden)
    // CHECK:   D::A4 (overridden)
    printf('-- FUNCTION_BLOCK A and its children --$N');
    FOR index := 1 TO 4 DO
        instanceRefArrayA[index]^.printName();
        instanceRefArrayA[index]^.A1();
        instanceRefArrayA[index]^.A2();
        instanceRefArrayA[index]^.A3();
        instanceRefArrayA[index]^.A4();
    END_FOR 

    // CHECK: -- FUNCTION_BLOCK B and its children --
    // CHECK: FB B
    // CHECK:   A::A1
    // CHECK:   B::A2 (overridden)
    // CHECK:   A::A3
    // CHECK:   A::A4
    // CHECK:   B::B1
    // CHECK:   B::B2
    // CHECK:   B::B3
    // CHECK: FB C
    // CHECK:   A::A1
    // CHECK:   B::A2 (overridden)
    // CHECK:   C::A3 (overridden)
    // CHECK:   A::A4
    // CHECK:   B::B1
    // CHECK:   C::B2 (overridden)
    // CHECK:   B::B3
    // CHECK: FB D
    // CHECK:   A::A1
    // CHECK:   B::A2 (overridden)
    // CHECK:   C::A3 (overridden)
    // CHECK:   D::A4 (overridden)
    // CHECK:   B::B1
    // CHECK:   C::B2 (overridden)
    // CHECK:   D::B3 (overridden)
    printf('$N$N-- FUNCTION_BLOCK B and its children --$N');
    FOR index := 1 TO 3 DO
        instanceRefArrayB[index]^.printName();
        instanceRefArrayB[index]^.A1();
        instanceRefArrayB[index]^.A2();
        instanceRefArrayB[index]^.A3();
        instanceRefArrayB[index]^.A4();
        instanceRefArrayB[index]^.B1();
        instanceRefArrayB[index]^.B2();
        instanceRefArrayB[index]^.B3();
    END_FOR 

    // CHECK: -- FUNCTION_BLOCK C and its children --
    // CHECK: FB C
    // CHECK:   A::A1
    // CHECK:   B::A2 (overridden)
    // CHECK:   C::A3 (overridden)
    // CHECK:   A::A4
    // CHECK:   B::B1
    // CHECK:   C::B2 (overridden)
    // CHECK:   B::B3
    // CHECK:   C::C1
    // CHECK:   C::C2
    // CHECK: FB D
    // CHECK:   A::A1
    // CHECK:   B::A2 (overridden)
    // CHECK:   C::A3 (overridden)
    // CHECK:   D::A4 (overridden)
    // CHECK:   B::B1
    // CHECK:   C::B2 (overridden)
    // CHECK:   D::B3 (overridden)
    // CHECK:   C::C1
    // CHECK:   D::C2 (overridden)
    printf('$N$N-- FUNCTION_BLOCK C and its children --$N');
    FOR index := 1 TO 2 DO
        instanceRefArrayC[index]^.printName();
        instanceRefArrayC[index]^.A1();
        instanceRefArrayC[index]^.A2();
        instanceRefArrayC[index]^.A3();
        instanceRefArrayC[index]^.A4();
        instanceRefArrayC[index]^.B1();
        instanceRefArrayC[index]^.B2();
        instanceRefArrayC[index]^.B3();
        instanceRefArrayC[index]^.C1();
        instanceRefArrayC[index]^.C2();
    END_FOR 

    // CHECK: -- FUNCTION_BLOCK D and its children (it doesnt have any 😔) --
    // CHECK: FB D
    // CHECK:   A::A1
    // CHECK:   B::A2 (overridden)
    // CHECK:   C::A3 (overridden)
    // CHECK:   D::A4 (overridden)
    // CHECK:   B::B1
    // CHECK:   C::B2 (overridden)
    // CHECK:   D::B3 (overridden)
    // CHECK:   C::C1
    // CHECK:   D::C2 (overridden)
    // CHECK:   D::D1
    printf('$N$N-- FUNCTION_BLOCK D and its children (it doesnt have any 😔) --$N');
    FOR index := 1 TO 1 DO
        instanceRefArrayD[index]^.printName();
        instanceRefArrayD[index]^.A1();
        instanceRefArrayD[index]^.A2();
        instanceRefArrayD[index]^.A3();
        instanceRefArrayD[index]^.A4();
        instanceRefArrayD[index]^.B1();
        instanceRefArrayD[index]^.B2();
        instanceRefArrayD[index]^.B3();
        instanceRefArrayD[index]^.C1();
        instanceRefArrayD[index]^.C2();
        instanceRefArrayD[index]^.D1();
    END_FOR 
END_FUNCTION