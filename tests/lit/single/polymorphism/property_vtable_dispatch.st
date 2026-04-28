// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: POU-based polymorphism (vtable dispatch) with properties. A method in the base FB
// accesses a property. When the property getter is overridden in a child FB, calling that method
// through a pointer-to-base should dispatch to the overridden getter via the vtable.

FUNCTION_BLOCK A
    VAR
        _val : DINT := 10;
    END_VAR

    PROPERTY_GET value: DINT
        value := _val;
    END_PROPERTY

    PROPERTY_SET value: DINT
        _val := value;
    END_PROPERTY

    METHOD printValue
        printf('value=%d$N', value);
    END_METHOD

    METHOD setValue
        VAR_INPUT x : DINT; END_VAR
        value := x;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    // Override getter — adds 1000 to distinguish from A's getter
    PROPERTY_GET value: DINT
        value := _val + 1000;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK C EXTENDS B
    // Override getter again — adds 2000
    PROPERTY_GET value: DINT
        value := _val + 2000;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION printViaPtr
    VAR_INPUT
        refInstance : POINTER TO A;
    END_VAR

    refInstance^.printValue();
END_FUNCTION

FUNCTION setViaPtr
    VAR_INPUT
        refInstance : POINTER TO A;
        x : DINT;
    END_VAR

    refInstance^.setValue(x);
END_FUNCTION

FUNCTION setDirectViaPtr
    VAR_INPUT
        refInstance : POINTER TO A;
        x : DINT;
    END_VAR

    refInstance^.value := x;
END_FUNCTION

FUNCTION main
    VAR
        instanceA : A;
        instanceB : B;
        instanceC : C;
    END_VAR

    // A — base getter
    printViaPtr(ADR(instanceA));        // CHECK: value=10

    // B — overridden getter adds 1000
    printViaPtr(ADR(instanceB));        // CHECK-NEXT: value=1010

    // C — overridden getter adds 2000
    printViaPtr(ADR(instanceC));        // CHECK-NEXT: value=2010

    // Setter through pointer on B (inherited from A), then read with overridden getter
    setViaPtr(ADR(instanceB), 42);
    printViaPtr(ADR(instanceB));        // CHECK-NEXT: value=1042

    // Setter through pointer on C, then read with C's getter
    setViaPtr(ADR(instanceC), 99);
    printViaPtr(ADR(instanceC));        // CHECK-NEXT: value=2099

    // Direct access on concrete instances still works
    printf('value=%d$N', instanceA.value);  // CHECK-NEXT: value=10
    printf('value=%d$N', instanceB.value);  // CHECK-NEXT: value=1042
    printf('value=%d$N', instanceC.value);  // CHECK-NEXT: value=2099

    // Direct property setter through pointer (not via method)
    setDirectViaPtr(ADR(instanceA), 77);
    printViaPtr(ADR(instanceA));            // CHECK-NEXT: value=77

    setDirectViaPtr(ADR(instanceB), 55);
    printViaPtr(ADR(instanceB));            // CHECK-NEXT: value=1055

    setDirectViaPtr(ADR(instanceC), 33);
    printViaPtr(ADR(instanceC));            // CHECK-NEXT: value=2033
END_FUNCTION
