// RUN: %COMPILE %s --error-format clang 2>&1 | %CHECK %s
// A FOR loop reports the operand conversion warnings of its end test `<next value> > <end>` (or `<`). The next
// value is a LINT for a counter of up to 32 bits and has the counter's type otherwise. The other generated
// statements report none.

FUNCTION main : DINT
    VAR
        c_int : INT;
        c_udint : UDINT;
        c_ulint : ULINT;
        c_lword : LWORD;
        step : DINT;
        end_d : DINT;
        end_u : UDINT;
        end_l : LINT;
        end_ul : ULINT;
        end_r : REAL;
    END_VAR

    // CHECK-NOT: {{.*}}warning{{.*}}
    FOR c_int := 10 TO 1 BY -3 DO END_FOR
    FOR c_int := 1 TO end_u BY step DO END_FOR
    FOR c_udint := 1 TO end_d DO END_FOR
    // CHECK: {{.*}}.st:[[@LINE+1]]:25:{{.*}}: warning[E161]: Operand of type 'ULINT' is converted to 'LINT', which may change its sign.
    FOR c_udint := 1 TO end_ul DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:25:{{.*}}: warning[E161]: Operand of type 'DINT' is converted to 'ULINT', which may change its sign.
    FOR c_ulint := 1 TO end_d DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:9:{{.*}}: warning[E161]: Operand of type 'ULINT' is converted to 'LINT', which may change its sign.
    FOR c_ulint := 1 TO end_l DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:9:{{.*}}: warning[E161]: Operand of type 'LWORD' is converted to 'LINT', which may change its sign.
    FOR c_lword := 1 TO end_l BY step DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:9:{{.*}}: warning[E162]: Implicit conversion from 'LINT' to 'LREAL' may lose precision.
    FOR c_int := 1 TO end_r DO END_FOR
END_FUNCTION

// CHECK-NOT: {{.*}}warning{{.*}}
