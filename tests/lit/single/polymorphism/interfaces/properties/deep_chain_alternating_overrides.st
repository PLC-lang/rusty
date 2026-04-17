// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Three-level POU chain FbC → FbB → FbA, where FbA implements IValue. FbB overrides
// only the getter (adds 1000), FbC overrides only the setter (adds 2000). Dispatch through the
// interface should resolve each accessor to the most-derived override:
//   FbA → (FbA.get, FbA.set)
//   FbB → (FbB.get, FbA.set)
//   FbC → (FbB.get, FbC.set)

INTERFACE IValue
    PROPERTY_GET value: DINT END_GET
    PROPERTY_SET value: DINT END_SET
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IValue
    VAR
        _val : DINT := 10;
    END_VAR

    PROPERTY_GET value: DINT
        value := _val;
    END_GET

    PROPERTY_SET value: DINT
        _val := value;
    END_SET
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB EXTENDS FbA
    // Override only getter — adds 1000
    PROPERTY_GET value: DINT
        value := _val + 1000;
    END_GET
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbC EXTENDS FbB
    // Override only setter — adds 2000
    PROPERTY_SET value: DINT
        _val := value + 2000;
    END_SET
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceA : FbA;
        instanceB : FbB;
        instanceC : FbC;
        iface : IValue;
    END_VAR

    // FbA: get=FbA, set=FbA
    iface := instanceA;
    printf('value=%d$N', iface.value);      // CHECK: value=10
    iface.value := 50;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=50

    // FbB: get=FbB(+1000), set=FbA(inherited)
    iface := instanceB;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=1010
    iface.value := 50;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=1050

    // FbC: get=FbB(inherited, +1000), set=FbC(+2000)
    iface := instanceC;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=1010
    iface.value := 50;
    printf('value=%d$N', iface.value);      // CHECK-NEXT: value=3050
END_FUNCTION
