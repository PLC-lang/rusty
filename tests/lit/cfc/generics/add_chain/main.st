FUNCTION main : DINT
    add_chain.a := 1000;
    add_chain.b := 2000;
    add_chain.c := 100000;
    add_chain();
    // CHECK: result = 103000
    printf('result = %d$N', add_chain.result);
END_FUNCTION
