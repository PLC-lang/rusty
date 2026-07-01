// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// End-to-end tests for the INSERT stdlib function (STRING and WSTRING).
// INSERT combines two strings, so its result can exceed the STRING[2048] /
// WSTRING[2048] result buffer. That used to overflow the buffer and crash;
// it must now truncate to 2048 instead. See libs/stdlib/src/string_functions.rs.

FUNCTION main : DINT
VAR
    s      : STRING[2048];
    ws     : WSTRING[2048];
    chunk  : STRING[2048]  := 'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA';
    wchunk : WSTRING[2048] := "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    big    : STRING[2048];
    wbig   : WSTRING[2048];
END_VAR
    // --- functional: STRING --- (keep the first 2 characters, then insert)
    s := INSERT('Hello', 'XYZ', 2);
    // CHECK: HeXYZllo
    printf('%s$N', REF(s));

    // --- functional: WSTRING ---
    ws := INSERT("Hello", "XYZ", 2);
    // CHECK: HeXYZllo
    printf('%s$N', REF(TO_STRING(ws)));

    // --- truncation: STRING ---
    big := CONCAT(chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk);
    // INSERT(big, big, 10) would be ~4096; must truncate to 2048.
    s := INSERT(big, big, 10);
    // CHECK: 2048
    printf('%d$N', LEN(s));

    // --- truncation: WSTRING ---
    wbig := CONCAT(wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk);
    ws := INSERT(wbig, wbig, 10);
    // CHECK: 2048
    printf('%d$N', LEN(ws));

    main := 0;
END_FUNCTION
