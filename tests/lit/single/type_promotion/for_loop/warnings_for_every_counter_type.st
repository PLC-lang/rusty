// RUN: %COMPILE %s --error-format clang 2>&1 | %CHECK %s
// A FOR loop reports the operand conversion warnings of the comparison `<counter> > <end>` (or `<`), as the
// same comparison does in user code. The generated `<counter> + <step>` and its comparison report none.

FUNCTION main : DINT
    VAR
        c_sint : SINT;
        c_int : INT;
        c_dint : DINT;
        c_lint : LINT;
        c_usint : USINT;
        c_uint : UINT;
        c_udint : UDINT;
        c_ulint : ULINT;
        c_byte : BYTE;
        c_word : WORD;
        c_dword : DWORD;
        c_lword : LWORD;
        step : DINT;
        end_d : DINT;
        end_u : UDINT;
        end_l : LINT;
        end_ul : ULINT;
    END_VAR

    FOR c_sint := 10 TO 1 BY -3 DO END_FOR
    FOR c_sint := 1 TO 10 BY step DO END_FOR
    FOR c_sint := 1 TO end_d DO END_FOR
    // CHECK: {{.*}}.st:[[@LINE+1]]:24:{{.*}}: warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
    FOR c_sint := 1 TO end_u BY 2 DO END_FOR
    FOR c_sint := 1 TO end_l DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:9:{{.*}}: warning[E161]: Operand of type 'SINT' is converted to 'ULINT', which may change its sign.
    FOR c_sint := 1 TO end_ul DO END_FOR

    FOR c_int := 10 TO 1 BY -3 DO END_FOR
    FOR c_int := 1 TO 10 BY step DO END_FOR
    FOR c_int := 1 TO end_d DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:23:{{.*}}: warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
    FOR c_int := 1 TO end_u BY 2 DO END_FOR
    FOR c_int := 1 TO end_l DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:9:{{.*}}: warning[E161]: Operand of type 'INT' is converted to 'ULINT', which may change its sign.
    FOR c_int := 1 TO end_ul DO END_FOR

    FOR c_dint := 10 TO 1 BY -3 DO END_FOR
    FOR c_dint := 1 TO 10 BY step DO END_FOR
    FOR c_dint := 1 TO end_d DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:24:{{.*}}: warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
    FOR c_dint := 1 TO end_u BY 2 DO END_FOR
    FOR c_dint := 1 TO end_l DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:9:{{.*}}: warning[E161]: Operand of type 'DINT' is converted to 'ULINT', which may change its sign.
    FOR c_dint := 1 TO end_ul DO END_FOR

    FOR c_lint := 10 TO 1 BY -3 DO END_FOR
    FOR c_lint := 1 TO 10 BY step DO END_FOR
    FOR c_lint := 1 TO end_d DO END_FOR
    FOR c_lint := 1 TO end_u BY 2 DO END_FOR
    FOR c_lint := 1 TO end_l DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:24:{{.*}}: warning[E161]: Operand of type 'ULINT' is converted to 'LINT', which may change its sign.
    FOR c_lint := 1 TO end_ul DO END_FOR

    FOR c_usint := 10 TO 1 BY -3 DO END_FOR
    FOR c_usint := 1 TO 10 BY step DO END_FOR
    FOR c_usint := 1 TO end_d DO END_FOR
    FOR c_usint := 1 TO end_u BY 2 DO END_FOR
    FOR c_usint := 1 TO end_l DO END_FOR
    FOR c_usint := 1 TO end_ul DO END_FOR

    FOR c_uint := 10 TO 1 BY -3 DO END_FOR
    FOR c_uint := 1 TO 10 BY step DO END_FOR
    FOR c_uint := 1 TO end_d DO END_FOR
    FOR c_uint := 1 TO end_u BY 2 DO END_FOR
    FOR c_uint := 1 TO end_l DO END_FOR
    FOR c_uint := 1 TO end_ul DO END_FOR

    FOR c_udint := 10 TO 1 BY -3 DO END_FOR
    FOR c_udint := 1 TO 10 BY step DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:9:{{.*}}: warning[E161]: Operand of type 'UDINT' is converted to 'DINT', which may change its sign.
    FOR c_udint := 1 TO end_d DO END_FOR
    FOR c_udint := 1 TO end_u BY 2 DO END_FOR
    FOR c_udint := 1 TO end_l DO END_FOR
    FOR c_udint := 1 TO end_ul DO END_FOR

    FOR c_ulint := 10 TO 1 BY -3 DO END_FOR
    FOR c_ulint := 1 TO 10 BY step DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:25:{{.*}}: warning[E161]: Operand of type 'DINT' is converted to 'ULINT', which may change its sign.
    FOR c_ulint := 1 TO end_d DO END_FOR
    FOR c_ulint := 1 TO end_u BY 2 DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:9:{{.*}}: warning[E161]: Operand of type 'ULINT' is converted to 'LINT', which may change its sign.
    FOR c_ulint := 1 TO end_l DO END_FOR
    FOR c_ulint := 1 TO end_ul DO END_FOR

    FOR c_byte := 10 TO 1 BY -3 DO END_FOR
    FOR c_byte := 1 TO 10 BY step DO END_FOR
    FOR c_byte := 1 TO end_d DO END_FOR
    FOR c_byte := 1 TO end_u BY 2 DO END_FOR
    FOR c_byte := 1 TO end_l DO END_FOR
    FOR c_byte := 1 TO end_ul DO END_FOR

    FOR c_word := 10 TO 1 BY -3 DO END_FOR
    FOR c_word := 1 TO 10 BY step DO END_FOR
    FOR c_word := 1 TO end_d DO END_FOR
    FOR c_word := 1 TO end_u BY 2 DO END_FOR
    FOR c_word := 1 TO end_l DO END_FOR
    FOR c_word := 1 TO end_ul DO END_FOR

    FOR c_dword := 10 TO 1 BY -3 DO END_FOR
    FOR c_dword := 1 TO 10 BY step DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:9:{{.*}}: warning[E161]: Operand of type 'DWORD' is converted to 'DINT', which may change its sign.
    FOR c_dword := 1 TO end_d DO END_FOR
    FOR c_dword := 1 TO end_u BY 2 DO END_FOR
    FOR c_dword := 1 TO end_l DO END_FOR
    FOR c_dword := 1 TO end_ul DO END_FOR

    FOR c_lword := 10 TO 1 BY -3 DO END_FOR
    FOR c_lword := 1 TO 10 BY step DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:25:{{.*}}: warning[E161]: Operand of type 'DINT' is converted to 'LWORD', which may change its sign.
    FOR c_lword := 1 TO end_d DO END_FOR
    FOR c_lword := 1 TO end_u BY 2 DO END_FOR
    // CHECK-NEXT: {{.*}}.st:[[@LINE+1]]:9:{{.*}}: warning[E161]: Operand of type 'LWORD' is converted to 'LINT', which may change its sign.
    FOR c_lword := 1 TO end_l DO END_FOR
    FOR c_lword := 1 TO end_ul DO END_FOR

END_FUNCTION
// CHECK-NOT: {{.*}}warning{{.*}}
