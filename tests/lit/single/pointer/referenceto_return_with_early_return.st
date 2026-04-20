// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Critical issue #3 / TODO: Multiple return paths — the REF= assignment to the function
// name happens in a conditional branch, not as the last statement.
FUNCTION selectRef : REFERENCE TO INT
    VAR_INPUT
        a   : REFERENCE TO INT;
        b   : REFERENCE TO INT;
        sel : BOOL;
    END_VAR

    IF sel THEN
        selectRef REF= a;
        RETURN;
    END_IF;

    selectRef REF= b;
END_FUNCTION

FUNCTION main : DINT
    VAR
        refVal : REFERENCE TO INT;
        valA   : INT := 10;
        valB   : INT := 20;
        conVal : INT;
    END_VAR

    refVal REF= valA;

    // sel=TRUE should return reference to valA
    refVal REF= selectRef(refVal, refVal, TRUE);
    conVal := refVal;

    // CHECK: 10
    printf('%d$N', conVal);
END_FUNCTION
