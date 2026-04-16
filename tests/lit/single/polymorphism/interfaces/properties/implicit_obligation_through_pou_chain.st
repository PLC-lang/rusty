// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: FbB extends FbA (which implements IValue) without overriding anything. FbC extends FbB,
// also without overriding. All three should be assignable to an IValue variable and dispatch
// correctly to FbA's inherited property accessors.

INTERFACE IValue
    PROPERTY value : DINT
        PROPERTY_GET value: DINT END_GET
        PROPERTY_SET value: DINT END_SET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IValue
    VAR
        _val : DINT := 10;
    END_VAR

    PROPERTY value : DINT
        PROPERTY_GET value: DINT
            value := _val;
        END_GET

        PROPERTY_SET value: DINT
            _val := value;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB EXTENDS FbA
    // No override — inherits everything from FbA
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbC EXTENDS FbB
    // No override — inherits transitively from FbA
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA : FbA;
        instanceB : FbB;
        instanceC : FbC;
        iface : IValue;
    END_VAR

    // FbA — direct implementor
    iface := instanceA;
    printf('value=%d$N', iface.value);      // CHECK: value=10
    iface.value := 42;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=42

    // FbB — inherits from FbA (one level)
    iface := instanceB;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=10
    iface.value := 77;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=77

    // FbC — inherits transitively through FbB (two levels)
    iface := instanceC;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=10
    iface.value := 99;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=99
END_FUNCTION
