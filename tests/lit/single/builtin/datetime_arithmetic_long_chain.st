// RUN: %COMPILE %s && %RUN | %CHECK %s
// A long chain of date and time arithmetic compiles in time linear to its length

FUNCTION main
    VAR
        t : TIME := TIME#1s;
    END_VAR

    t := ADD(t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t, t);
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#30s

    t := t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t + t;
    printf('%s$N', REF(TIME_TO_STRING(t)));                                         // CHECK: T#15m

END_FUNCTION
