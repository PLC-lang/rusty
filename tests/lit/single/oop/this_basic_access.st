// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb
VAR
    x : INT := 10;
    y : INT := 20;
END_VAR

METHOD get_sum : INT
    get_sum := x + y;
END_METHOD
METHOD get_values : INT
    VAR
        x : INT := 100; // Shadows fb's x
        y : INT := 200; // Shadows fb's y
    END_VAR
    VAR_INPUT
        use_function_block : BOOL;
    END_VAR

    IF use_function_block THEN
        get_values := THIS^.x + y; // Use fb's x but local y
    ELSE
        get_values := x + y; // Use shadowed x and y
    END_IF;
END_METHOD

    printf('x = %d$N', x);                          // CHECK: x = 10
    printf('THIS^.x = %d$N', THIS^.x);            // CHECK: THIS^.x = 10
    printf('y = %d$N', y);                          // CHECK: y = 20
    printf('get_values(TRUE) = %d$N', get_values(TRUE));    // CHECK: get_values(TRUE) = 210
    printf('get_values(FALSE) = %d$N', get_values(FALSE));  // CHECK: get_values(FALSE) = 300
    printf('THIS^.get_sum() = %d$N', THIS^.get_sum());    // CHECK: THIS^.get_sum() = 30
END_FUNCTION_BLOCK

FUNCTION main : INT
VAR
    inst : fb;
END_VAR
    inst.__vtable := ADR(__vtable_fb_instance); // FIXME: Remove

    inst();
END_FUNCTION
