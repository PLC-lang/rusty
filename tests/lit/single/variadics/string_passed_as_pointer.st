// RUN: (%COMPILE %s && %RUN) | %CHECK %s
// Test that STRING types are correctly passed as pointers to variadic functions
// per C spec (strings decay to char* pointers)
// CHECK: My string is: hello
FUNCTION main : DINT
VAR
    myString : STRING := 'hello';
END_VAR
    printf('My string is: %s$N', myString);
END_FUNCTION
