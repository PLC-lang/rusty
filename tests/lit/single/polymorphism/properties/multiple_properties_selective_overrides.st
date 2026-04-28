// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: A has two properties (x and y). B extends A and overrides only the getter of x
// (adds 100) and only the setter of y (adds 200). Dispatch through POINTER TO A verifies
// that each property's each accessor is resolved independently through the vtable.

FUNCTION_BLOCK A
    VAR
        _x : DINT := 1;
        _y : DINT := 2;
    END_VAR

    PROPERTY_GET x: DINT
        x := _x;
    END_PROPERTY

    PROPERTY_SET x: DINT
        _x := x;
    END_PROPERTY

    PROPERTY_GET y: DINT
        y := _y;
    END_PROPERTY

    PROPERTY_SET y: DINT
        _y := y;
    END_PROPERTY

    METHOD printXY
        printf('x=%d, y=%d$N', x, y);
    END_METHOD

    METHOD setX
        VAR_INPUT v : DINT; END_VAR
        x := v;
    END_METHOD

    METHOD setY
        VAR_INPUT v : DINT; END_VAR
        y := v;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    // Override only getter of x — adds 100
    PROPERTY_GET x: DINT
        x := _x + 100;
    END_PROPERTY

    // Override only setter of y — adds 200
    PROPERTY_SET y: DINT
        _y := y + 200;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION printViaPtr
    VAR_INPUT
        p : POINTER TO A;
    END_VAR

    p^.printXY();
END_FUNCTION

FUNCTION setXViaPtr
    VAR_INPUT
        p : POINTER TO A;
        v : DINT;
    END_VAR

    p^.setX(v);
END_FUNCTION

FUNCTION setYViaPtr
    VAR_INPUT
        p : POINTER TO A;
        v : DINT;
    END_VAR

    p^.setY(v);
END_FUNCTION

FUNCTION main
    VAR
        instanceA : A;
        instanceB : B;
    END_VAR

    // A: x.get=A, x.set=A, y.get=A, y.set=A
    printViaPtr(ADR(instanceA));                // CHECK: x=1, y=2
    setXViaPtr(ADR(instanceA), 10);
    setYViaPtr(ADR(instanceA), 20);
    printViaPtr(ADR(instanceA));                // CHECK-NEXT: x=10, y=20

    // B: x.get=B(+100), x.set=A(inherited), y.get=A(inherited), y.set=B(+200)
    printViaPtr(ADR(instanceB));                // CHECK-NEXT: x=101, y=2
    setXViaPtr(ADR(instanceB), 10);
    setYViaPtr(ADR(instanceB), 20);
    printViaPtr(ADR(instanceB));                // CHECK-NEXT: x=110, y=220
END_FUNCTION
