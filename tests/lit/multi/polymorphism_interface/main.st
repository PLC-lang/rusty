FUNCTION main
    VAR
        instA: FbA;
        instB: FbB;
        instC: FbC;

        arrA: ARRAY[1..3] OF IA;
        arrB: ARRAY[1..2] OF IB;
        arrC: ARRAY[1..1] OF IC;

        i: DINT;
    END_VAR

    // All three FBs implement IA
    arrA[1] := instA;
    arrA[2] := instB;
    arrA[3] := instC;

    FOR i := 1 TO 3 DO
        printf('arrA[%d].describeA()=%d$N', i, arrA[i].describeA());
    END_FOR;

    // FbB and FbC implement IB
    arrB[1] := instB;
    arrB[2] := instC;

    FOR i := 1 TO 2 DO
        printf('arrB[%d].describeB()=%d$N', i, arrB[i].describeB());
    END_FOR;

    // Only FbC implements IC
    arrC[1] := instC;

    printf('arrC[1].describeC()=%d$N', arrC[1].describeC());
END_FUNCTION
