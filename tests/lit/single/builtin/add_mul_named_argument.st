// RUN: not %COMPILE %s --error-format clang 2>&1 | %CHECK %s
//
// ADD and MUL declare a single variadic parameter, which has no name. A named argument is
// rejected, also IN1 and also with the standard library linked.

FUNCTION main : DINT
VAR
    a : DINT := 1;
    b : DINT := 2;
END_VAR
    // CHECK: {{.*}}add_mul_named_argument.st:[[@LINE+2]]:{{[0-9]+}}:{{.*}}error[E089]: Invalid call parameters
    // CHECK-NEXT: {{.*}}add_mul_named_argument.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E048]: Could not resolve reference to IN1
    main := ADD(IN1 := a, b);
    // CHECK-NEXT: {{.*}}add_mul_named_argument.st:[[@LINE+2]]:{{[0-9]+}}:{{.*}}error[E089]: Invalid call parameters
    // CHECK-NEXT: {{.*}}add_mul_named_argument.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E048]: Could not resolve reference to IN1
    main := MUL(IN1 := a, b);
    // CHECK-NEXT: error: Compilation aborted due to critical errors
END_FUNCTION
