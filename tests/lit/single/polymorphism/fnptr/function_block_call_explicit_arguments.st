// RUN: (%COMPILE %s && %RUN) | %CHECK %s

TYPE Position2D:
    STRUCT
        x: DINT;
        y: DINT;
    END_STRUCT
END_TYPE

FUNCTION_BLOCK FbA
    VAR
        localState: DINT := 0;
    END_VAR

    VAR_INPUT
        in1: DINT;
        in2: STRING;
        in3: Position2D;
        in4: ARRAY[1..2] OF STRING;
        in5: ARRAY[1..2] OF Position2D;
    END_VAR

    VAR_OUTPUT
        out1: DINT;
        out2: STRING;
        out3: Position2D;
        out4: ARRAY[1..2] OF STRING;
        out5: ARRAY[1..2] OF Position2D;
    END_VAR

    VAR_IN_OUT
        inout1: DINT;
        inout2: STRING;
        inout3: Position2D;
        inout4: ARRAY[1..2] OF STRING;
        inout5: ARRAY[1..2] OF Position2D;
    END_VAR

    METHOD increaseLocalState
        localState := localState + 1;
    END_METHOD

    printf('localState = %d$N', localState);

    printf('in1 = %d$N', in1);
    printf('in2 = %s$N', ADR(in2));
    printf('in3 = (%d, %d)$N', in3.x, in3.y);
    printf('in4 = [%s, %s]$N', ADR(in4[1]), ADR(in4[2]));
    printf('in5 = [(%d, %d), (%d, %d)]$N', in5[1].x, in5[1].y, in5[2].x, in5[2].y);

    printf('inout1 = %d$N', inout1);
    printf('inout2 = %s$N', ADR(inout2));
    printf('inout3 = (%d, %d)$N', inout3.x, inout3.y);
    printf('inout4 = [%s, %s]$N', ADR(inout4[1]), ADR(inout4[2]));
    printf('inout5 = [(%d, %d), (%d, %d)]$N', inout5[1].x, inout5[1].y, inout5[2].x, inout5[2].y);

    out1 := 55;
    out2 := 'sixty';
    out3.x := 65;
    out3.y := 70;
    out4[1] := 'seventy-five';
    out4[2] := 'eighty';
    out5[1].x := 85;
    out5[1].y := 90;
    out5[2].x := 95;
    out5[2].y := 100;

    inout1 := 105;
    inout2 := 'one-hundred-ten';
    inout3.x := 115;
    inout3.y := 120;
    inout4[1] := 'one-hundred-twenty-five';
    inout4[2] := 'one-hundred-thirty';
    inout5[1].x := 135;
    inout5[1].y := 140;
    inout5[2].x := 145;
    inout5[2].y := 150;
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instanceFbA: FbA;
        bodyPtr: __FPOINTER FbA := ADR(FbA);

        localIn1: DINT;
        localIn2: STRING;
        localIn3: Position2D;
        localIn4: ARRAY[1..2] OF STRING;
        localIn5: ARRAY[1..2] OF Position2D;

        localOut1: DINT;
        localOut2: STRING;
        localOut3: Position2D;
        localOut4: ARRAY[1..2] OF STRING;
        localOut5: ARRAY[1..2] OF Position2D;

        localInout1: DINT;
        localInout2: STRING;
        localInout3: Position2D;
        localInout4: ARRAY[1..2] OF STRING;
        localInout5: ARRAY[1..2] OF Position2D;
    END_VAR

    localIn3.x := 15;
    localIn3.y := 20;
    localIn4[1] := 'twenty-five';
    localIn4[2] := 'thirty';
    localIn5[1].x := 35;
    localIn5[1].y := 40;
    localIn5[2].x := 45;
    localIn5[2].y := 50;

    instanceFbA.increaseLocalState();

    // Partially initialized
    // localState = 1
    // CHECK: in1 = 0
    // CHECK: in2 =
    // CHECK: in3 = (15, 20)
    // CHECK: in4 = [twenty-five, thirty]
    // CHECK: in5 = [(35, 40), (45, 50)]
    // CHECK: inout1 = 0
    // CHECK: inout2 =
    // CHECK: inout3 = (0, 0)
    // CHECK: inout4 = [, ]
    // CHECK: inout5 = [(0, 0), (0, 0)]
    bodyPtr^(
        instanceFbA,
        inout5 := localInout5,
        in1 := localIn1,
        in2 := localIn2,
        in3 := localIn3,
        in4 := localIn4,
        in5 := localIn5,
        out1 => localOut1,
        out2 => localOut2,
        out3 => localOut3,
        out4 => localOut4,
        out5 => localOut5,
        inout1 := localInout1,
        inout2 := localInout2,
        inout3 := localInout3,
        inout4 := localInout4
    );

    // Locally, these should not be modified even if touched by the function block because they are passed by value
    // CHECK: localIn1 = 0
    // CHECK: localIn2 =
    // CHECK: localIn3 = (15, 20)
    // CHECK: localIn4 = [twenty-five, thirty]
    // CHECK: localIn5 = [(35, 40), (45, 50)]
    printf('localIn1 = %d$N', localIn1);
    printf('localIn2 = %s$N', ADR(localIn2));
    printf('localIn3 = (%d, %d)$N', localIn3.x, localIn3.y);
    printf('localIn4 = [%s, %s]$N', ADR(localIn4[1]), ADR(localIn4[2]));
    printf('localIn5 = [(%d, %d), (%d, %d)]$N', localIn5[1].x, localIn5[1].y, localIn5[2].x, localIn5[2].y);

    // Locally, these should be modified because they are passed by reference (not really but kind of)
    // CHECK: localOut1 = 55
    // CHECK: localOut2 = sixty
    // CHECK: localOut3 = (65, 70)
    // CHECK: localOut4 = [seventy-five, eighty]
    // CHECK: localOut5 = [(85, 90), (95, 100)]
    printf('localOut1 = %d$N', localOut1);
    printf('localOut2 = %s$N', ADR(localOut2));
    printf('localOut3 = (%d, %d)$N', localOut3.x, localOut3.y);
    printf('localOut4 = [%s, %s]$N', ADR(localOut4[1]), ADR(localOut4[2]));
    printf('localOut5 = [(%d, %d), (%d, %d)]$N', localOut5[1].x, localOut5[1].y, localOut5[2].x, localOut5[2].y);

    // Locally, these should be modified because they are passed by reference
    // CHECK: localInout1 = 105
    // CHECK: localInout2 = one-hundred-ten
    // CHECK: localInout3 = (115, 120)
    // CHECK: localInout4 = [one-hundred-twenty-five, one-hundred-thirty]
    // CHECK: localInout5 = [(135, 140), (145, 150)]
    printf('localInout1 = %d$N', localInout1);
    printf('localInout2 = %s$N', ADR(localInout2));
    printf('localInout3 = (%d, %d)$N', localInout3.x, localInout3.y);
    printf('localInout4 = [%s, %s]$N', ADR(localInout4[1]), ADR(localInout4[2]));
    printf('localInout5 = [(%d, %d), (%d, %d)]$N', localInout5[1].x, localInout5[1].y, localInout5[2].x, localInout5[2].y);
END_FUNCTION