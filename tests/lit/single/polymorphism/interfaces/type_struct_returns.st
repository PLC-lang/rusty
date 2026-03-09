// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Scenario: Ensure user-defined STRUCT return values are passed correctly through interface dispatch.

TYPE User :
    STRUCT
        id: ULINT;
        name: STRING;
        fingerprint: ARRAY[1..5] OF DINT;
    END_STRUCT
END_TYPE

INTERFACE A
    METHOD return_user: User
    END_METHOD
END_INTERFACE

FUNCTION_BLOCK FbA IMPLEMENTS A
    VAR
        callCounter: DINT;
    END_VAR

    METHOD return_user: User
        callCounter := callCounter + 1;

        IF callCounter = 1 THEN
            return_user.id := 101;
            return_user.name := 'first';
            return_user.fingerprint[1] := 101;
            return_user.fingerprint[2] := 102;
            return_user.fingerprint[3] := 103;
            return_user.fingerprint[4] := 104;
            return_user.fingerprint[5] := 105;
        ELSE
            return_user.id := 201;
            return_user.name := 'second';
            return_user.fingerprint[1] := 201;
            return_user.fingerprint[2] := 202;
            return_user.fingerprint[3] := 203;
            return_user.fingerprint[4] := 204;
            return_user.fingerprint[5] := 205;
        END_IF
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION main
    VAR
        instance: FbA;
        reference: A;
        result: User;
    END_VAR

    reference := instance;

    // CHECK: main::result_first = (id=101, name=first, fp=[101, 102, 103, 104, 105])
    result := reference.return_user();
    printf('main::result_first = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
        result.id,
        result.name,
        result.fingerprint[1],
        result.fingerprint[2],
        result.fingerprint[3],
        result.fingerprint[4],
        result.fingerprint[5]
    );

    // CHECK-NEXT: main::result_second = (id=201, name=second, fp=[201, 202, 203, 204, 205])
    result := reference.return_user();
    printf('main::result_second = (id=%d, name=%s, fp=[%d, %d, %d, %d, %d])$N',
        result.id,
        result.name,
        result.fingerprint[1],
        result.fingerprint[2],
        result.fingerprint[3],
        result.fingerprint[4],
        result.fingerprint[5]
    );
END_FUNCTION
