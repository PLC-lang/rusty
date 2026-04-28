// RUN: not %PLC --check --error-format clang --error-config %S/../../util/diagnostics_e127_as_error.json %s 2>&1 | %CHECK %s
// E127 is promoted to error so we can assert both diagnostics are emitted.
// CHECK-DAG: {{.*}}array_partial_init_warning_type_and_var.st:{{[0-9]+}}:{{[0-9]+}}:{{.*}}error[E127]: Fewer initial values for array `a` than expected. Expected 5, found 2.
// CHECK-DAG: {{.*}}array_partial_init_warning_type_and_var.st:{{[0-9]+}}:{{[0-9]+}}:{{.*}}error[E127]: Fewer initial values for array `userArrayType` than expected. Expected 5, found 4.

PROGRAM PLC_PRG
    VAR CONSTANT
        x : INT := 0;
    END_VAR
    VAR
        a : userArrayType := [x,x];
    END_VAR
END_PROGRAM

TYPE userArrayType : ARRAY [1..5] OF INT := [1,2,4,5];
END_TYPE
