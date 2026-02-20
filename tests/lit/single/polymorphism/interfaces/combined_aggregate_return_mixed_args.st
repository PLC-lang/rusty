// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: A method returns a STRUCT (aggregate) and takes a mix of scalar, interface, and multiple
// different interface-typed parameters. Tests both positional (unnamed) and named arguments with
// completely reordered parameter positions to verify that fat pointer wrapping targets the correct
// parameters regardless of call-site argument order.

TYPE Result :
    STRUCT
        summary: STRING;
        total: DINT;
    END_STRUCT
END_TYPE

INTERFACE IA
    METHOD describe : STRING
    END_METHOD
END_INTERFACE

INTERFACE IB
    METHOD value : DINT
    END_METHOD
END_INTERFACE

INTERFACE IC
    METHOD combine : Result
        VAR_INPUT
            x: DINT;
            a: IA;
            y: REAL;
            b: IB;
            label: STRING;
        END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS IA
    METHOD describe : STRING
        describe := 'FbA';
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbB IMPLEMENTS IB
    METHOD value : DINT
        value := 100;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK FbC IMPLEMENTS IC
    METHOD combine : Result
        VAR_INPUT
            x: DINT;
            a: IA;
            y: REAL;
            b: IB;
            label: STRING;
        END_VAR

        // Dispatch through both interface arguments to verify their fat pointers are valid
        combine.summary := CONCAT(label, ': ');
        combine.summary := CONCAT(combine.summary, a.describe());
        combine.total := x + b.value() + REAL_TO_DINT(y);
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        fa: FbA;
        fb: FbB;
        fc: FbC;
        ref_c: IC;
        result: Result;
    END_VAR

    ref_c := fc;

    // Positional (unnamed) arguments: x=10, a=fa, y=3.0, b=fb, label='test'
    // CHECK: test: FbA, 113
    result := ref_c.combine(10, fa, 3.0, fb, 'test');
    printf('%s, %d$N', result.summary, result.total);

    // Named arguments in completely different order from declaration
    // CHECK-NEXT: named: FbA, 125
    result := ref_c.combine(label := 'named', b := fb, y := 5.0, x := 20, a := fa);
    printf('%s, %d$N', result.summary, result.total);

    // Named arguments in yet another order, partial overlap with declaration order
    // CHECK-NEXT: mixed: FbA, 130
    result := ref_c.combine(a := fa, x := 30, label := 'mixed', y := 0.0, b := fb);
    printf('%s, %d$N', result.summary, result.total);
END_FUNCTION
