// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Ensure user-defined STRUCT arguments are passed correctly through interface dispatch for
// VAR_INPUT, VAR_OUTPUT, VAR_IN_OUT and mixed signatures.

TYPE User :
    STRUCT
        id: ULINT;
        name: STRING;
        fingerprint: ARRAY[1..5] OF DINT;
    END_STRUCT
END_TYPE

INTERFACE A
    METHOD input
        VAR_INPUT
            in: User;
        END_VAR
    END_METHOD

    METHOD output
        VAR_OUTPUT
            out: User;
        END_VAR
    END_METHOD

    METHOD inout
        VAR_IN_OUT
            inout: User;
        END_VAR
    END_METHOD

    METHOD in_out_and_inout
        VAR_INPUT
            in: User;
        END_VAR

        VAR_OUTPUT
            out: User;
        END_VAR

        VAR_IN_OUT
            inout: User;
        END_VAR
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS A
    METHOD input
        VAR_INPUT
            in: User;
        END_VAR

        printf('FbA::input, in = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
            in.id,
            in.name,
            in.fingerprint[1],
            in.fingerprint[2],
            in.fingerprint[3],
            in.fingerprint[4],
            in.fingerprint[5]
        );
    END_METHOD

    METHOD output
        VAR_OUTPUT
            out: User;
        END_VAR

        printf('FbA::output, out = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
            out.id,
            out.name,
            out.fingerprint[1],
            out.fingerprint[2],
            out.fingerprint[3],
            out.fingerprint[4],
            out.fingerprint[5]
        );

        out.id := 900;
        out.name := 'output_dirty';
        out.fingerprint[1] := 901;
        out.fingerprint[2] := 902;
        out.fingerprint[3] := 903;
        out.fingerprint[4] := 904;
        out.fingerprint[5] := 905;
    END_METHOD

    METHOD inout
        VAR_IN_OUT
            inout: User;
        END_VAR

        printf('FbA::inout, inout = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
            inout.id,
            inout.name,
            inout.fingerprint[1],
            inout.fingerprint[2],
            inout.fingerprint[3],
            inout.fingerprint[4],
            inout.fingerprint[5]
        );

        inout.id := 800;
        inout.name := 'inout_dirty';
        inout.fingerprint[1] := 801;
        inout.fingerprint[2] := 802;
        inout.fingerprint[3] := 803;
        inout.fingerprint[4] := 804;
        inout.fingerprint[5] := 805;
    END_METHOD

    METHOD in_out_and_inout
        VAR_INPUT
            in: User;
        END_VAR

        VAR_OUTPUT
            out: User;
        END_VAR

        VAR_IN_OUT
            inout: User;
        END_VAR

        printf('FbA::in_out_and_inout, in = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
            in.id,
            in.name,
            in.fingerprint[1],
            in.fingerprint[2],
            in.fingerprint[3],
            in.fingerprint[4],
            in.fingerprint[5]
        );
        printf('FbA::in_out_and_inout, out = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
            out.id,
            out.name,
            out.fingerprint[1],
            out.fingerprint[2],
            out.fingerprint[3],
            out.fingerprint[4],
            out.fingerprint[5]
        );
        printf('FbA::in_out_and_inout, inout = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
            inout.id,
            inout.name,
            inout.fingerprint[1],
            inout.fingerprint[2],
            inout.fingerprint[3],
            inout.fingerprint[4],
            inout.fingerprint[5]
        );

        out.id := 700;
        out.name := 'out_dirty';
        out.fingerprint[1] := 701;
        out.fingerprint[2] := 702;
        out.fingerprint[3] := 703;
        out.fingerprint[4] := 704;
        out.fingerprint[5] := 705;

        inout.id := 600;
        inout.name := 'inout_dirty_2';
        inout.fingerprint[1] := 601;
        inout.fingerprint[2] := 602;
        inout.fingerprint[3] := 603;
        inout.fingerprint[4] := 604;
        inout.fingerprint[5] := 605;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance: FbA;
        reference: A;

        localInputUser: User := (id := 201, name := 'input', fingerprint := [201, 202, 203, 204, 205]);

        localOutputUserClean: User := (id := 301, name := 'output_clean', fingerprint := [301, 302, 303, 304, 305]);
        localOutputUser: User := (id := 301, name := 'output_clean', fingerprint := [301, 302, 303, 304, 305]);

        localInOutUserClean: User := (id := 401, name := 'inout_clean', fingerprint := [401, 402, 403, 404, 405]);
        localInOutUser: User := (id := 401, name := 'inout_clean', fingerprint := [401, 402, 403, 404, 405]);
    END_VAR

    reference := instance;

    // CHECK: FbA::input, in = (id=201, name=input, fp=[201, 202, 203, 204, 205])
    reference.input(localInputUser);

    // CHECK-NEXT: FbA::output, out = (id=301, name=output_clean, fp=[301, 302, 303, 304, 305])
    // CHECK-NEXT: main::output = (id=900, name=output_dirty, fp=[901, 902, 903, 904, 905])
    // CHECK-NEXT: main::output = (id=301, name=output_clean, fp=[301, 302, 303, 304, 305])
    reference.output(localOutputUser);
    printf('main::output = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
        localOutputUser.id,
        localOutputUser.name,
        localOutputUser.fingerprint[1],
        localOutputUser.fingerprint[2],
        localOutputUser.fingerprint[3],
        localOutputUser.fingerprint[4],
        localOutputUser.fingerprint[5]
    );
    localOutputUser := localOutputUserClean;
    printf('main::output = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
        localOutputUser.id,
        localOutputUser.name,
        localOutputUser.fingerprint[1],
        localOutputUser.fingerprint[2],
        localOutputUser.fingerprint[3],
        localOutputUser.fingerprint[4],
        localOutputUser.fingerprint[5]
    );

    // CHECK-NEXT: FbA::inout, inout = (id=401, name=inout_clean, fp=[401, 402, 403, 404, 405])
    // CHECK-NEXT: main::inout = (id=800, name=inout_dirty, fp=[801, 802, 803, 804, 805])
    // CHECK-NEXT: main::inout = (id=401, name=inout_clean, fp=[401, 402, 403, 404, 405])
    reference.inout(localInOutUser);
    printf('main::inout = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
        localInOutUser.id,
        localInOutUser.name,
        localInOutUser.fingerprint[1],
        localInOutUser.fingerprint[2],
        localInOutUser.fingerprint[3],
        localInOutUser.fingerprint[4],
        localInOutUser.fingerprint[5]
    );
    localInOutUser := localInOutUserClean;
    printf('main::inout = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
        localInOutUser.id,
        localInOutUser.name,
        localInOutUser.fingerprint[1],
        localInOutUser.fingerprint[2],
        localInOutUser.fingerprint[3],
        localInOutUser.fingerprint[4],
        localInOutUser.fingerprint[5]
    );

    // CHECK-NEXT: FbA::in_out_and_inout, in = (id=201, name=input, fp=[201, 202, 203, 204, 205])
    // CHECK-NEXT: FbA::in_out_and_inout, out = (id=301, name=output_clean, fp=[301, 302, 303, 304, 305])
    // CHECK-NEXT: FbA::in_out_and_inout, inout = (id=401, name=inout_clean, fp=[401, 402, 403, 404, 405])
    // CHECK-NEXT: main::out_after_mix = (id=700, name=out_dirty, fp=[701, 702, 703, 704, 705])
    // CHECK-NEXT: main::inout_after_mix = (id=600, name=inout_dirty_2, fp=[601, 602, 603, 604, 605])
    reference.in_out_and_inout(localInputUser, localOutputUser, localInOutUser);
    printf('main::out_after_mix = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
        localOutputUser.id,
        localOutputUser.name,
        localOutputUser.fingerprint[1],
        localOutputUser.fingerprint[2],
        localOutputUser.fingerprint[3],
        localOutputUser.fingerprint[4],
        localOutputUser.fingerprint[5]
    );
    printf('main::inout_after_mix = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
        localInOutUser.id,
        localInOutUser.name,
        localInOutUser.fingerprint[1],
        localInOutUser.fingerprint[2],
        localInOutUser.fingerprint[3],
        localInOutUser.fingerprint[4],
        localInOutUser.fingerprint[5]
    );
END_FUNCTION
