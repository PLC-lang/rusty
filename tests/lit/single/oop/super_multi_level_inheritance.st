// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK grandparent
VAR
    gp_value : INT := 100;
END_VAR

METHOD calculate : INT
    calculate := gp_value;
END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK parent EXTENDS grandparent
VAR
    p_value : INT := 200;
END_VAR

METHOD calculate : INT
    calculate := p_value + SUPER^.calculate();
END_METHOD
END_FUNCTION_BLOCK

FUNCTION_BLOCK child EXTENDS parent
VAR
    c_value : INT := 300;
END_VAR

METHOD calculate : INT
    calculate := c_value + SUPER^.calculate();
END_METHOD

    // Access values from different levels
    printf('c_value = %d$N', c_value);                // CHECK: c_value = 300
    printf('p_value = %d$N', p_value);                // CHECK: p_value = 200
    printf('gp_value = %d$N', gp_value);              // CHECK: gp_value = 100
    
    // Call methods from different inheritance levels
    printf('calculate() = %d$N', calculate());        // CHECK: calculate() = 600
    printf('SUPER^.calculate() = %d$N', SUPER^.calculate());  // CHECK: SUPER^.calculate() = 300
END_FUNCTION_BLOCK

FUNCTION main : INT
VAR
    inst : child;
END_VAR
    inst();
END_FUNCTION
