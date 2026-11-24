// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK --implicit-check-not='{{.*}}error[E037]{{.*}}' %s

// A constant integer value other than 0 or 1 is no BOOL value. The compiler rejects it in a
// body assignment, an initializer and a call argument, whatever form computes the value.
// The values 0 and 1, non-constant integers, and members whose name matches a constant stay valid.

TYPE State : (idle, running, done); END_TYPE

VAR_GLOBAL CONSTANT
    cTwo : INT := 2;
    cOne : INT := 1;
    cFalse : BOOL := 0;
END_VAR

FUNCTION takesBool : BOOL
VAR_INPUT
    x : BOOL;
END_VAR
    takesBool := x;
END_FUNCTION

FUNCTION_BLOCK withProperty
VAR
    _p : BOOL;
END_VAR
PROPERTY_SET prop : BOOL
    _p := prop;
END_PROPERTY
END_FUNCTION_BLOCK

FUNCTION_BLOCK Base
VAR_OUTPUT
    done : BOOL;
END_VAR
END_FUNCTION_BLOCK

FUNCTION_BLOCK Derived EXTENDS Base
VAR
    last : BOOL;
END_VAR
    last := done;
    last := SUPER^.done;
END_FUNCTION_BLOCK

PROGRAM mainProg
VAR
    inst : withProperty;
    // CHECK-DAG: {{.*}}bool_constant_values.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign the value 2 to 'BOOL'
    fromConstant : BOOL := cTwo;
    validFromConstant : BOOL := cOne;
    b : BOOL;
    i : DINT;
    d : Derived;
END_VAR
    // CHECK-DAG: {{.*}}bool_constant_values.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign the value 5 to 'BOOL'
    b := INT#5;
    // CHECK-DAG: {{.*}}bool_constant_values.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign the value 2 to 'BOOL'
    b := 1 + 1;
    // CHECK-DAG: {{.*}}bool_constant_values.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign the value 3 to 'BOOL'
    takesBool(INT#3);
    // CHECK-DAG: {{.*}}bool_constant_values.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign the value 4 to 'BOOL'
    inst.prop := INT#4;
    inst.prop := INT#1;
    b := INT#1;
    b := 1 * 1;
    b := i;
    takesBool(cOne);
    b := d.done;
    b := NOT cFalse;
END_PROGRAM
