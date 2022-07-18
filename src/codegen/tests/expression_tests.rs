// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::test_utils::tests::codegen;

#[test]
fn pointers_in_function_return() {
    let result = codegen(
        r#"FUNCTION func : REF_TO INT
        END_FUNCTION"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn structs_in_function_return() {
    let result = codegen(
        r#"
        TYPE myStruct : STRUCT
            x : INT;
            END_STRUCT
        END_TYPE
        FUNCTION func : myStruct
        END_FUNCTION"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn unary_expressions_can_be_real() {
    let result = codegen(
        r#"
            PROGRAM prg
            VAR
                a,b : REAL;
            END_VAR
                b := -2.0;
                a := -b;
            END_PROGRAM
        "#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn type_mix_in_call() {
    let result = codegen(
        "
        FUNCTION foo : INT
        VAR_INPUT
            in : INT;
        END_VAR
        END_FUNCTION
        FUNCTION baz : INT
            foo(1.5);
        END_FUNCTION
    ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_comparison_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_EQUAL : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR

        END_FUNCTION

        FUNCTION baz : INT
            VAR a,b : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := 'a' = 'b';
            result := a = b;
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_equal_with_constant_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_EQUAL : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION baz : INT
            VAR a,b : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := a = 'b';
            result := 'a' = b;
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_less_with_constant_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_LESS : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION baz : INT
            VAR a : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := a < 'b';
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_greater_with_constant_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_GREATER : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION baz : INT
            VAR a : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := a > 'b';
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_not_equal_with_constant_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_EQUAL : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION baz : INT
            VAR a : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := a <> 'b';
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_smaller_or_equal_with_constant_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_LESS : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION
        FUNCTION STRING_EQUAL : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION baz : INT
            VAR a,b : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := a <= 'b';
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn string_greater_or_equal_with_constant_test() {
    let result = codegen(
        r#"
        FUNCTION STRING_GREATER : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION
        FUNCTION STRING_EQUAL : BOOL
            VAR_INPUT op1, op2: STRING[1024] END_VAR
        END_FUNCTION

        FUNCTION baz : INT
            VAR a,b : STRING; END_VAR
            VAR result : BOOL; END_VAR

            result := a >= 'b';
        END_FUNCTION
    "#,
    );

    insta::assert_snapshot!(result);
}

#[test]
fn ranged_number_type_comparing_test() {
    let result = codegen(
        r#"
        FUNCTION baz : INT
            VAR x,y : INT(0..500); END_VAR;

            x = 3;
            x < y;
            y <= 0;
        END_FUNCTION
    "#,
    );

    //should result in normal number-comparisons
    insta::assert_snapshot!(result);
}

#[test]
fn aliased_ranged_number_type_comparing_test() {
    let result = codegen(
        r#"
        TYPE MyInt: INT(0..500); END_TYPE
        FUNCTION baz : INT
            VAR x,y : MyInt; END_VAR;

            x = 3;
            x < y;
            y <= 0;
        END_FUNCTION
    "#,
    );

    //should result in normal number-comparisons
    insta::assert_snapshot!(result);
}

#[test]
fn aliased_number_type_comparing_test() {
    let result = codegen(
        r#"
        TYPE MyInt: INT; END_TYPE

        FUNCTION baz : INT
            VAR x,y : MyInt; END_VAR;

            x = 3;
            x < y;
            y <= 0;
        END_FUNCTION
    "#,
    );

    //should result in normal number-comparisons
    insta::assert_snapshot!(result);
}

#[test]
fn cast_pointer_to_lword() {
    let result = codegen(
        r#"
        FUNCTION baz : INT
            VAR 
                ptr_x : POINTER TO INT; 
                y : LWORD; 
            END_VAR;

            y := ptr_x;
        END_FUNCTION
    "#,
    );

    //should result in normal number-comparisons
    insta::assert_snapshot!(result);
}

#[test]
fn cast_lword_to_pointer() {
    let result = codegen(
        r#"
        FUNCTION baz : INT
            VAR 
                ptr_x : POINTER TO INT; 
                y : LWORD;
            END_VAR;

            ptr_x := y;
        END_FUNCTION
    "#,
    );

    //should result in normal number-comparisons
    insta::assert_snapshot!(result);
}

#[test]
fn cast_between_pointer_types() {
    let result = codegen(
        r#"
        PROGRAM baz
            VAR 
                ptr_x : POINTER TO BYTE; 
                y : WORD;
            END_VAR;

            ptr_x := &y;
        END_PROGRAM
    "#,
    );

    //should result in bitcast conversion when assigning to ptr_x
    insta::assert_snapshot!(result);
}

#[test]
fn unnecessary_casts_between_pointer_types() {
    let result = codegen(
        r#"
        TYPE MyByte : BYTE; END_TYPE
        
        PROGRAM baz
            VAR 
                ptr : POINTER TO BYTE; 
                b : BYTE;
                si : SINT;
                mb : MyByte;
            END_VAR;

            ptr := &b; //no cast necessary
            ptr := &si; //no cast necessary
            ptr := &mb; //no cast necessary
        END_PROGRAM
    "#,
    );

    //should not result in bitcast
    insta::assert_snapshot!(result);
}

#[test]
fn access_string_via_byte_array() {
    let result = codegen(
        r#"
        TYPE MyByte : BYTE; END_TYPE
        
        PROGRAM baz
            VAR 
                str: STRING[10];
                ptr : POINTER TO BYTE; 
                bytes : POINTER TO ARRAY[0..9] OF BYTE;
            END_VAR;

            ptr := &str; //bit-cast expected
            bytes := &str;
        END_PROGRAM
    "#,
    );

    //should result in bitcasts
    insta::assert_snapshot!(result);
}

#[test]
fn pointer_arithmetics() {
    // codegen should be successful for binary expression for pointer<->int / int<->pointer / pointer<->pointer
    let result = codegen(
        "
		PROGRAM main
		VAR
			x : INT := 10;
			y : INT := 20;
			pt : REF_TO INT;
			comp : BOOL;
		END_VAR
		pt := &(x);

		(* +/- *)
		pt := pt + 1;
		pt := pt + 1 + 1;
		pt := 1 + pt;
		pt := pt - y;
		pt := 1 + pt + 1;
		pt := pt - y - 1;
		pt := 1 + 1 + pt ;
		pt := y + pt - y ;
		pt := y + y + pt ;

		(* compare pointer-pointer / pointer-int *)
		comp := pt = pt;
		comp := pt <> y;
		comp := pt < pt;
		comp := pt > y;
		comp := pt <= pt;
		comp := y >= pt;
		END_PROGRAM
		",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn pointer_arithmetics_function_call() {
    // codegen should be successful for binary expression for pointer<->int / int<->pointer / pointer<->pointer
    let result = codegen(
        "
        FUNCTION foo : LINT
        END_FUNCTION

		PROGRAM main
		VAR
			pt : REF_TO INT;
            x : INT;
			comp : BOOL;
		END_VAR
		pt := &(x);

		(* +/- *)
		pt := pt + foo();

		(* compare pointer-pointer / pointer-int *)
		comp := pt = pt;
		comp := pt <> foo();
		comp := pt < pt;
		comp := pt > foo();
		comp := pt <= pt;
		comp := foo() >= pt;
		END_PROGRAM
		",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn nested_call_statements() {
    // GIVEN some nested call statements
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            a : DINT;
        END_VAR
        END_FUNCTION

		PROGRAM main
            foo(foo(2));
		END_PROGRAM
		",
    );
    // WHEN compiled
    // WE expect a flat sequence of calls, no regions and branching
    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_adr() {
    // GIVEN some nested call statements
    let result = codegen(
        "
		PROGRAM main
        VAR
            a : REF_TO DINT;
            b : DINT;
        END_VAR
            a := ADR(b);
		END_PROGRAM
		",
    );
    // WHEN compiled
    // We expect a direct conversion to lword and subsequent assignment (no call)
    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_ref() {
    // GIVEN some nested call statements
    let result = codegen(
        "
		PROGRAM main
        VAR
            a : REF_TO DINT;
            b : DINT;
        END_VAR
            a := REF(b);
		END_PROGRAM
		",
    );
    // WHEN compiled
    // We expect a direct conversion and subsequent assignment to pointer(no call)
    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_mux() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b,c,d,e : DINT;
        END_VAR
            a := MUX(3, b,c,d,e); //3 = d
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_sel() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b,c : DINT;
        END_VAR
            a := SEL(TRUE, b,c);
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_sel_as_expression() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b,c : DINT;
        END_VAR
            a := SEL(TRUE, b,c) + 10;
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_move() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b : DINT;
        END_VAR
            a := MOVE(b);
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn test_max_int() {
    let result = codegen(
        r"
    {external}
    FUNCTION MAX<U : ANY> : U
    VAR_INPUT in : {sized} U...; END_VAR
    END_FUNCTION
    
    FUNCTION main : INT
    main := MAX(INT#5,INT#2,INT#1,INT#3,INT#4,INT#7,INT#-1);
    END_FUNCTION",
    );

    insta::assert_snapshot!(result);
}
