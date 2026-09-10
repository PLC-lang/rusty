// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that string functions tolerate invalid UTF-8/UTF-16 content
// (bytes written through pointers, comms buffers or C interop)
// instead of aborting the process

FUNCTION main : DINT
VAR
    s : STRING := 'abc';
    t : STRING;
    w : WSTRING;
    ws : WSTRING := "abc";
    p : REF_TO BYTE;
    pw : REF_TO WORD;
END_VAR
    // invalid UTF-8: patch the first byte of 'abc'
    p := REF(s);
    p^ := 16#FF;

    // CHECK: len_counts_invalid_byte 3
    printf('len_counts_invalid_byte %d$N', LEN(s));

    // CHECK: find_positions_across_invalid_byte 3
    printf('find_positions_across_invalid_byte %d$N', FIND(s, 'c'));

    t := CONCAT(s, 'd');
    // CHECK: concat_keeps_raw_bytes 4
    printf('concat_keeps_raw_bytes %d$N', LEN(t));

    w := STRING_TO_WSTRING(s);
    // CHECK: string_to_wstring_replaces 3
    printf('string_to_wstring_replaces %d$N', LEN(w));

    // invalid UTF-16: patch an unpaired surrogate into "abc"
    pw := REF(ws);
    pw^ := WORD#16#D800;

    // CHECK: wstring_len_counts_surrogate 3
    printf('wstring_len_counts_surrogate %d$N', LEN(ws));

    w := LEFT(ws, 2);
    // CHECK: left_replaces_surrogate 2
    printf('left_replaces_surrogate %d$N', LEN(w));

    main := 0;
END_FUNCTION
