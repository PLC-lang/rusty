// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: An interface method returns a user-defined STRUCT (aggregate) and takes an interface-typed
// argument. Combines struct return lowering with fat pointer wrapping. Also tests that two different
// concrete types dispatched through the same interface produce distinct results.

TYPE Pair :
    STRUCT
        first: STRING;
        second: STRING;
    END_STRUCT
END_TYPE

INTERFACE IA
    METHOD make_pair : Pair
        VAR_INPUT
            other: IA;
        END_VAR
    END_METHOD

    METHOD tag : STRING
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD make_pair : Pair
        VAR_INPUT
            other: IA;
        END_VAR
        make_pair.first := 'FbA';
        make_pair.second := other.tag();
    END_METHOD

    METHOD tag : STRING
        tag := 'TagA';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS IA
    METHOD make_pair : Pair
        VAR_INPUT
            other: IA;
        END_VAR
        make_pair.first := 'FbB';
        make_pair.second := other.tag();
    END_METHOD

    METHOD tag : STRING
        tag := 'TagB';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        a: FbA;
        b: FbB;
        ref_a: IA;
        ref_b: IA;
        result: Pair;
    END_VAR

    ref_a := a;
    ref_b := b;

    // FbA.make_pair dispatches other.tag() through FbB's vtable
    // CHECK: first=FbA, second=TagB
    result := ref_a.make_pair(b);
    printf('first=%s, second=%s$N', result.first, result.second);

    // FbB.make_pair dispatches other.tag() through FbA's vtable
    // CHECK-NEXT: first=FbB, second=TagA
    result := ref_b.make_pair(a);
    printf('first=%s, second=%s$N', result.first, result.second);

    // Named argument
    // CHECK-NEXT: first=FbA, second=TagA
    result := ref_a.make_pair(other := a);
    printf('first=%s, second=%s$N', result.first, result.second);
END_FUNCTION
