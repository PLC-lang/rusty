// RUN: rm -f %t.out
// RUN: not %COMPILE --error-format clang %s 2>&1 | %CHECK --implicit-check-not='{{.*}}error[E037]{{.*}}' %s
// RUN: test ! -e %t.out

// A variable initializer must have a type that is assignable to the declared type, no matter
// where the variable is declared. These declarations used to pass validation and then abort in
// code generation.

TYPE MyStruct : STRUCT
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'DINT' to 'STRING'
    member : STRING := 3;
END_STRUCT END_TYPE

VAR_GLOBAL
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'STRING' to 'DINT'
    gInt : DINT := 'abc';
END_VAR

FUNCTION_BLOCK MyFb
VAR
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'DINT' to 'WSTRING'
    fbStr : WSTRING := 3;
END_VAR
END_FUNCTION_BLOCK

PROGRAM mainProg
VAR
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'DINT' to 'STRING'
    str : STRING := 3;
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'REAL' to 'STRING'
    strFromReal : STRING := 3.5;
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'BOOL' to 'STRING'
    strFromBool : STRING := TRUE;
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'STRING' to 'INT'
    int : INT := 'abc';
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'BOOL' to 'DINT'
    dintFromBool : DINT := TRUE;
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'BOOL' to 'INT'
    intFromBool : INT := TRUE;
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'BOOL' to 'REAL'
    realFromBool : REAL := TRUE;
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'DINT' to 'BOOL'
    boolFromInt : BOOL := 5;
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'BOOL' to 'DINT'
    dintsFromBool : ARRAY[0..0] OF DINT := [TRUE];
    // CHECK-DAG: {{.*}}initializer_type_mismatch.st:[[@LINE+1]]:{{[0-9]+}}:{{.*}}error[E037]: Invalid assignment: cannot assign 'STRING' to 'DINT'
    dintsFromStr : ARRAY[0..0] OF DINT := ['a'];

    // Compatible initializers are not reported
    validStr : STRING := 'abc';
    validInt : DINT := 3;
    validReal : LREAL := 3;
    validStruct : MyStruct := (member := 'abc');
    validArray : ARRAY[0..1] OF DINT := [1, 2];
    validBool : BOOL := 1;
    validBools : ARRAY[0..1] OF BOOL := [TRUE, 0];
    validNested : ARRAY[0..1] OF ARRAY[0..0] OF INT := [[1], [2]];
    validFb : MyFb;
END_VAR
END_PROGRAM

// CHECK: error: Compilation aborted due to critical errors
