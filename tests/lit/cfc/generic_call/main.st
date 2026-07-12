// Entry point: seed both selectable inputs, run the network once, and observe that SEL
// picked IN1 (the G input is wired to the literal TRUE).
FUNCTION main : DINT
    mainProgram.localIn0 := 10;
    mainProgram.localIn1 := 20;
    mainProgram();

    // CHECK: result = 20
    printf('result = %d$N', mainProgram.result);
END_FUNCTION
