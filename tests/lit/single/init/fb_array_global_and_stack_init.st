// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Arrays of function blocks declared as globals and as locals of stateless
// POUs (stack-constructed on each call) must construct every element.
VAR_GLOBAL
    init_calls : DINT;
    g_arr : ARRAY[0..2] OF MyFb;
END_VAR

FUNCTION_BLOCK MyFb
VAR
    x : INT := 5;
END_VAR
    METHOD FB_INIT
        init_calls := init_calls + 1;
    END_METHOD
END_FUNCTION_BLOCK

FUNCTION check_locals : DINT
VAR
    l_arr : ARRAY[0..1] OF MyFb;
END_VAR
    check_locals := l_arr[1].x;
END_FUNCTION

FUNCTION main : DINT
    printf('%d$N', g_arr[2].x);     // CHECK: 5
    printf('%d$N', init_calls);     // CHECK: 3
    printf('%d$N', check_locals()); // CHECK: 5
    // stack locals are constructed on each call: 3 at startup + 2 per call
    printf('%d$N', init_calls);     // CHECK: 5
END_FUNCTION
