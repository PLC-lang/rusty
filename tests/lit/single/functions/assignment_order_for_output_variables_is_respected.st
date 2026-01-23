// RUN: (%COMPILE %s && %RUN) | %CHECK %s

FUNCTION TWO_OUTPUT_FUNCTION
    VAR_OUTPUT
        out1 : DINT;
        out2 : DINT;
    END_VAR
    out1 := 1;
    out2 := 2;
END_FUNCTION

FUNCTION main
VAR
    dIntVar : DINT;
    arrVar : ARRAY[0..3] OF DINT := [0,0,0];
END_VAR

    TWO_OUTPUT_FUNCTION(out2=>dintVar, out1=>dintVar); // dintVar= 2(RuSTy), 1(CODESYS)

    // CHECK: 1
    PRINTF('%d$N', dIntVar);

    TWO_OUTPUT_FUNCTION(out1=>dintVar, out2=>arrVar[dintVar]); // arrVar=[2 0 0] (RuSTy), [0 2 0](CODESYS)

    // CHECK: [0 2 0]
    PRINTF('[%d %d %d]$N', arrVar[0], arrVar[1], arrVar[2]);

END_FUNCTION
