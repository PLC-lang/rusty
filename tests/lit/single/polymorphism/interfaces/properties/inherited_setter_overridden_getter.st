// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: FbB extends FbA which implements interface IA with a GET+SET property. FbB overrides
// only the getter (adds 1000). Dispatch through an IA variable should use FbB's overridden getter
// and FbA's inherited setter when pointing at FbB.

INTERFACE IValue
    PROPERTY value : DINT
        GET END_GET
        SET END_SET
    END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IValue
    VAR
        _val : DINT := 10;
    END_VAR

    PROPERTY value : DINT
        GET
            value := _val;
        END_GET

        SET
            _val := value;
        END_SET
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB EXTENDS FbA
    // Override only getter — adds 1000 to distinguish
    PROPERTY value : DINT
        GET
            value := _val + 1000;
        END_GET
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

    // FbB — getter overridden in FbB (+1000), setter inherited from FbA
    iface := instanceB;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=1010
    iface.value := 42;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=1042
END_FUNCTION
