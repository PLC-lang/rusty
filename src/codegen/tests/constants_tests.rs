use crate::test_utils::tests::codegen;
use plc_util::filtered_assert_snapshot;

#[test]
fn assigning_const_string_variable() {
    // GIVEN a const string assigned to a variable
    let result = codegen(
        r#"
        PROGRAM main
        VAR
            str : STRING;
        END_VAR
            str := const_str;
        END_PROGRAM

        VAR_GLOBAL CONSTANT
            const_str : STRING := 'global constant string';
        END_VAR
    "#,
    );
    // THEN we expect a memcopy for the assignment
    filtered_assert_snapshot!(result);
}

#[test]
fn assigning_const_array_variable() {
    // GIVEN a const array assigned to a variable
    let result = codegen(
        r#"
        PROGRAM main
        VAR
            arr : ARRAY[0..3] OF INT;
        END_VAR
            arr := const_arr;
        END_PROGRAM

        VAR_GLOBAL CONSTANT
            const_arr : ARRAY[0..3] OF INT := (1,2,3,4);
        END_VAR
    "#,
    );
    // THEN we expect a memcopy for the assignment
    filtered_assert_snapshot!(result);
}

#[test]
fn assigning_const_struct_variable() {
    //GIVEN a const struct assigned to a variable
    let result = codegen(
        r#"
        TYPE Point :
            STRUCT
                x,y : INT;
            END_STRUCT
        END_TYPE

        PROGRAM main
        VAR
            strct : Point;
        END_VAR
            strct := const_strct;
        END_PROGRAM

        VAR_GLOBAL CONSTANT
            const_strct : Point := (x := 1, y := 2);
        END_VAR
    "#,
    );
    // THEN we expect a memcopy for the assignment
    filtered_assert_snapshot!(result);
}
