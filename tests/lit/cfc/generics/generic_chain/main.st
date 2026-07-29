FUNCTION main : DINT
    generic_chain.a := 1000;
    generic_chain.b := 2000;
    generic_chain.c := 100000;
    generic_chain();
    // CHECK: result = 103000
    printf('result = %d$N', generic_chain.result);
END_FUNCTION
