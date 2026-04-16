// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: FbB extends FbA which implements interface IA with a PROPERTY_GET+PROPERTY_SET property. FbB overrides
// only the setter (adds 1000). Dispatch through an IA variable should use FbA's inherited getter
// and FbB's overridden setter when pointing at FbB.

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
    // Override only setter — adds 1000 to distinguish
    PROPERTY value : DINT
        PROPERTY_SET value: DINT
            _val := value + 1000;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA : FbA;
        instanceB : FbB;
        iface : IValue;
    END_VAR

    // FbA — both accessors from FbA
    iface := instanceA;
    printf('value=%d$N', iface.value);      // CHECK: value=10
    iface.value := 42;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=42

    // FbB — getter inherited from FbA, setter overridden in FbB (+1000)
    iface := instanceB;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=10
    iface.value := 42;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=1042
END_FUNCTION
