FUNCTION main : DINT
    operator_functions.a := TRUE;
    operator_functions.b := TRUE;
    operator_functions.c := TRUE;
    operator_functions.x := 17;
    operator_functions.y := 5;
    operator_functions();
    // CHECK: first = 1 1 0 2 0
    printf('first = %u %u %u %d %u$N', operator_functions.and_result, operator_functions.or_result, operator_functions.xor_result, operator_functions.mod_result, operator_functions.not_result);
    // CHECK: first-operator = 1 1 0 2 0
    printf('first-operator = %u %u %u %d %u$N', TRUE AND TRUE AND TRUE, TRUE OR TRUE, TRUE XOR TRUE, 17 MOD 5, NOT TRUE);

    operator_functions.a := FALSE;
    operator_functions.b := TRUE;
    operator_functions.c := TRUE;
    operator_functions.x := 20;
    operator_functions.y := 7;
    operator_functions();
    // CHECK: second = 0 1 1 6 1
    printf('second = %u %u %u %d %u$N', operator_functions.and_result, operator_functions.or_result, operator_functions.xor_result, operator_functions.mod_result, operator_functions.not_result);
    // CHECK: second-operator = 0 1 1 6 1
    printf('second-operator = %u %u %u %d %u$N', FALSE AND TRUE AND TRUE, FALSE OR TRUE, FALSE XOR TRUE, 20 MOD 7, NOT FALSE);
END_FUNCTION
