// RUN: %COMPILE %s && %RUN | %CHECK %s
// Deeply nested calls compile in time linear to their depth

FUNCTION increment : DINT
    VAR_INPUT
        value : DINT;
    END_VAR

    increment := value + 1;
END_FUNCTION

FUNCTION main
    VAR
        x : DINT := 0;
    END_VAR

    x := increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(increment(x))))))))))))))))))))))))))))));
    printf('%d$N', x);                                                              // CHECK: 30

END_FUNCTION
