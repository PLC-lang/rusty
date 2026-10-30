// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK %s
// Scenario: a pointer type that points to itself, directly or through another pointer type, is
// named in an interface diagnostic up to the first repeated type instead of recursing forever.

INTERFACE IA
    METHOD foo : DINT
    END_METHOD
END_INTERFACE

TYPE __P : POINTER TO __P; END_TYPE
TYPE __Q : POINTER TO __R; END_TYPE
TYPE __R : POINTER TO __Q; END_TYPE

FUNCTION f_val : DINT
    VAR_INPUT
        x : IA;
    END_VAR
END_FUNCTION

FUNCTION f_inout : DINT
    VAR_IN_OUT
        x : IA;
    END_VAR
END_FUNCTION

FUNCTION main : DINT
    VAR
        p : __P;
        q : __Q;
    END_VAR
    // CHECK-DAG: {{.*}}argument_self_pointer.st:[[@LINE+1]]:11:{{.*}}error[E126]: Invalid assignment: 'POINTER TO __P' does not implement interface 'IA'
    f_val(p);
    // CHECK-DAG: {{.*}}argument_self_pointer.st:[[@LINE+1]]:13:{{.*}}error[E126]: Type mismatch: expected type 'IA' but received 'POINTER TO __P'
    f_inout(p);
    // CHECK-DAG: {{.*}}argument_self_pointer.st:[[@LINE+1]]:11:{{.*}}error[E126]: Invalid assignment: 'POINTER TO POINTER TO __Q' does not implement interface 'IA'
    f_val(q);
END_FUNCTION
// CHECK: error: Compilation aborted due to critical errors
