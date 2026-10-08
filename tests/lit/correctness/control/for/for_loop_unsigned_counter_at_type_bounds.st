// RUN: (%COMPILE %s && %RUN) | %CHECK %s

// A FOR loop ends when the step wraps an unsigned counter at the bounds of its type
FUNCTION main : DINT
    VAR
        u8 : USINT;
        u16 : UINT;
        u32 : UDINT;
        u64 : ULINT;
        iterations : DINT;
    END_VAR

    iterations := 0;
    FOR u32 := 3 TO 0 BY -1 DO
        iterations := iterations + 1;
    END_FOR
    // CHECK: 4 4294967295
    printf('%d %u$N', iterations, u32);

    iterations := 0;
    FOR u16 := 3 TO 0 BY -1 DO
        iterations := iterations + 1;
    END_FOR
    // CHECK: 4 65535
    printf('%d %d$N', iterations, u16);

    iterations := 0;
    FOR u64 := 5 TO 0 BY -2 DO
        iterations := iterations + 1;
    END_FOR
    // CHECK: 3 18446744073709551615
    printf('%d %llu$N', iterations, u64);

    iterations := 0;
    FOR u8 := 250 TO 255 DO
        iterations := iterations + 1;
    END_FOR
    // CHECK: 6 0
    printf('%d %d$N', iterations, u8);

    iterations := 0;
    FOR u32 := 4294967290 TO 4294967295 DO
        iterations := iterations + 1;
    END_FOR
    // CHECK: 6 0
    printf('%d %u$N', iterations, u32);

    // A range across 2^31
    iterations := 0;
    FOR u32 := 2147483646 TO 2147483649 DO
        iterations := iterations + 1;
    END_FOR
    // CHECK: 4
    printf('%d$N', iterations);
END_FUNCTION
