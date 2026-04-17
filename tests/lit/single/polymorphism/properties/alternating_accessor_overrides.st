// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Three-level POU chain C → B → A. B overrides only the getter (adds 1000),
// C overrides only the setter (adds 2000). Dispatch through POINTER TO A should resolve
// each accessor independently to the most-derived override:
//   A → (A.get, A.set)
//   B → (B.get, A.set)
//   C → (B.get, C.set)

FUNCTION_BLOCK A
    VAR
        _val : DINT := 10;
    END_VAR

    PROPERTY_GET value: DINT
        value := _val;
    END_GET

    PROPERTY_SET value: DINT
        _val := value;
    END_SET

    METHOD printValue
        printf('value=%d$N', value);
    END_METHOD

    METHOD setValue
        VAR_INPUT x : DINT; END_VAR
        value := x;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    // Override only getter — adds 1000
    PROPERTY_GET value: DINT
        value := _val + 1000;
    END_GET
END_FUNCTION_BLOCK

FUNCTION_BLOCK C EXTENDS B
    // Override only setter — adds 2000
    PROPERTY_SET value: DINT
        _val := value + 2000;
    END_SET
END_FUNCTION_BLOCK

FUNCTION printViaPtr
    VAR_INPUT
        p : POINTER TO A;
    END_VAR

    p^.printValue();
END_FUNCTION

FUNCTION setViaPtr
    VAR_INPUT
        p : POINTER TO A;
        x : DINT;
    END_VAR

    p^.setValue(x);
END_FUNCTION

FUNCTION main
    VAR
        instanceA : A;
        instanceB : B;
        instanceC : C;
    END_VAR

    // A: get=A, set=A
    printViaPtr(ADR(instanceA));        // CHECK: value=10
    setViaPtr(ADR(instanceA), 50);
    printViaPtr(ADR(instanceA));        // CHECK-NEXT: value=50

    // B: get=B(+1000), set=A(inherited)
    printViaPtr(ADR(instanceB));        // CHECK-NEXT: value=1010
    setViaPtr(ADR(instanceB), 50);
    printViaPtr(ADR(instanceB));        // CHECK-NEXT: value=1050

    // C: get=B(inherited, +1000), set=C(+2000)
    printViaPtr(ADR(instanceC));        // CHECK-NEXT: value=1010
    setViaPtr(ADR(instanceC), 50);
    printViaPtr(ADR(instanceC));        // CHECK-NEXT: value=3050
END_FUNCTION
