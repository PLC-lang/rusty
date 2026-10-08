// RUN: (%COMPILE %s && %RUN) | %CHECK %s

VAR_GLOBAL
    endCalls, stepCalls : DINT;
END_VAR

FUNCTION getEnd : DINT
    endCalls := endCalls + 1;
    getEnd := 3;
END_FUNCTION

FUNCTION getStep : DINT
    stepCalls := stepCalls + 1;
    getStep := 1;
END_FUNCTION

FUNCTION main : DINT
    VAR
        i, iterations : DINT;
    END_VAR

    // The end and step expressions are evaluated once per iteration and once more to end the loop
    FOR i := 0 TO getEnd() BY getStep() DO
        iterations := iterations + 1;
    END_FOR
    // CHECK: 4 5 5
    printf('%d %d %d$N', iterations, endCalls, stepCalls);
END_FUNCTION
