// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: A free FUNCTION (not a method on an interface) returns an aggregate type (STRING) and takes
// an interface-typed VAR_INPUT. This tests the interaction between AggregateTypeLowerer and
// InterfaceDispatchLowerer for plain function calls where the callee is known statically but the
// argument still needs fat pointer wrapping.

INTERFACE IA
    METHOD name : STRING
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD name : STRING
        name := 'FbA';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS IA
    METHOD name : STRING
        name := 'FbB';
    END_METHOD
END_FUNCTION_BLOCK

// Free function with aggregate return + interface parameter
FUNCTION format_greeting : STRING
    VAR_INPUT
        prefix: STRING;
        iface: IA;
        suffix: STRING;
    END_VAR

    format_greeting := CONCAT(prefix, iface.name());
    format_greeting := CONCAT(format_greeting, suffix);
END_FUNCTION

FUNCTION main
    VAR
        a: FbA;
        b: FbB;
        result: STRING;
    END_VAR

    // Positional call with concrete FbA instance
    // CHECK: [Hello FbA!]
    result := format_greeting('[Hello ', a, '!]');
    printf('%s$N', result);

    // Positional call with concrete FbB instance
    // CHECK-NEXT: [Hello FbB!]
    result := format_greeting('[Hello ', b, '!]');
    printf('%s$N', result);

    // Named call with reordered arguments
    // CHECK-NEXT: (Greetings FbA.)
    result := format_greeting(suffix := '.)', iface := a, prefix := '(Greetings ');
    printf('%s$N', result);

    // Named call with yet another order
    // CHECK-NEXT: {Hi FbB}
    result := format_greeting(iface := b, suffix := '}', prefix := '{Hi ');
    printf('%s$N', result);
END_FUNCTION
