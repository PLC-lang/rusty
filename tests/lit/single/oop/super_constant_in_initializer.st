// RUN: %COMPILE %s && %RUN | %CHECK %s
FUNCTION_BLOCK grandparent
VAR CONSTANT
    base_value : INT := 42;
END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK parent EXTENDS grandparent
END_FUNCTION_BLOCK

FUNCTION_BLOCK child EXTENDS parent
VAR CONSTANT
    const_from_super : INT := SUPER^.base_value + 1;
END_VAR
VAR
    var_from_super : INT := SUPER^.base_value;
    var_from_inherited : INT := base_value * 2;
END_VAR
VAR_TEMP
    temp_from_super : INT := SUPER^.base_value;
END_VAR
    printf('const_from_super = %d$N', const_from_super);        // CHECK: const_from_super = 43
    printf('var_from_super = %d$N', var_from_super);            // CHECK: var_from_super = 42
    printf('var_from_inherited = %d$N', var_from_inherited);    // CHECK: var_from_inherited = 84
    printf('temp_from_super = %d$N', temp_from_super);          // CHECK: temp_from_super = 42
END_FUNCTION_BLOCK

FUNCTION main : INT
VAR
    inst : child;
END_VAR
    inst();
END_FUNCTION
