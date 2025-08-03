// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb
VAR
    multiplier: INT := 3;
    x: INT := 10;
    y: INT := 20;
    z: INT := 30;
END_VAR

METHOD calculate: INT
    VAR_INPUT
        a: INT;
        b: INT;
    END_VAR
    calculate := a * b;
END_METHOD

METHOD get_value: INT
    get_value := x + y + z;
END_METHOD

METHOD test
    VAR
        result: INT;
    END_VAR

    // Basic parenthesized expression with THIS
    result := (THIS^.x + THIS^.y);
    // CHECK: Basic parentheses: 30
    printf('Basic parentheses: %d$N', result);

    // Nested parentheses with THIS
    result := ((THIS^.x + THIS^.y) * (THIS^.z));
    // CHECK: Nested parentheses: 900
    printf('Nested parentheses: %d$N', result);

    // Parenthesized expression with method call
    result := (THIS^.x + THIS^.y + (THIS^.calculate(THIS^.x, THIS^.y)));
    // CHECK: Expression with method: 230
    printf('Expression with method: %d$N', result);

    // Complex calculation with mixed precedence and parentheses
    result := ((THIS^.x * multiplier) + (THIS^.y * 2) + THIS^.z);
    // CHECK: Complex calculation: 100
    printf('Complex calculation: %d$N', result);

    // Conditional expression with parentheses
    IF ((THIS^.x + THIS^.y) > THIS^.z) THEN
        result := 42;
    ELSE
        result := 24;
    END_IF;
    // CHECK: Conditional with parentheses: 24
    printf('Conditional with parentheses: %d$N', result);

    // Combined THIS references across parentheses
    result := (THIS^.get_value()) + (THIS^.x * THIS^.y / 2);
    // CHECK: Combined THIS references: 160
    printf('Combined THIS references: %d$N', result);
END_METHOD

END_FUNCTION_BLOCK

FUNCTION main: INT
VAR
    inst: fb;
END_VAR
    inst.__vtable := ADR(__vtable_fb_instance); // FIXME: Remove
    inst.test();
END_FUNCTION
