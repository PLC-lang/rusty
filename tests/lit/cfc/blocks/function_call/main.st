FUNCTION main : DINT
    VAR
        result : DINT;
        doubled : DINT;
    END_VAR
    result := function_call(in1 := 3, in2 := 4, doubledOut => doubled);
    // CHECK: result = 7, doubled = 14
    printf('result = %d, doubled = %d$N', result, doubled);
END_FUNCTION
