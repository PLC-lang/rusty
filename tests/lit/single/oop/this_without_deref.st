// RUN: (%COMPILE %s && %RUN) | %CHECK %s
FUNCTION_BLOCK fb
VAR
    p_ref : REF_TO fb;
    value : INT := 42;
END_VAR

METHOD get_value : INT
get_value := value;
END_METHOD

    // THIS without deref creates a reference
    p_ref := THIS;

    // Accessing through reference
    printf('p_ref^.value = %d$N', p_ref^.value);      // CHECK: p_ref^.value = 42
    printf('p_ref^.get_value() = %d$N', p_ref^.get_value()); // CHECK: p_ref^.get_value() = 42

    // Modifying through reference
    p_ref^.value := 100;
    printf('THIS^.value after modification = %d$N', THIS^.value); // CHECK: THIS^.value after modification = 100

    // Compare direct access vs reference access
    printf('Are they equal? %d$N', THIS^.value = p_ref^.value); // CHECK: Are they equal? 1
END_FUNCTION_BLOCK

FUNCTION main : INT
VAR
    inst : fb;
END_VAR
    inst();
END_FUNCTION
