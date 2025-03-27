// RUN: (%COMPILE %s && %RUN) | %CHECK %s
VAR_GLOBAL
    counter : DINT := 10;
    message : STRING := 'Global message';
    value : REAL := 3.14;
END_VAR

FUNCTION main : DINT
VAR
    counter : DINT := 5;
    message : STRING := 'Local message';
    value : REAL := 2.71;
END_VAR
    // Access local variables
    printf('Local counter: %d$N', counter); // CHECK: Local counter: 5
    printf('Local message: %s$N', REF(message)); // CHECK: Local message: Local message
    printf('Local value: %.2f$N', value);   // CHECK: Local value: 2.71
    
    // Access global variables using the global namespace operator
    printf('Global counter: %d$N', .counter); // CHECK: Global counter: 10
    printf('Global message: %s$N', REF(.message)); // CHECK: Global message: Global message
    printf('Global value: %.2f$N', .value);   // CHECK: Global value: 3.14
    
    // Modify global variables
    .counter := .counter + 1;
    printf('Modified global counter: %d$N', .counter); // CHECK: Modified global counter: 11
    
    // Use global variables in expressions
    counter := counter + .counter;
    printf('Local counter + Global counter: %d$N', counter); // CHECK: Local counter + Global counter: 16
    
    // Multiple global references in one expression
    .value := .value * 2.0;
    printf('Global value doubled: %.2f$N', .value); // CHECK: Global value doubled: 6.28
END_FUNCTION
