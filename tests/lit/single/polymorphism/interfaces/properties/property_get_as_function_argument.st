// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Passing a property getter result as an argument to a function, dispatched through
// an interface reference. Mirrors the `property_get_as_function_argument` unit test.

INTERFACE IValue
    PROPERTY_GET value: DINT END_PROPERTY
END_INTERFACE

FUNCTION_BLOCK FbStore IMPLEMENTS IValue
    VAR
        _val : DINT;
    END_VAR

    PROPERTY_GET value: DINT
        value := _val;
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION consumer
    VAR_INPUT
        x : DINT;
    END_VAR

    printf('consumed=%d$N', x);
END_FUNCTION

FUNCTION main
    VAR
        instance : FbStore;
        iface : IValue;
    END_VAR

    instance._val := 42;
    iface := instance;

    // Property getter result passed directly as function argument
    consumer(iface.value);              // CHECK: consumed=42

    instance._val := 99;
    iface := instance;
    consumer(iface.value);              // CHECK-NEXT: consumed=99
END_FUNCTION
