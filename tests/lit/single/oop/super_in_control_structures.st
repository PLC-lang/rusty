// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// CHECK: Initial counter: 0
// CHECK: Loop increment: 1
// CHECK: Loop increment: 2
// CHECK: Loop increment: 3
// CHECK: Counter is three
// CHECK: While increment: 4
// CHECK: While increment: 5
// CHECK: Parent counter: 5
FUNCTION_BLOCK parent
VAR
    counter : INT := 0;
    threshold : INT := 5;
END_VAR

METHOD increment : INT
    counter := counter + 1;
    increment := counter;
END_METHOD

METHOD reset
    counter := 0;
END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK child EXTENDS parent
METHOD increment : INT
    increment := SUPER^.increment() + 1; // Increment parent's counter and add 1
END_METHOD
METHOD test
    VAR
        i : INT;
        result : INT;
    END_VAR
    
    // Reset parent's counter
    SUPER^.reset();
    
    // Use SUPER^ in IF statement
    IF SUPER^.counter < SUPER^.threshold THEN
        printf('Initial counter: %d$N', SUPER^.counter);
    END_IF;
    
    // Use SUPER^ in FOR loop
    FOR i := 1 TO 3 BY 1 DO
        result := SUPER^.increment();
        printf('Loop increment: %d$N', result);
    END_FOR;
    
    // Use SUPER^ in CASE statement
    CASE SUPER^.counter OF
        3: printf('Counter is three$N');
        4: printf('Counter is four$N');
    END_CASE;
    
    // Use SUPER^ in WHILE loop
    WHILE SUPER^.counter < 5 DO
        result := SUPER^.increment();
        printf('While increment: %d$N', result);
    END_WHILE;
END_METHOD

    test();
    
    printf('Parent counter: %d$N', SUPER^.counter);
END_FUNCTION_BLOCK

FUNCTION main : INT
VAR
    inst : child;
END_VAR
    inst();
END_FUNCTION
