// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Array elements must have their vtable pointer set up: virtual dispatch
// through a dereferenced pointer reads the element's __vtable field and
// crashes if construction skipped it.
FUNCTION_BLOCK Counter
VAR
    base : DINT := 30;
END_VAR
    METHOD Value : DINT
        Value := base + 12;
    END_METHOD
END_FUNCTION_BLOCK

PROGRAM prog
VAR
    arr : ARRAY[0..3] OF Counter;
    r : REF_TO Counter;
END_VAR
    r := ADR(arr[2]);
    printf('%d$N', r^.Value()); // CHECK: 42
END_PROGRAM

FUNCTION main : DINT
    prog();
END_FUNCTION
