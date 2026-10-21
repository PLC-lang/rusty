// RUN: rm -f %t.out
// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK --implicit-check-not='{{.*}}__itable{{.*}}' %s
// RUN: test ! -e %t.out
// Scenario: only POU instances and interface variables can become an interface value. A DINT passed
// to an interface parameter is rejected during validation instead of failing in code generation.

INTERFACE IA
    METHOD foo : DINT
    END_METHOD
END_INTERFACE

FUNCTION f_val : DINT
    VAR_INPUT
        x : IA;
    END_VAR
END_FUNCTION

FUNCTION f_out : DINT
    VAR_OUTPUT
        x : IA;
    END_VAR
END_FUNCTION

FUNCTION main : DINT
    VAR
        d : DINT;
    END_VAR
    // CHECK-DAG: {{.*}}argument_non_pou.st:[[@LINE+1]]:11:{{.*}}error[E126]: Invalid assignment: 'DINT' does not implement interface 'IA'
    f_val(d);
    // CHECK-DAG: {{.*}}argument_non_pou.st:[[@LINE+1]]:16:{{.*}}error[E126]: Type mismatch: expected type 'IA' but received 'DINT'; parameter 'x' is passed by reference and needs a variable of type 'IA'
    f_out(x => d);
END_FUNCTION
// CHECK: error: Compilation aborted due to critical errors
