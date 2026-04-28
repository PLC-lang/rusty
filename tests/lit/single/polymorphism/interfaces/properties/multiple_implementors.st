// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Two function blocks implement the same interface property with different backing logic.
// FbPlain returns the stored value directly, FbDoubling returns twice the stored value. Assigning
// each to the same interface variable and reading the property verifies that dispatch picks the
// correct accessor implementation each time.

INTERFACE IValue
    PROPERTY_GET value: DINT END_PROPERTY
    PROPERTY_SET value: DINT END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbPlain IMPLEMENTS IValue
    VAR
        _val : DINT := 10;
    END_VAR

    PROPERTY_GET value: DINT
        value := _val;
    END_PROPERTY

    PROPERTY_SET value: DINT
        _val := value;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbDoubling IMPLEMENTS IValue
    VAR
        _val : DINT := 10;
    END_VAR

    PROPERTY_GET value: DINT
        value := _val * 2;
    END_PROPERTY

    PROPERTY_SET value: DINT
        _val := value;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        plain : FbPlain;
        doubling : FbDoubling;
        iface : IValue;
    END_VAR

    // FbPlain — getter returns _val directly
    iface := plain;
    printf('value=%d$N', iface.value);      // CHECK: value=10
    iface.value := 42;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=42

    // FbDoubling — getter returns _val * 2
    iface := doubling;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=20
    iface.value := 42;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=84
END_FUNCTION
