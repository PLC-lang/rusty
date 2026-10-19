// RUN: not %COMPILE %s --error-format clang 2>&1 | %CHECK %s
// A POU that takes the name of a built-in function is rejected with a message that names the collision.

// CHECK: {{.*}}error[E004]: `ABS` is the name of a built-in function
FUNCTION ABS : DINT
VAR_INPUT
    IN : DINT;
END_VAR
END_FUNCTION

// CHECK-NEXT: {{.*}}error[E004]: `Sel` is the name of a built-in function
FUNCTION Sel : DINT
VAR_INPUT
    G : BOOL;
    IN0 : DINT;
    IN1 : DINT;
END_VAR
END_FUNCTION

// CHECK-NEXT: error: Compilation aborted due to critical errors
