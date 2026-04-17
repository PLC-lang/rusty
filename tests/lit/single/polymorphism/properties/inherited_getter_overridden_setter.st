// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: B extends A which has a property with both PROPERTY_GET and PROPERTY_SET. B overrides only the setter.
// Dispatch through POINTER TO A should use A's inherited getter and B's overridden setter.

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
    // Override only the setter — adds 1000 to distinguish from A's setter
    PROPERTY_SET value: DINT
        _val := value + 1000;
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
    END_VAR

    // A — base getter and setter
    printViaPtr(ADR(instanceA));        // CHECK: value=10
    setViaPtr(ADR(instanceA), 42);
    printViaPtr(ADR(instanceA));        // CHECK-NEXT: value=42

    // B — inherited getter, overridden setter (adds 1000)
    printViaPtr(ADR(instanceB));        // CHECK-NEXT: value=10
    setViaPtr(ADR(instanceB), 42);
    printViaPtr(ADR(instanceB));        // CHECK-NEXT: value=1042
END_FUNCTION
