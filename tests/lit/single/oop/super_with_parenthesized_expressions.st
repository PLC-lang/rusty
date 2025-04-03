// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// CHECK: Basic parentheses: 30
// CHECK: Nested parentheses: 900
// CHECK: Expression with method: 230
// CHECK: Complex calculation: 100
// CHECK: Conditional with parentheses: 24
// CHECK: Combined SUPER references: 160
FUNCTION_BLOCK parent
VAR
    x : INT := 10;
    y : INT := 20;
    z : INT := 30;
END_VAR

METHOD calculate : INT
    VAR_INPUT
        a : INT;
        b : INT;
    END_VAR
    calculate := a * b;
END_METHOD

METHOD get_value : INT
    get_value := x + y + z;
END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK child EXTENDS parent
VAR
    multiplier : INT := 3;
END_VAR

METHOD test
    VAR
        result : INT;
    END_VAR
    
    // Basic parenthesized expression with SUPER
    result := (SUPER^.x + SUPER^.y);
    printf('Basic parentheses: %d$N', result);
    
    // Nested parentheses with SUPER
    result := ((SUPER^.x + SUPER^.y) * (SUPER^.z));
    printf('Nested parentheses: %d$N', result);
    
    // Parenthesized expression with method call
    result := (SUPER^.x + SUPER^.y + (SUPER^.calculate(SUPER^.x, SUPER^.y)));
    printf('Expression with method: %d$N', result);
    
    // Complex calculation with mixed precedence and parentheses
    result := ((SUPER^.x * multiplier) + (SUPER^.y * 2) + SUPER^.z);
    printf('Complex calculation: %d$N', result);
    
    // Conditional expression with parentheses
    IF ((SUPER^.x + SUPER^.y) > SUPER^.z) THEN
        result := 42;
    ELSE
        result := 24;
    END_IF;
    printf('Conditional with parentheses: %d$N', result);
    
    // Combined SUPER references across parentheses
    result := (SUPER^.get_value()) + (SUPER^.x * SUPER^.y / 2);
    printf('Combined SUPER references: %d$N', result);
END_METHOD

END_FUNCTION_BLOCK

FUNCTION main : INT
VAR
    inst : child;
END_VAR
    inst.test();
END_FUNCTION
