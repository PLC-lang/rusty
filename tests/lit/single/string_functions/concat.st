// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// End-to-end tests for the CONCAT stdlib function (STRING and WSTRING).
// Also covers truncation to the STRING[2048] / WSTRING[2048] result buffer:
// concatenating past 2048 code units used to overflow the result buffer and
// crash; it must now truncate to 2048 instead. See libs/stdlib/src/string_functions.rs.

FUNCTION main : DINT
VAR
    s      : STRING[2048];
    ws     : WSTRING[2048];
    chunk  : STRING[2048]  := 'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA';
    wchunk : WSTRING[2048] := "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    big    : STRING[2048];
    wbig   : WSTRING[2048];
END_VAR
    // --- functional: STRING ---
    s := CONCAT('Hello', ', ', 'World', '!');
    // CHECK: Hello, World!
    printf('%s$N', REF(s));

    // --- functional: WSTRING ---
    ws := CONCAT("Hello", ", ", "World", "!");
    // CHECK: Hello, World!
    printf('%s$N', REF(TO_STRING(ws)));

    // --- truncation: STRING ---
    // 16 * 128 == 2048 fills the buffer exactly (no truncation yet).
    big := CONCAT(chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk, chunk);
    // CHECK: 2048
    printf('%d$N', LEN(big));
    // CONCAT(big, big) would be 4096; must truncate to 2048 instead of crashing.
    s := CONCAT(big, big);
    // CHECK: 2048
    printf('%d$N', LEN(s));

    // --- truncation: WSTRING ---
    wbig := CONCAT(wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk, wchunk);
    ws := CONCAT(wbig, wbig);
    // CHECK: 2048
    printf('%d$N', LEN(ws));

    main := 0;
END_FUNCTION
