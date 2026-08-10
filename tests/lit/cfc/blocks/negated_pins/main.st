FUNCTION main : DINT
    // NOT on DINT is the bitwise complement, NOT x = -x - 1.
    // in1 := NOT NOT a1 = 10; in2 := NOT a2 = -4; myAdd = 6;
    // b1 := NOT NOT myAdd = 6; b2 := NOT myAddDoubled = NOT 12 = -13.
    function_args.a1 := 10;
    function_args.a2 := 3;
    function_args();
    // CHECK: b1 = 6, b2 = -13
    printf('b1 = %d, b2 = %d$N', function_args.b1, function_args.b2);
END_FUNCTION
