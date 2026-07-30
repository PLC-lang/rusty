// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Every element of an array of function blocks must be fully constructed:
// declared defaults applied and FB_INIT invoked once per element.
VAR_GLOBAL
    init_calls : DINT;
END_VAR

FUNCTION_BLOCK MyFb
VAR
    x : INT := 7;
    marker : DINT;
END_VAR
    METHOD FB_INIT
        init_calls := init_calls + 1;
        marker := 42;
    END_METHOD
END_FUNCTION_BLOCK

PROGRAM prog
VAR
    arr : ARRAY[0..15] OF MyFb;
    single : MyFb;
END_VAR
    // one FB_INIT call per array element plus one for `single`
    printf('%d$N', init_calls);     // CHECK: 17
    printf('%d$N', arr[0].x);       // CHECK: 7
    printf('%d$N', arr[1].marker);  // CHECK: 42
    printf('%d$N', arr[15].x);      // CHECK: 7
    printf('%d$N', single.marker);  // CHECK: 42
END_PROGRAM

FUNCTION main : DINT
    prog();
END_FUNCTION
