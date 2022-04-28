use crate::{
    diagnostics::Diagnostic,
    test_utils::tests::{codegen, codegen_without_unwrap},
};

#[test]
fn initial_values_in_struct_types() {
    let result = codegen(
        "
        TYPE MyStruct:
        STRUCT
          x : INT := 7;
          xx : INT;
          y : BOOL := TRUE;
          yy : BOOL;
          z : REAL := 3.1415;
          zz : REAL;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL x : MyStruct; END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn struct_initial_values_different_data_types() {
    let result = codegen(
        "
        TYPE MyStruct:
        STRUCT
          b  : BYTE   := 7;
          s  : SINT   := 7;
          us : USINT  := 7;
          w  : WORD   := 7;
          i  : INT    := 7;
          ui : UINT   := 7;
          dw : DWORD  := 7;
          di : DINT   := 7;
          udi: UDINT  := 7;
          lw : LWORD  := 7;
          li : LINT   := 7;
          uli: ULINT  := 7;
          r  : REAL   := 7.7;
          lr : LREAL  := 7.7;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL x : MyStruct; END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_type_alias() {
    let result = codegen(
        "
        TYPE MyInt: INT := 7; END_TYPE 
        VAR_GLOBAL x : MyInt; END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_sub_range_type() {
    let result = codegen(
        "
        TYPE MyInt: INT(0..1000) := 7; END_TYPE 
        VAR_GLOBAL x : MyInt; END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn expression_list_as_array_initilization() {
    let result = codegen(
        "
		VAR_GLOBAL
			arr : ARRAY[-1..3] OF INT := 1, 2, 3;
			b_exp : ARRAY[-1..4] OF DINT := 1+3, 2*3, 7-1, 10;
			str : ARRAY[-1..2] OF STRING := 'first', 'second';
		END_VAR
		",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn incomplete_array_initialization() {
    let result = codegen(
        "
		VAR_GLOBAL
			arr : ARRAY[0..5] OF INT := 0, 1, 2;
		END_VAR
		",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn incomplete_array_initialization_with_custom_init_value() {
    let result = codegen(
        "
        TYPE MyInt : INT := 7; END_TYPE

		VAR_GLOBAL
			arr : ARRAY[0..5] OF MyInt := 0, 1, 2;
		END_VAR
		",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn alias_chain_with_lots_of_initializers() {
    let result = codegen(
        "
        TYPE MyInt: MyOtherInt1; END_TYPE 
        VAR_GLOBAL 
          x0 : MyInt; 
          x1 : MyOtherInt1; 
          x2 : MyOtherInt2; 
          x3 : MyOtherInt3; 
        END_VAR
        TYPE MyOtherInt3 : DINT := 3; END_TYPE
        TYPE MyOtherInt1 : MyOtherInt2 := 1; END_TYPE
        TYPE MyOtherInt2 : MyOtherInt3 := 2; END_TYPE
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_single_dimension_array_variable() {
    let result = codegen(
        "
        VAR_GLOBAL 
          a : ARRAY[0..2] OF SINT  := [1, 2, 3]; 
          b : ARRAY[0..2] OF INT  := [1, 2, 3]; 
          c : ARRAY[0..2] OF DINT  := [1, 2, 3]; 
          d : ARRAY[0..2] OF LINT  := [1, 2, 3]; 
          e : ARRAY[0..2] OF USINT  := [1, 2, 3]; 
          f : ARRAY[0..2] OF UINT  := [1, 2, 3]; 
          g : ARRAY[0..2] OF ULINT := [1, 2, 3]; 
          h : ARRAY[0..2] OF BOOL := [TRUE, FALSE, TRUE]; 
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_single_dimension_array_type() {
    let result = codegen(
        "
        TYPE MyArray : ARRAY[0..2] OF INT := [1, 2, 3]; END_TYPE
        VAR_GLOBAL x : MyArray; END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_multi_dimension_array_variable() {
    let result = codegen(
        "
         VAR_GLOBAL 
           a : ARRAY[0..1, 0..1] OF BYTE  := [1,2,3,4]; 
         END_VAR
         ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_array_variable_using_multiplied_statement() {
    let result = codegen(
        "
         VAR_GLOBAL 
           a : ARRAY[0..3] OF BYTE  := [4(7)]; 
           b : ARRAY[0..3] OF BYTE  := [2, 2(7), 3]; 
           c : ARRAY[0..9] OF BYTE  := [5(0,1)]; 
           d : ARRAY[0..9] OF BYTE  := [2(2(0), 2(1), 2)]; 
         END_VAR
         ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn initial_values_in_struct_variable() {
    let result = codegen(
        "
        TYPE MyStruct: STRUCT
          a: DINT;
          b: DINT;
        END_STRUCT
        END_TYPE

         VAR_GLOBAL 
           a : MyStruct  := (a:=3, b:=5); 
           b : MyStruct  := (b:=3, a:=5); 
         END_VAR
         ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn complex_initial_values_in_struct_variable_using_multiplied_statement() {
    let result = codegen(
        "
        TYPE MyPoint: STRUCT
          x: DINT;
          y: DINT;
        END_STRUCT
        END_TYPE
 
        TYPE MyStruct: STRUCT
          point: MyPoint;
          my_array: ARRAY[0..3] OF INT;
          f : DINT;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL 
          a : MyStruct  := (
              point := (x := 1, y:= 2),
              my_array := [0,1,2,3],
              f := 7
            ); 
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn struct_with_one_field_can_be_initialized() {
    let result = codegen(
        "
        TYPE MyPoint: STRUCT
          x: DINT;
        END_STRUCT
        END_TYPE
 
        VAR_GLOBAL 
          a : MyPoint := ( x := 7);
        END_VAR
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn struct_initializer_needs_assignments() {
    let source = "
            TYPE Point: STRUCT
              x: DINT;
              y: DINT;
            END_STRUCT
            END_TYPE
 
            VAR_GLOBAL
                x : Point := (x := 1, 2);
            END_VAR
           ";
    let result = codegen_without_unwrap(source);
    assert_eq!(
        result,
        Err(Diagnostic::codegen_error(
            "struct literal must consist of explicit assignments in the form of member := value",
            (185..186).into()
        ))
    );
    assert_eq!(source[185..186].to_string(), "2".to_string());
}

#[test]
fn struct_initialization_uses_types_default_if_not_provided() {
    // GIVEN a custom dataType MyDINT with initial value of 7
    // AND a struct point that uses it for member z
    // AND a global instance that does not initializes z
    let source = "
            TYPE MyDINT : DINT := 7; END_TYPE

            TYPE Point: STRUCT
              x: DINT;
              y: DINT;
              z: MyDINT;
            END_STRUCT
            END_TYPE
 
            VAR_GLOBAL
                x : Point := (x := 1, y := 2);
            END_VAR
           ";

    //WHEN it is generated
    let result = codegen(source);

    //THEN we expect z to be 7
    insta::assert_snapshot!(result);
}

#[test]
fn struct_initializer_uses_fallback_to_field_default() {
    let source = "
            TYPE Point: STRUCT
              x: DINT;
              y: DINT;
              z: DINT := 3;
            END_STRUCT
            END_TYPE
 
            VAR_GLOBAL
                x : Point := (x := 1, y := 2);
            END_VAR
           ";
    let result = codegen(source);

    insta::assert_snapshot!(result);
}
