// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: An interface method returns an aggregate type (STRING) and also takes an interface-typed
// argument. Both the aggregate return lowering and fat pointer wrapping must cooperate: the fat pointer
// preamble must execute before the call, and the aggregate alloca must also be set up before the call.
// This is the exact scenario that was broken when AggregateTypeLowerer ran in post_index, causing the
// interface argument to receive wrong type hints and skip fat pointer wrapping.

INTERFACE IA
    METHOD greet : STRING
        VAR_INPUT
            other: IA;
        END_VAR
    END_METHOD

    METHOD name : STRING
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD greet : STRING
        VAR_INPUT
            other: IA;
        END_VAR
        greet := 'FbA says hello to ';
        // Dispatch through the interface argument to verify the fat pointer is valid
        greet := CONCAT(greet, other.name());
    END_METHOD

    METHOD name : STRING
        name := 'FbA';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS IA
    METHOD greet : STRING
        VAR_INPUT
            other: IA;
        END_VAR
        greet := 'FbB says hello to ';
        greet := CONCAT(greet, other.name());
    END_METHOD

    METHOD name : STRING
        name := 'FbB';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        a: FbA;
        b: FbB;
        ref_a: IA;
        ref_b: IA;
        result: STRING;
    END_VAR

    ref_a := a;
    ref_b := b;

    // Method returns STRING (aggregate) and takes an interface argument;
    // the argument must be wrapped in a fat pointer and dispatch through it.
    // CHECK: FbA says hello to FbB
    result := ref_a.greet(b);
    printf('%s$N', result);

    // CHECK-NEXT: FbB says hello to FbA
    result := ref_b.greet(a);
    printf('%s$N', result);

    // Pass an already-interface-typed variable as the argument
    // CHECK-NEXT: FbA says hello to FbB
    result := ref_a.greet(ref_b);
    printf('%s$N', result);
END_FUNCTION
