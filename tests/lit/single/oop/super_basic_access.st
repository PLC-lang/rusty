// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK parent
VAR
    x : INT := 10;
    y : INT := 20;
END_VAR

METHOD get_sum : INT
    get_sum := x + y;
END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK child EXTENDS parent
METHOD get_values : INT
    VAR
        x : INT := 100; // Shadows parent's x
    END_VAR
    VAR_INPUT
        use_parent : BOOL;
    END_VAR
    
    IF use_parent THEN
        get_values := SUPER^.x + y; // Use parent's x but child inherits y
    ELSE
        get_values := x + y; // Use shadowed x and inherited y
    END_IF;
END_METHOD

    printf('x = %d$N', x);                          // CHECK: x = 10
    printf('SUPER^.x = %d$N', SUPER^.x);            // CHECK: SUPER^.x = 10
    printf('y = %d$N', y);                          // CHECK: y = 20
    printf('get_values(TRUE) = %d$N', get_values(TRUE));    // CHECK: get_values(TRUE) = 30
    printf('get_values(FALSE) = %d$N', get_values(FALSE));  // CHECK: get_values(FALSE) = 120
    printf('SUPER^.get_sum() = %d$N', SUPER^.get_sum());    // CHECK: SUPER^.get_sum() = 30
END_FUNCTION_BLOCK

FUNCTION main : INT
VAR
    inst : child;
END_VAR
    inst.__vtable := ADR(__vtable_child_instance); // FIXME: Remove
    inst();
END_FUNCTION
