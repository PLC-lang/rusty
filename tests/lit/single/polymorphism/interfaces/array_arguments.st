// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Ensure ARRAY[1..5] OF DINT arguments are passed correctly through interface dispatch for
// VAR_INPUT, VAR_OUTPUT, VAR_IN_OUT and mixed signatures.

TYPE DintArray5: ARRAY[1..5] OF DINT; END_TYPE

INTERFACE A
    METHOD input
        VAR_INPUT
            in: DintArray5;
        END_VAR
    END_METHOD

    METHOD output
        VAR_OUTPUT
            out: DintArray5;
        END_VAR
    END_METHOD

    METHOD inout
        VAR_IN_OUT
            inout: DintArray5;
        END_VAR
    END_METHOD

    METHOD in_out_and_inout
        VAR_INPUT
            in: DintArray5;
        END_VAR

        VAR_OUTPUT
            out: DintArray5;
        END_VAR

        VAR_IN_OUT
            inout: DintArray5;
        END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS A
    METHOD input
        VAR_INPUT
            in: DintArray5;
        END_VAR

        printf('FbA::input, in = [%d, %d, %d, %d, %d]$N',
            in[1],
            in[2],
            in[3],
            in[4],
            in[5]
        );
    END_METHOD

    METHOD output
        VAR_OUTPUT
            out: DintArray5;
        END_VAR

        printf('FbA::output, out = [%d, %d, %d, %d, %d]$N',
            out[1],
            out[2],
            out[3],
            out[4],
            out[5]
        );

        out[1] := 90;
        out[2] := 91;
        out[3] := 92;
        out[4] := 93;
        out[5] := 94;
    END_METHOD

    METHOD inout
        VAR_IN_OUT
            inout: DintArray5;
        END_VAR

        printf('FbA::inout, inout = [%d, %d, %d, %d, %d]$N',
            inout[1],
            inout[2],
            inout[3],
            inout[4],
            inout[5]
        );

        inout[1] := 80;
        inout[2] := 81;
        inout[3] := 82;
        inout[4] := 83;
        inout[5] := 84;
    END_METHOD

    METHOD in_out_and_inout
        VAR_INPUT
            in: DintArray5;
        END_VAR

        VAR_OUTPUT
            out: DintArray5;
        END_VAR

        VAR_IN_OUT
            inout: DintArray5;
        END_VAR

        printf('FbA::in_out_and_inout, in = [%d, %d, %d, %d, %d]$N',
            in[1],
            in[2],
            in[3],
            in[4],
            in[5]
        );
        printf('FbA::in_out_and_inout, out = [%d, %d, %d, %d, %d]$N',
            out[1],
            out[2],
            out[3],
            out[4],
            out[5]
        );
        printf('FbA::in_out_and_inout, inout = [%d, %d, %d, %d, %d]$N',
            inout[1],
            inout[2],
            inout[3],
            inout[4],
            inout[5]
        );

        out[1] := 70;
        out[2] := 71;
        out[3] := 72;
        out[4] := 73;
        out[5] := 74;

        inout[1] := 60;
        inout[2] := 61;
        inout[3] := 62;
        inout[4] := 63;
        inout[5] := 64;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance: FbA;
        reference: A;

        localInputArray: DintArray5 := [21, 22, 23, 24, 25];

        localOutputArrayClean: DintArray5 := [31, 32, 33, 34, 35];
        localOutputArray: DintArray5 := [31, 32, 33, 34, 35];

        localInOutArrayClean: DintArray5 := [41, 42, 43, 44, 45];
        localInOutArray: DintArray5 := [41, 42, 43, 44, 45];
    END_VAR

    reference := instance;

    // CHECK: FbA::input, in = [21, 22, 23, 24, 25]
    reference.input(localInputArray);

    // CHECK-NEXT: FbA::output, out = [31, 32, 33, 34, 35]
    // CHECK-NEXT: main::output = [90, 91, 92, 93, 94]
    // CHECK-NEXT: main::output = [31, 32, 33, 34, 35]
    reference.output(localOutputArray);
    printf('main::output = [%d, %d, %d, %d, %d]$N',
        localOutputArray[1],
        localOutputArray[2],
        localOutputArray[3],
        localOutputArray[4],
        localOutputArray[5]
    );
    localOutputArray := localOutputArrayClean;
    printf('main::output = [%d, %d, %d, %d, %d]$N',
        localOutputArray[1],
        localOutputArray[2],
        localOutputArray[3],
        localOutputArray[4],
        localOutputArray[5]
    );

    // CHECK-NEXT: FbA::inout, inout = [41, 42, 43, 44, 45]
    // CHECK-NEXT: main::inout = [80, 81, 82, 83, 84]
    // CHECK-NEXT: main::inout = [41, 42, 43, 44, 45]
    reference.inout(localInOutArray);
    printf('main::inout = [%d, %d, %d, %d, %d]$N',
        localInOutArray[1],
        localInOutArray[2],
        localInOutArray[3],
        localInOutArray[4],
        localInOutArray[5]
    );
    localInOutArray := localInOutArrayClean;
    printf('main::inout = [%d, %d, %d, %d, %d]$N',
        localInOutArray[1],
        localInOutArray[2],
        localInOutArray[3],
        localInOutArray[4],
        localInOutArray[5]
    );

    // CHECK-NEXT: FbA::in_out_and_inout, in = [21, 22, 23, 24, 25]
    // CHECK-NEXT: FbA::in_out_and_inout, out = [31, 32, 33, 34, 35]
    // CHECK-NEXT: FbA::in_out_and_inout, inout = [41, 42, 43, 44, 45]
    // CHECK-NEXT: main::out_after_mix = [70, 71, 72, 73, 74]
    // CHECK-NEXT: main::inout_after_mix = [60, 61, 62, 63, 64]
    reference.in_out_and_inout(localInputArray, localOutputArray, localInOutArray);
    printf('main::out_after_mix = [%d, %d, %d, %d, %d]$N',
        localOutputArray[1],
        localOutputArray[2],
        localOutputArray[3],
        localOutputArray[4],
        localOutputArray[5]
    );
    printf('main::inout_after_mix = [%d, %d, %d, %d, %d]$N',
        localInOutArray[1],
        localInOutArray[2],
        localInOutArray[3],
        localInOutArray[4],
        localInOutArray[5]
    );
END_FUNCTION
