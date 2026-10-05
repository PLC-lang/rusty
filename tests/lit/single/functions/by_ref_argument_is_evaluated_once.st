// RUN: %COMPILE %s && %RUN | %CHECK %s
// An expression passed to a by-reference input is evaluated once, also when the compiler carries it
// out through a replacement such as a builtin or a standard library call

VAR_GLOBAL
    calls : DINT;
END_VAR

FUNCTION next_number : DINT
    calls := calls + 1;
    next_number := calls;
END_FUNCTION

FUNCTION next_duration : TIME
    calls := calls + 1;
    next_duration := T#5s;
END_FUNCTION

FUNCTION read_number : DINT
VAR_INPUT {ref}
    in : DINT;
END_VAR
    read_number := in;
END_FUNCTION

FUNCTION read_duration : TIME
VAR_INPUT {ref}
    in : TIME;
END_VAR
    read_duration := in;
END_FUNCTION

FUNCTION main : DINT
    VAR
        n : DINT := 10;
        t : TIME := T#1s;
        result : DINT;
        duration : TIME;
    END_VAR

    result := read_number(n + next_number());
    printf('%d$N', result);                                                         // CHECK: 11
    printf('%d$N', calls);                                                          // CHECK: 1

    result := read_number(ADD(n, next_number()));
    printf('%d$N', result);                                                         // CHECK: 12
    printf('%d$N', calls);                                                          // CHECK: 2

    duration := read_duration(ADD(t, next_duration()));
    printf('%s$N', REF(TIME_TO_STRING(duration)));                                  // CHECK: T#6s
    printf('%d$N', calls);                                                          // CHECK: 3

END_FUNCTION
