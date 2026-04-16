// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: A defines a method `doubleValue` that reads and writes a property internally.
// B extends A and overrides only the setter (adds 100). When `doubleValue` is called on a B
// instance through POINTER TO A, the read dispatches to A's getter but the write dispatches
// to B's overridden setter — proving that property dispatch inside a non-overridden method
// is truly dynamic.

FUNCTION_BLOCK A
    VAR
        _val : DINT := 5;
    END_VAR

    PROPERTY value : DINT
        PROPERTY_GET value: DINT
            value := _val;
        END_GET

        PROPERTY_SET value: DINT
            _val := value;
        END_SET
    END_PROPERTY

    METHOD doubleValue
        value := value * 2;
    END_METHOD

    METHOD printValue
        printf('value=%d$N', value);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK B EXTENDS A
    // Override only setter — adds 100
    PROPERTY value : DINT
        PROPERTY_SET value: DINT
            _val := value + 100;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION printViaPtr
    VAR_INPUT
        p : POINTER TO A;
    END_VAR

    p^.printValue();
END_FUNCTION

FUNCTION doubleViaPtr
    VAR_INPUT
        p : POINTER TO A;
    END_VAR

    p^.doubleValue();
END_FUNCTION

FUNCTION main
    VAR
        instanceA : A;
        instanceB : B;
    END_VAR

    // A — base: doubleValue reads 5, writes 10
    printViaPtr(ADR(instanceA));        // CHECK: value=5
    doubleViaPtr(ADR(instanceA));
    printViaPtr(ADR(instanceA));        // CHECK-NEXT: value=10
    doubleViaPtr(ADR(instanceA));
    printViaPtr(ADR(instanceA));        // CHECK-NEXT: value=20

    // B — inherited getter, overridden setter (+100)
    // doubleValue reads 5 (A.get), writes 10 → B.set stores 10+100=110
    printViaPtr(ADR(instanceB));        // CHECK-NEXT: value=5
    doubleViaPtr(ADR(instanceB));
    printViaPtr(ADR(instanceB));        // CHECK-NEXT: value=110
    // doubleValue reads 110 (A.get), writes 220 → B.set stores 220+100=320
    doubleViaPtr(ADR(instanceB));
    printViaPtr(ADR(instanceB));        // CHECK-NEXT: value=320
END_FUNCTION
