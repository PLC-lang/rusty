// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK foo
VAR
    bar: DINT := 42;
END_VAR
    METHOD addToBar: DINT
    VAR_INPUT
        in: INT;
    END_VAR
    VAR 
        bar: DINT := 17;
    END_VAR        
        bar := in + bar;
        addToBar := bar;
    END_METHOD
    
    addToBar(3);
    printf('%d$N', bar); // CHECK: 42 
END_FUNCTION_BLOCK
    
FUNCTION main
VAR
    fb: foo;
    x: DINT;
END_VAR
    fb();
    x := fb.addToBar(3);
    printf('%d$N', x); // CHECK: 20 
END_FUNCTION