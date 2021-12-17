use crate::test_utils::tests::codegen;


#[test]
fn initial_values_in_global_constant_variables() {
    let result = codegen(
        r#"
        VAR_GLOBAL CONSTANT
          c_INT : INT := 7;
          c_3c : INT := 3 * c_INT;
          
          c_BOOL : BOOL := TRUE;
          c_not : BOOL := NOT c_BOOL;
          c_str : STRING := 'Hello';
          c_wstr : WSTRING := "World";

          c_real : REAL := 3.14;
          c_lreal : LREAL := 3.1415;
        END_VAR

        VAR_GLOBAL CONSTANT
          x : INT := c_INT;
          y : INT := c_INT + c_INT;
          z : INT := c_INT + c_3c + 4;

          b : BOOL := c_BOOL;
          nb : BOOL := c_not;
          bb : BOOL := c_not AND NOT c_not;

          str : STRING := c_str;
          wstr : WSTRING := c_wstr;

          r : REAL := c_real / 2;
          tau : LREAL := 2 * c_lreal;
        END_VAR
        "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_global_variables() {
    let result = codegen(
        "
        VAR_GLOBAL
          x : INT := 7;
          y : BOOL := TRUE;
          z : REAL := 3.1415;
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);

}

#[test]
fn initial_values_in_global_variables_out_of_order() {
    let result = codegen(
        "
        VAR_GLOBAL
        x : MyFB;
        END_VAR
        
        PROGRAM prg
        VAR
        x : MyFB;            
        END_VAR
        END_PROGRAM

        //if this fb is moved to the top, the initializer works
        FUNCTION_BLOCK MyFB
          VAR
            x : INT := 77;            
          END_VAR
        END_FUNCTION_BLOCK
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn uninitialized_global_array() {
    let result = codegen(
        "
         VAR_GLOBAL 
           a : ARRAY[0..1] OF BYTE; 
         END_VAR
         ",
    );

   
    insta::assert_snapshot!(result);

}
