// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// End-to-end tests for the REPLACE stdlib function (STRING and WSTRING).
// REPLACE can grow a string when the replacement is longer than the replaced
// region, so its result can exceed the STRING[2048] / WSTRING[2048] result
// buffer. That used to overflow the buffer and crash; it must now truncate to
// 2048 instead. See libs/stdlib/src/string_functions.rs.

FUNCTION main : DINT
VAR
    s      : STRING[2048];
    ws     : WSTRING[2048];
    chunk  : STRING[2048]  := 'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA';
    wchunk : WSTRING[2048] := "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    big    : STRING[2048];
    wbig   : WSTRING[2048];
END_VAR
    // --- functional: STRING --- (replace 2 characters starting at position 2)
    s := REPLACE('Hello', 'XX', 2, 2);
    // CHECK: HXXlo
    printf('%s$N', REF(s));

    // --- functional: WSTRING ---
    ws := REPLACE("Hello", "XX", 2, 2);
    // CHECK: HXXlo
    printf('%s$N', REF(TO_STRING(ws)));

    // --- truncation: STRING ---
    big := CONCAT(chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk);
    // REPLACE(big, big, 1, 1) would be ~4095; must truncate to 2048.
    s := REPLACE(big, big, 1, 1);
    // CHECK: 2048
    printf('%d$N', LEN(s));

    // --- truncation: WSTRING ---
    wbig := CONCAT(wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk);
    ws := REPLACE(wbig, wbig, 1, 1);
    // CHECK: 2048
    printf('%d$N', LEN(ws));

    main := 0;
END_FUNCTION
