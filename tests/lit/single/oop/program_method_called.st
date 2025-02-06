// RUN: (%COMPILE %s && %RUN) | %CHECK %s
PROGRAM foo
VAR
    bar: DINT := 42;
END_VAR
    METHOD addToBar: DINT
    VAR_INPUT
        in: INT;
    END_VAR
        bar := in + bar;
        addToBar := bar;
    END_METHOD
    
    addToBar(3);
    printf('%d$N', bar); // CHECK: 45
END_PROGRAM
    
FUNCTION main
VAR
    x: DINT;
END_VAR
    foo();
    x := foo.addToBar(3);
    printf('%d$N', x); // CHECK: 48
END_FUNCTION