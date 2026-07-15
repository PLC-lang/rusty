FUNCTION main : DINT
    fb_call();
    fb_call();
    // CHECK: a = 2, b = 20
    printf('a = %d, b = %d$N', fb_call.outA, fb_call.outB);
END_FUNCTION
