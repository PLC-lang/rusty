// RUN: rm -f %T/%basename_t.out
// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
// RUN: test ! -e %T/%basename_t.out

// In the instance struct a VAR_IN_OUT member is a pointer that is only bound to a
// target while the POU is being called. Reading or writing it from outside used to
// pass validation and dereference a null pointer at runtime.

// CHECK-NOT: {{.*}}error[E037]{{.*}}
// CHECK: {{.*}}var_in_out_access_outside_scope.st:31:{{.*}}error[E037]: VAR_IN_OUT variables cannot be accessed outside of their scope.
// CHECK: {{.*}}var_in_out_access_outside_scope.st:32:{{.*}}error[E037]: VAR_IN_OUT variables cannot be accessed outside of their scope.
// CHECK: error: Compilation aborted due to critical errors

FUNCTION_BLOCK FB_Sample
VAR_IN_OUT
    Velocity : LREAL;
END_VAR
    Velocity := Velocity + 1.0;
END_FUNCTION_BLOCK

FUNCTION_BLOCK FB_Child EXTENDS FB_Sample
    Velocity := 2.0;
END_FUNCTION_BLOCK

PROGRAM mainProg
VAR
    myVel : LREAL;
    fbSample : FB_Sample;
END_VAR
    fbSample(Velocity := myVel);
    fbSample.Velocity := myVel;
    myVel := fbSample.Velocity;
END_PROGRAM
