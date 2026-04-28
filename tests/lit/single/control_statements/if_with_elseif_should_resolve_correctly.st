// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION foo : ARRAY[0..1] OF STRING[10]
    VAR
        result : ARRAY[0..1] OF STRING[10];
    END_VAR
    result[0] := 'One';
    result[1] := 'Two';
    foo := result;
END_FUNCTION

FUNCTION getFooResult : STRING[10]
    VAR_INPUT
        index: INT;
    END_VAR
    VAR_IN_OUT
        counterVar : INT;
    END_VAR
    VAR
        fooResult : ARRAY[0..1] OF STRING[10];
    END_VAR
    fooResult := foo();
    counterVar := counterVar + 1;
    getFooResult := fooResult[index];
END_FUNCTION

FUNCTION main
    VAR
        result : INT;
        counterVar: INT;
    END_VAR

    counterVar := 0;

    IF getFooResult(0, counterVar) = 'One' THEN
        result := 1;
    ELSIF getFooResult(1, counterVar) = 'Two' THEN
        result := 2;
    ELSE
        result := 3;
    END_IF

    // CHECK: 1
    printf('%d$N', counterVar);
END_FUNCTION
