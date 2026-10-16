// RUN: %COMPILE %s && %RUN | %CHECK %s
//
// `FOR i := 0 TO count - 1` with a UDINT counter and `count = 0` runs zero times, because
// `count - 1` runs in DINT and is -1.

FUNCTION main
    VAR
        i : UDINT;
        count : UDINT := 0;
        passes : DINT := 0;
    END_VAR
    FOR i := 0 TO count - 1 DO
        passes := passes + 1;
        IF passes >= 3 THEN
            EXIT;
        END_IF
    END_FOR
    printf('passes: %d$N', passes);
    // CHECK: passes: 0
END_FUNCTION
