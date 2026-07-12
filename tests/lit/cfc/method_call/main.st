// Entry point: seed the instance state and the method argument, run the network once,
// and observe the method's return value routed into the sink (40 + 2 = 42).
FUNCTION main : DINT
    mainProgram.myInstance.base := 40;
    mainProgram.localOffset := 2;
    mainProgram();

    // CHECK: result = 42
    printf('result = %d$N', mainProgram.localResult);
END_FUNCTION
