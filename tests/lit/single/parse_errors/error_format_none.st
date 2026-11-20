// RUN: not %COMPILE %s --error-format none 2>&1 | %CHECK %s
//
// With the error format none, parse errors are not printed. Only the abort line is printed.

FUNCTION foo : BOOL
VAR_INPUT
    in : BOOL;
    in2
END_VAR
;
END_FUNCTION

// CHECK-NOT: {{.+}}
// CHECK: error: Compilation aborted due to critical parse errors
// CHECK-NOT: {{.+}}
