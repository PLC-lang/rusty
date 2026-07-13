FUNCTION main : DINT
    fan_out.foo := 42;
    fan_out();
    // CHECK: bar = 42, baz = 42
    printf('bar = %d, baz = %d$N', fan_out.bar, fan_out.baz);
END_FUNCTION
