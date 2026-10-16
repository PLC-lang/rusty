// RUN: %COMPILE %s && %RUN | %CHECK %s
// Scenario: A property's getter and setter share one name location. A short-circuit operand in the
// setter that needs a fat pointer preamble gets its BOOL temporary declared on the setter, not on the
// getter, and a retained binding in the getter is emitted once, for the generated accessor only.

INTERFACE IA
    METHOD foo
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD foo
        printf('FbA::foo$N');
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK Consumer
    VAR_INPUT
        in1 : REFERENCE TO IA;
    END_VAR

    in1.foo();
END_FUNCTION_BLOCK

FUNCTION consume : BOOL
    VAR_INPUT
        in1 : IA;
    END_VAR

    consume := TRUE;
END_FUNCTION

VAR_GLOBAL
    consumer : Consumer;
    instance : FbA;
    enabled : BOOL := TRUE;
END_VAR

FUNCTION_BLOCK Owner
    VAR
        flag : BOOL;
    END_VAR

    PROPERTY_GET value : BOOL
        consumer(instance);
        value := flag;
    END_PROPERTY

    PROPERTY_SET value : BOOL
        flag := enabled AND_THEN consume(instance);
    END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        owner : Owner;
    END_VAR

    // CHECK: FbA::foo
    // CHECK-NEXT: value = 1
    owner.value := FALSE;
    printf('value = %d$N', owner.value);
END_FUNCTION
