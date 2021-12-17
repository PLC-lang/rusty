use crate::{test_utils::tests::{codegen, codegen_without_unwrap}, diagnostics::Diagnostic};

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

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%MyStruct = type { i16, i16, i1, i1, float, float }

@MyStruct__init = global %MyStruct { i16 7, i16 0, i1 true, i1 false, float 0x400921CAC0000000, float 0.000000e+00 }
@x = global %MyStruct { i16 7, i16 0, i1 true, i1 false, float 0x400921CAC0000000, float 0.000000e+00 }
"#;

    assert_eq!(result, expected);
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

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%MyStruct = type { i8, i8, i8, i16, i16, i16, i32, i32, i32, i64, i64, i64, float, double }

@MyStruct__init = global %MyStruct { i8 7, i8 7, i8 7, i16 7, i16 7, i16 7, i32 7, i32 7, i32 7, i64 7, i64 7, i64 7, float 0x401ECCCCC0000000, double 7.700000e+00 }
@x = global %MyStruct { i8 7, i8 7, i8 7, i16 7, i16 7, i16 7, i32 7, i32 7, i32 7, i64 7, i64 7, i64 7, float 0x401ECCCCC0000000, double 7.700000e+00 }
"#;

    assert_eq!(result, expected);
}

#[test]
fn initial_values_in_type_alias() {
    let result = codegen(
        "
        TYPE MyInt: INT := 7; END_TYPE 
        VAR_GLOBAL x : MyInt; END_VAR
        ",
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@x = global i16 7
"#;

    assert_eq!(result, expected);
}

#[test]
fn initial_values_in_sub_range_type() {
    let result = codegen(
        "
        TYPE MyInt: INT(0..1000) := 7; END_TYPE 
        VAR_GLOBAL x : MyInt; END_VAR
        ",
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@x = global i16 7
"#;

    assert_eq!(result, expected);
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

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@x0 = global i32 1
@x1 = global i32 1
@x2 = global i32 2
@x3 = global i32 3
"#;

    assert_eq!(result, expected);
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

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@a = global [3 x i8] c"\01\02\03"
@b = global [3 x i16] [i16 1, i16 2, i16 3]
@c = global [3 x i32] [i32 1, i32 2, i32 3]
@d = global [3 x i64] [i64 1, i64 2, i64 3]
@e = global [3 x i8] c"\01\02\03"
@f = global [3 x i16] [i16 1, i16 2, i16 3]
@g = global [3 x i64] [i64 1, i64 2, i64 3]
@h = global [3 x i1] [i1 true, i1 false, i1 true]
"#;

    assert_eq!(result, expected);
}

#[test]
fn initial_values_in_single_dimension_array_type() {
    let result = codegen(
        "
        TYPE MyArray : ARRAY[0..2] OF INT := [1, 2, 3]; END_TYPE
        VAR_GLOBAL x : MyArray; END_VAR
        ",
    );

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@x = global [3 x i16] [i16 1, i16 2, i16 3]
"#;

    assert_eq!(result, expected);
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

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@a = global [2 x [2 x i8]] c"\01\02\03\04"
"#;

    assert_eq!(result, expected);
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

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

@a = global [4 x i8] c"\07\07\07\07"
@b = global [4 x i8] c"\02\07\07\03"
@c = global [10 x i8] c"\00\01\00\01\00\01\00\01\00\01"
@d = global [10 x i8] c"\00\00\01\01\02\00\00\01\01\02"
"#;

    assert_eq!(result, expected);
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

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%MyStruct = type { i32, i32 }

@MyStruct__init = global %MyStruct zeroinitializer
@a = global %MyStruct { i32 3, i32 5 }
@b = global %MyStruct { i32 5, i32 3 }
"#;

    assert_eq!(result, expected);
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

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%MyPoint = type { i32, i32 }
%MyStruct = type { %MyPoint, [4 x i16], i32 }

@MyPoint__init = global %MyPoint zeroinitializer
@MyStruct__init = global %MyStruct zeroinitializer
@a = global %MyStruct { %MyPoint { i32 1, i32 2 }, [4 x i16] [i16 0, i16 1, i16 2, i16 3], i32 7 }
"#;

    assert_eq!(result, expected);
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

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%MyPoint = type { i32 }

@MyPoint__init = global %MyPoint zeroinitializer
@a = global %MyPoint { i32 7 }
"#;

    assert_eq!(result, expected);
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
    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%Point = type { i32, i32, i32 }

@Point__init = global %Point { i32 0, i32 0, i32 7 }
@x = global %Point { i32 1, i32 2, i32 7 }
"#;
    assert_eq!(expected, result);
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

    let expected = r#"; ModuleID = 'main'
source_filename = "main"

%Point = type { i32, i32, i32 }

@Point__init = global %Point { i32 0, i32 0, i32 3 }
@x = global %Point { i32 1, i32 2, i32 3 }
"#;
    assert_eq!(expected, result);
}