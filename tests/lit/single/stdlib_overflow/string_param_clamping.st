// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that out-of-range string function parameters clamp or leave the
// base string unchanged, matching CODESYS (except the position-0 quirk
// of DELETE/REPLACE, which is treated as an invalid position here)

FUNCTION main : DINT
VAR
    s : STRING;
END_VAR
    s := LEFT('hello', 10);
    // CHECK: hello
    printf('%s$N', REF(s));

    s := MID('hello', 10, 3);
    // CHECK: llo
    printf('%s$N', REF(s));

    s := MID('hello', 2, 10);
    // CHECK: mid_invalid_position_yields_empty
    IF s = '' THEN
        printf('mid_invalid_position_yields_empty$N');
    END_IF

    s := INSERT('hello', 'XX', 10);
    // CHECK: hello
    printf('%s$N', REF(s));

    s := DELETE('hello', 3, 4);
    // CHECK: hel
    printf('%s$N', REF(s));

    s := DELETE('hello', 2, 10);
    // CHECK: hello
    printf('%s$N', REF(s));

    s := REPLACE('hello', 'X', 3, 4);
    // CHECK: helX
    printf('%s$N', REF(s));

    s := REPLACE('hello', 'X', 2, 0);
    // CHECK: hello
    printf('%s$N', REF(s));

    main := 0;
END_FUNCTION
