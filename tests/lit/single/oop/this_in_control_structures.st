// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK child
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
METHOD test
    VAR
        i : INT;
        result : INT;
    END_VAR

    // Reset parent's counter
    THIS^.reset();

    // Use THIS^ in IF statement
    // CHECK: Initial counter: 0
    IF THIS^.counter < THIS^.threshold THEN
        printf('Initial counter: %d$N', THIS^.counter);
    END_IF;

    // Use THIS^ in FOR loop
    // CHECK: Loop increment: 1
    // CHECK: Loop increment: 2
    // CHECK: Loop increment: 3
    FOR i := 1 TO 3 BY 1 DO
        result := THIS^.increment();
        printf('Loop increment: %d$N', result);
    END_FOR;

    // Use THIS^ in CASE statement
    CASE THIS^.counter OF
        // CHECK: Counter is three
        3: printf('Counter is three$N');
        4: printf('Counter is four$N');
    END_CASE;

    // Use THIS^ in WHILE loop
    // CHECK: While increment: 4
    // CHECK: While increment: 5
    WHILE THIS^.counter < 5 DO
        result := THIS^.increment();
        printf('While increment: %d$N', result);
    END_WHILE;
END_METHOD

    test();

    // CHECK: Parent counter: 5
    printf('Parent counter: %d$N', THIS^.counter);
END_FUNCTION_BLOCK

FUNCTION main : INT
VAR
    inst : child;
END_VAR
    inst.__vtable := ADR(__vtable_child_instance); // FIXME: Remove

    inst();
END_FUNCTION
