FUNCTION main : DINT
    chain.foo := 7;
    chain();
    // CHECK: bar = 7
    printf('bar = %d$N', chain.bar);
END_FUNCTION
