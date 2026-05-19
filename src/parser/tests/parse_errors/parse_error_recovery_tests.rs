use crate::test_utils::tests::parse_buffered;
use insta::assert_snapshot;
use plc_ast::{
    ast::{AstStatement, DataType, GenericBinding, TypeNature},
    control_statements::AstControlStatement,
};

#[test]
fn type_with_wrong_end_keyword_recovers_at_next_top_level_declaration() {
    let src = r"
        TYPE Position:
            STRUCT
                x: DINT;
            END_STRUCT
        END_POSITION

        FUNCTION_BLOCK FbA
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E007]: Unexpected token: expected `END_TYPE` but found END_POSITION
      ┌─ <internal>:6:9
      │
    6 │         END_POSITION
      │         ^^^^^^^^^^^^ Unexpected token: expected `END_TYPE` but found END_POSITION

    error[E006]: Missing expected Token `END_TYPE`
      ┌─ <internal>:8:9
      │
    8 │         FUNCTION_BLOCK FbA
      │         ^^^^^^^^^^^^^^ Missing expected Token `END_TYPE`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "FbA");
}

#[test]
fn type_missing_end_type_recovers_at_next_top_level_declaration() {
    let src = r"
        TYPE Position:
            STRUCT
                x: DINT;
            END_STRUCT

        FUNCTION main
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_TYPE`
      ┌─ <internal>:7:9
      │
    7 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `END_TYPE`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "main");
}

#[test]
fn struct_missing_end_recovers_at_next_top_level_declaration() {
    let src = r"
        TYPE Position:
            STRUCT
                x: DINT;

        FUNCTION main
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_STRUCT`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `END_STRUCT`

    error[E006]: Missing expected Token `END_TYPE`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `END_TYPE`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "main");
}

#[test]
fn struct_missing_member_semicolon_recovers_at_next_member() {
    let src = r"
        TYPE Position:
            STRUCT
                x: DINT
                y: DINT;
            END_STRUCT
        END_TYPE
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `;`
      ┌─ <internal>:5:17
      │
    5 │                 y: DINT;
      │                 ^ Missing expected Token `;`
    ");

    let DataType::StructType { variables, .. } = &unit.user_types[0].data_type else {
        panic!("expected struct type");
    };
    assert_eq!(variables.len(), 2);
    assert_eq!(variables[0].name, "x");
    assert_eq!(variables[1].name, "y");
}

#[test]
fn struct_member_missing_colon_before_type_recovers_at_type_name() {
    let src = r"
        TYPE Position:
            STRUCT
                x INT;
                y: INT;
            END_STRUCT
        END_TYPE
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `:`
      ┌─ <internal>:4:19
      │
    4 │                 x INT;
      │                   ^^^ Missing expected Token `:`
    ");

    let DataType::StructType { variables, .. } = &unit.user_types[0].data_type else {
        panic!("expected struct type");
    };
    assert_eq!(variables.len(), 2);
    assert_eq!(variables[0].name, "x");
    assert_eq!(variables[1].name, "y");
}

#[test]
fn enum_missing_close_paren_recovers_at_next_top_level_declaration() {
    let src = r"
        TYPE Color:
            (
                Red,
                Green

        FUNCTION main
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `)`
      ┌─ <internal>:7:9
      │
    7 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `)`

    error[E006]: Missing expected Token `;`
      ┌─ <internal>:7:9
      │
    7 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `;`

    error[E006]: Missing expected Token `END_TYPE`
      ┌─ <internal>:7:9
      │
    7 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `END_TYPE`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "main");
}

#[test]
fn enum_missing_comma_recovers_at_next_enum_element() {
    let src = r"
        TYPE Color:
            (
                Red
                Green,
                Blue
            );
        END_TYPE
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `,`
      ┌─ <internal>:5:17
      │
    5 │                 Green,
      │                 ^^^^^ Missing expected Token `,`
    ");

    let DataType::EnumType { elements, .. } = &unit.user_types[0].data_type else {
        panic!("expected enum type");
    };
    let AstStatement::ExpressionList(elements) = elements.get_stmt() else {
        panic!("expected enum element list");
    };
    assert_eq!(elements.len(), 3);
}

#[test]
fn array_missing_of_recovers_at_next_top_level_declaration() {
    let src = r"
        TYPE Matrix:
            ARRAY [1..10]

        FUNCTION main
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `OF`
      ┌─ <internal>:5:9
      │
    5 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `OF`

    error[E006]: Missing expected Token `;`
      ┌─ <internal>:5:9
      │
    5 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `;`

    error[E006]: Missing expected Token `END_TYPE`
      ┌─ <internal>:5:9
      │
    5 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `END_TYPE`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "main");
}

#[test]
fn array_missing_close_bracket_keeps_of_element_type() {
    let src = r"
        TYPE Matrix:
            ARRAY [1..10 OF DINT;
        END_TYPE
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `]`
      ┌─ <internal>:3:26
      │
    3 │             ARRAY [1..10 OF DINT;
      │                          ^^ Missing expected Token `]`
    ");
    assert_eq!(unit.user_types.len(), 1);
    assert!(matches!(unit.user_types[0].data_type, DataType::ArrayType { .. }));
}

#[test]
fn array_bounds_missing_comma_recovers_at_next_range() {
    let src = r"
        TYPE Matrix:
            ARRAY [1..10 20..30] OF DINT;
        END_TYPE

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `,`
      ┌─ <internal>:3:26
      │
    3 │             ARRAY [1..10 20..30] OF DINT;
      │                          ^^ Missing expected Token `,`
    ");
    assert_eq!(unit.user_types.len(), 1);
    assert!(matches!(unit.user_types[0].data_type, DataType::ArrayType { .. }));
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "other");
}

#[test]
fn variable_array_missing_of_recovers_at_next_variable() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                xs: ARRAY [1..10]
                y: DINT;
            END_VAR
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `OF`
      ┌─ <internal>:5:17
      │
    5 │                 y: DINT;
      │                 ^ Missing expected Token `OF`

    error[E006]: Missing expected Token `;`
      ┌─ <internal>:5:17
      │
    5 │                 y: DINT;
      │                 ^ Missing expected Token `;`
    ");

    let variables = &unit.pous[0].variable_blocks[0].variables;
    assert_eq!(variables.len(), 1);
    assert_eq!(variables[0].name, "y");
}

#[test]
fn variable_array_bounds_missing_comma_recovers_at_next_range() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                xs: ARRAY [1..10 20..30] OF DINT;
                y: DINT;
            END_VAR
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `,`
      ┌─ <internal>:4:34
      │
    4 │                 xs: ARRAY [1..10 20..30] OF DINT;
      │                                  ^^ Missing expected Token `,`
    ");

    let variables = &unit.pous[0].variable_blocks[0].variables;
    assert_eq!(variables.len(), 2);
    assert_eq!(variables[0].name, "xs");
    assert_eq!(variables[1].name, "y");
}

#[test]
fn variable_missing_colon_before_type_recovers_at_type_name() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                x INT;
                y: INT;
            END_VAR
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `:`
      ┌─ <internal>:4:19
      │
    4 │                 x INT;
      │                   ^^^ Missing expected Token `:`
    ");

    let variables = &unit.pous[0].variable_blocks[0].variables;
    assert_eq!(variables.len(), 2);
    assert_eq!(variables[0].name, "x");
    assert_eq!(variables[1].name, "y");
}

#[test]
fn string_size_missing_close_bracket_recovers_at_next_variable() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                s: STRING[10
                x: DINT;
            END_VAR
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `]`
      ┌─ <internal>:5:17
      │
    5 │                 x: DINT;
      │                 ^ Missing expected Token `]`

    error[E006]: Missing expected Token `;`
      ┌─ <internal>:5:17
      │
    5 │                 x: DINT;
      │                 ^ Missing expected Token `;`
    ");

    let variables = &unit.pous[0].variable_blocks[0].variables;
    assert_eq!(variables.len(), 2);
    assert_eq!(variables[0].name, "s");
    assert_eq!(variables[1].name, "x");
}

#[test]
fn string_size_missing_close_bracket_recovers_at_next_top_level_declaration() {
    let src = r"
        TYPE Name:
            STRING[10

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `]`
      ┌─ <internal>:5:9
      │
    5 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `]`

    error[E006]: Missing expected Token `;`
      ┌─ <internal>:5:9
      │
    5 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `;`

    error[E006]: Missing expected Token `END_TYPE`
      ┌─ <internal>:5:9
      │
    5 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `END_TYPE`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "other");
}

#[test]
fn reference_type_missing_base_recovers_at_next_top_level_declaration() {
    let src = r"
        TYPE RefInt:
            REF_TO

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token DataTypeDefinition
      ┌─ <internal>:5:9
      │
    5 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token DataTypeDefinition

    error[E006]: Missing expected Token `;`
      ┌─ <internal>:5:9
      │
    5 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `;`

    error[E006]: Missing expected Token `END_TYPE`
      ┌─ <internal>:5:9
      │
    5 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `END_TYPE`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "other");
}

#[test]
fn reference_variable_missing_base_recovers_at_next_variable() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                r: REF_TO
                x: DINT;
            END_VAR
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token DataTypeDefinition
      ┌─ <internal>:5:17
      │
    5 │                 x: DINT;
      │                 ^ Missing expected Token DataTypeDefinition

    error[E006]: Missing expected Token `;`
      ┌─ <internal>:5:17
      │
    5 │                 x: DINT;
      │                 ^ Missing expected Token `;`
    ");
    let variables = &unit.pous[0].variable_blocks[0].variables;
    assert_eq!(variables.len(), 1);
    assert_eq!(variables[0].name, "x");
}

#[test]
fn interface_extends_missing_comma_recovers_at_next_interface_name() {
    let src = r"
        INTERFACE IBase
        END_INTERFACE

        INTERFACE IOther
        END_INTERFACE

        INTERFACE IFoo EXTENDS IBase IOther
        END_INTERFACE
    ";

    let (_, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `,`
      ┌─ <internal>:8:38
      │
    8 │         INTERFACE IFoo EXTENDS IBase IOther
      │                                      ^^^^^^ Missing expected Token `,`
    ");
}

#[test]
fn implements_missing_comma_recovers_at_next_interface_name() {
    let src = r"
        INTERFACE IBase
        END_INTERFACE

        INTERFACE IOther
        END_INTERFACE

        FUNCTION_BLOCK FbA IMPLEMENTS IBase IOther
        END_FUNCTION_BLOCK
    ";

    let (_, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `,`
      ┌─ <internal>:8:45
      │
    8 │         FUNCTION_BLOCK FbA IMPLEMENTS IBase IOther
      │                                             ^^^^^^ Missing expected Token `,`
    ");
}

#[test]
fn function_generics_missing_comma_recovers_at_next_generic_binding() {
    let src = r"
        FUNCTION test<T: ANY R: ANY_NUM> : R
        END_FUNCTION

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `,`
      ┌─ <internal>:2:30
      │
    2 │         FUNCTION test<T: ANY R: ANY_NUM> : R
      │                              ^ Missing expected Token `,`
    ");
    assert_eq!(unit.pous.len(), 2);
    assert_eq!(unit.pous[0].generics.len(), 2);
    assert_eq!(unit.pous[0].generics[0], GenericBinding { name: "T".into(), nature: TypeNature::Any });
    assert_eq!(unit.pous[0].generics[1], GenericBinding { name: "R".into(), nature: TypeNature::Num });
    assert_eq!(unit.pous[1].name, "other");
}

#[test]
fn method_generics_missing_comma_recovers_at_next_generic_binding() {
    let src = r"
        CLASS C
            METHOD test<T: ANY R: ANY_NUM> : R
            END_METHOD
        END_CLASS

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `,`
      ┌─ <internal>:3:32
      │
    3 │             METHOD test<T: ANY R: ANY_NUM> : R
      │                                ^ Missing expected Token `,`
    ");
    assert_eq!(unit.pous.len(), 3);
    assert_eq!(unit.pous[1].name, "C.test");
    assert_eq!(unit.pous[1].generics.len(), 2);
    assert_eq!(unit.pous[2].name, "other");
}

#[test]
fn function_generics_missing_close_recovers_at_return_type() {
    let src = r"
        FUNCTION test<T: ANY, R: ANY_NUM : R
        END_FUNCTION

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `>`
      ┌─ <internal>:2:42
      │
    2 │         FUNCTION test<T: ANY, R: ANY_NUM : R
      │                                          ^ Missing expected Token `>`
    ");
    assert_eq!(unit.pous.len(), 2);
    assert_eq!(unit.pous[0].generics.len(), 2);
    assert_eq!(unit.pous[1].name, "other");
}

#[test]
fn call_missing_close_paren_recovers_at_statement_semicolon() {
    let src = r"
        FUNCTION main
            foo(1, 2;
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `)`
      ┌─ <internal>:3:21
      │
    3 │             foo(1, 2;
      │                     ^ Missing expected Token `)`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.implementations.len(), 1);
}

#[test]
fn subscript_missing_close_bracket_recovers_at_statement_semicolon() {
    let src = r"
        FUNCTION main
            x := arr[1;
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `]`
      ┌─ <internal>:3:23
      │
    3 │             x := arr[1;
      │                       ^ Missing expected Token `]`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.implementations.len(), 1);
}

#[test]
fn struct_initializer_missing_close_paren_recovers_at_variable_semicolon() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                p: Position := (x := 1, y := 2;
                z: DINT;
            END_VAR
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `)`
      ┌─ <internal>:4:47
      │
    4 │                 p: Position := (x := 1, y := 2;
      │                                               ^ Missing expected Token `)`
    ");

    let variables = &unit.pous[0].variable_blocks[0].variables;
    assert_eq!(variables.len(), 2);
    assert_eq!(variables[0].name, "p");
    assert_eq!(variables[1].name, "z");
}

#[test]
fn struct_initializer_missing_comma_recovers_at_next_field_assignment() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                p: Position := (x := 1 y := 2);
                z: DINT;
            END_VAR
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `,`
      ┌─ <internal>:4:40
      │
    4 │                 p: Position := (x := 1 y := 2);
      │                                        ^ Missing expected Token `,`
    ");

    let variables = &unit.pous[0].variable_blocks[0].variables;
    assert_eq!(variables.len(), 2);
    assert_eq!(variables[0].name, "p");
    assert_eq!(variables[1].name, "z");
}

#[test]
fn parenthesized_initializer_missing_close_recovers_at_next_variable() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                p: Position := (x := 1
                z: DINT;
            END_VAR
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `)`
      ┌─ <internal>:5:17
      │
    5 │                 z: DINT;
      │                 ^ Missing expected Token `)`

    error[E006]: Missing expected Token `;`
      ┌─ <internal>:5:17
      │
    5 │                 z: DINT;
      │                 ^ Missing expected Token `;`
    ");

    let variables = &unit.pous[0].variable_blocks[0].variables;
    assert_eq!(variables.len(), 2);
    assert_eq!(variables[0].name, "p");
    assert_eq!(variables[1].name, "z");
}

#[test]
fn call_initializer_missing_close_recovers_at_next_variable() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                p: DINT := foo(1
                z: DINT;
            END_VAR
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `)`
      ┌─ <internal>:5:17
      │
    5 │                 z: DINT;
      │                 ^ Missing expected Token `)`

    error[E006]: Missing expected Token `;`
      ┌─ <internal>:5:17
      │
    5 │                 z: DINT;
      │                 ^ Missing expected Token `;`
    ");

    let variables = &unit.pous[0].variable_blocks[0].variables;
    assert_eq!(variables.len(), 2);
    assert_eq!(variables[0].name, "p");
    assert_eq!(variables[1].name, "z");
}

#[test]
fn index_initializer_missing_close_recovers_at_next_variable() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                p: DINT := arr[1
                z: DINT;
            END_VAR
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `]`
      ┌─ <internal>:5:17
      │
    5 │                 z: DINT;
      │                 ^ Missing expected Token `]`

    error[E006]: Missing expected Token `;`
      ┌─ <internal>:5:17
      │
    5 │                 z: DINT;
      │                 ^ Missing expected Token `;`
    ");

    let variables = &unit.pous[0].variable_blocks[0].variables;
    assert_eq!(variables.len(), 2);
    assert_eq!(variables[0].name, "p");
    assert_eq!(variables[1].name, "z");
}

#[test]
fn array_literal_missing_comma_recovers_at_next_element() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                xs: Arr := [1 2];
                y: DINT;
            END_VAR
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `,`
      ┌─ <internal>:4:31
      │
    4 │                 xs: Arr := [1 2];
      │                               ^ Missing expected Token `,`
    ");

    let variables = &unit.pous[0].variable_blocks[0].variables;
    assert_eq!(variables.len(), 2);
    assert_eq!(variables[0].name, "xs");
    assert_eq!(variables[1].name, "y");
}

#[test]
fn if_missing_then_recovers_at_statement_body() {
    let src = r"
        FUNCTION main
            IF x
                y := 1;
            END_IF
            z := 2;
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `THEN`
      ┌─ <internal>:4:17
      │
    4 │                 y := 1;
      │                 ^ Missing expected Token `THEN`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.implementations.len(), 1);
}

#[test]
fn for_missing_to_recovers_at_end_expression() {
    let src = r"
        FUNCTION main
            FOR i := 0 10 DO
                y := i;
            END_FOR
            z := 2;
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `TO`
      ┌─ <internal>:3:24
      │
    3 │             FOR i := 0 10 DO
      │                        ^^ Missing expected Token `TO`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.implementations.len(), 1);
}

#[test]
fn case_missing_of_recovers_at_first_case_label() {
    let src = r"
        FUNCTION main
            CASE x
                1: y := 1;
            END_CASE
            z := 2;
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `OF`
      ┌─ <internal>:4:17
      │
    4 │                 1: y := 1;
      │                 ^ Missing expected Token `OF`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.implementations.len(), 1);
}

#[test]
fn case_missing_label_colon_recovers_at_statement_body() {
    let src = r"
        FUNCTION main
            CASE x OF
                1 y := 1;
                2: y := 2;
            END_CASE
            z := 3;
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `:`
      ┌─ <internal>:4:19
      │
    4 │                 1 y := 1;
      │                   ^ Missing expected Token `:`
    ");

    let AstStatement::ControlStatement(AstControlStatement::Case(case)) =
        unit.implementations[0].statements[0].get_stmt()
    else {
        panic!("expected case statement");
    };
    assert_eq!(case.case_blocks.len(), 2);
    assert_eq!(case.case_blocks[0].body.len(), 1);
    assert_eq!(unit.implementations[0].statements.len(), 2);
}

#[test]
fn case_missing_label_colon_recovers_at_next_case_label() {
    let src = r"
        FUNCTION main
            CASE x OF
                1
                2: y := 2;
            END_CASE
            z := 3;
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `:`
      ┌─ <internal>:5:17
      │
    5 │                 2: y := 2;
      │                 ^ Missing expected Token `:`
    ");

    let AstStatement::ControlStatement(AstControlStatement::Case(case)) =
        unit.implementations[0].statements[0].get_stmt()
    else {
        panic!("expected case statement");
    };
    assert_eq!(case.case_blocks.len(), 2);
    assert!(case.case_blocks[0].body.is_empty());
    assert_eq!(unit.implementations[0].statements.len(), 2);
}

#[test]
fn repeat_missing_end_repeat_recovers_at_next_statement() {
    let src = r"
        FUNCTION main
            REPEAT
                x := 1;
            UNTIL x > 5
            z := 2;
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_REPEAT`
      ┌─ <internal>:6:13
      │
    6 │             z := 2;
      │             ^ Missing expected Token `END_REPEAT`
    ");
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.implementations.len(), 1);
}

#[test]
fn pou_missing_end_recovers_at_next_top_level_declaration() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                x: DINT;
            END_VAR

        FUNCTION main
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_FUNCTION_BLOCK`
      ┌─ <internal>:7:9
      │
    7 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `END_FUNCTION_BLOCK`
    ");
    assert_eq!(unit.pous.len(), 2);
    assert_eq!(unit.pous[0].name, "FbA");
    assert_eq!(unit.pous[1].name, "main");
}

#[test]
fn var_block_missing_end_recovers_at_next_top_level_declaration() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                x: DINT;

        FUNCTION main
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_VAR`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `END_VAR`

    error[E006]: Missing expected Token `END_FUNCTION_BLOCK`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `END_FUNCTION_BLOCK`
    ");
    assert_eq!(unit.pous.len(), 2);
    assert_eq!(unit.pous[0].name, "FbA");
    assert_eq!(unit.pous[1].name, "main");
}

#[test]
fn var_block_missing_end_recovers_at_next_member_declaration() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                x: DINT;

            METHOD second
            END_METHOD
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_VAR`
      ┌─ <internal>:6:13
      │
    6 │             METHOD second
      │             ^^^^^^ Missing expected Token `END_VAR`
    ");
    assert_eq!(unit.pous.len(), 2);
    assert_eq!(unit.pous[0].name, "FbA");
    assert_eq!(unit.pous[1].name, "FbA.second");
}

#[test]
fn var_block_missing_variable_semicolon_recovers_at_next_variable() {
    let src = r"
        FUNCTION_BLOCK FbA
            VAR
                x: DINT
                y: DINT;
            END_VAR
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `;`
      ┌─ <internal>:5:17
      │
    5 │                 y: DINT;
      │                 ^ Missing expected Token `;`
    ");
    let variables = &unit.pous[0].variable_blocks[0].variables;
    assert_eq!(variables.len(), 2);
    assert_eq!(variables[0].name, "x");
    assert_eq!(variables[1].name, "y");
}

#[test]
fn var_config_missing_end_recovers_at_next_top_level_declaration() {
    let src = r"
        VAR_CONFIG
            main.x AT %QX0.0 : BOOL;

        FUNCTION main
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_VAR`
      ┌─ <internal>:5:9
      │
    5 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `END_VAR`
    ");
    assert_eq!(unit.var_config.len(), 1);
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "main");
}

#[test]
fn var_config_missing_semicolon_recovers_at_next_top_level_declaration() {
    let src = r"
        VAR_CONFIG
            main.x AT %QX0.0 : BOOL

        FUNCTION main
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `;`
      ┌─ <internal>:5:9
      │
    5 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `;`

    error[E006]: Missing expected Token `END_VAR`
      ┌─ <internal>:5:9
      │
    5 │         FUNCTION main
      │         ^^^^^^^^ Missing expected Token `END_VAR`
    ");
    assert_eq!(unit.var_config.len(), 1);
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "main");
}

#[test]
fn var_config_missing_semicolon_recovers_at_next_config_entry() {
    let src = r"
        VAR_CONFIG
            main.x AT %QW0 : DINT
            main.y AT %QW1 : INT;
        END_VAR

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `;`
      ┌─ <internal>:4:13
      │
    4 │             main.y AT %QW1 : INT;
      │             ^^^^ Missing expected Token `;`
    ");
    assert_eq!(unit.var_config.len(), 2);
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "other");
}

#[test]
fn var_config_missing_hardware_access_skips_type_tail() {
    let src = r"
        VAR_CONFIG
            main.x AT : BOOL;
            main.y AT %QW1 : INT;
        END_VAR

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @r"
    error[E006]: Missing expected Token hardware access
      ┌─ <internal>:3:23
      │
    3 │             main.x AT : BOOL;
      │                       ^ Missing expected Token hardware access
    ");
    assert_eq!(unit.var_config.len(), 1);
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "other");
}

#[test]
fn if_missing_end_recovers_at_next_top_level_declaration() {
    let src = r"
        FUNCTION main
            IF TRUE THEN
                x := 1;

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_IF`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `END_IF`

    error[E006]: Missing expected Token `END_FUNCTION`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `END_FUNCTION`
    ");
    assert_eq!(unit.pous.len(), 2);
    assert_eq!(unit.pous[0].name, "main");
    assert_eq!(unit.pous[1].name, "other");
}

#[test]
fn for_missing_end_recovers_at_next_top_level_declaration() {
    let src = r"
        FUNCTION main
            FOR i := 0 TO 10 DO
                x := i;

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_FOR`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `END_FOR`

    error[E006]: Missing expected Token `END_FUNCTION`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `END_FUNCTION`
    ");
    assert_eq!(unit.pous.len(), 2);
    assert_eq!(unit.pous[0].name, "main");
    assert_eq!(unit.pous[1].name, "other");
}

#[test]
fn while_missing_end_recovers_at_next_top_level_declaration() {
    let src = r"
        FUNCTION main
            WHILE TRUE DO
                x := 1;

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_WHILE`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `END_WHILE`

    error[E006]: Missing expected Token `END_FUNCTION`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `END_FUNCTION`
    ");
    assert_eq!(unit.pous.len(), 2);
    assert_eq!(unit.pous[0].name, "main");
    assert_eq!(unit.pous[1].name, "other");
}

#[test]
fn repeat_missing_until_recovers_at_next_top_level_declaration() {
    let src = r"
        FUNCTION main
            REPEAT
                x := 1;

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `UNTIL`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `UNTIL`

    error[E006]: Missing expected Token `END_FUNCTION`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `END_FUNCTION`
    ");
    assert_eq!(unit.pous.len(), 2);
    assert_eq!(unit.pous[0].name, "main");
    assert_eq!(unit.pous[1].name, "other");
}

#[test]
fn case_missing_end_recovers_at_next_top_level_declaration() {
    let src = r"
        FUNCTION main
            CASE x OF
                1: y := 1;

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_CASE`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `END_CASE`

    error[E006]: Missing expected Token `END_FUNCTION`
      ┌─ <internal>:6:9
      │
    6 │         FUNCTION other
      │         ^^^^^^^^ Missing expected Token `END_FUNCTION`
    ");
    assert_eq!(unit.pous.len(), 2);
    assert_eq!(unit.pous[0].name, "main");
    assert_eq!(unit.pous[1].name, "other");
}

#[test]
fn method_missing_end_recovers_at_next_member_declaration() {
    let src = r"
        FUNCTION_BLOCK FbA
            METHOD first

            METHOD second
            END_METHOD
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_METHOD`
      ┌─ <internal>:5:13
      │
    5 │             METHOD second
      │             ^^^^^^ Missing expected Token `END_METHOD`
    ");
    assert_eq!(unit.pous.len(), 3);
    assert_eq!(unit.pous[0].name, "FbA");
    assert_eq!(unit.pous[1].name, "FbA.first");
    assert_eq!(unit.pous[2].name, "FbA.second");
}

#[test]
fn property_missing_end_recovers_at_next_member_declaration() {
    let src = r"
        FUNCTION_BLOCK FbA
            PROPERTY_GET first: DINT
                first := 1;

            METHOD second
            END_METHOD
        END_FUNCTION_BLOCK
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_PROPERTY`
      ┌─ <internal>:6:13
      │
    6 │             METHOD second
      │             ^^^^^^ Missing expected Token `END_PROPERTY`
    ");
    assert_eq!(unit.pous.len(), 2);
    assert_eq!(unit.pous[0].name, "FbA");
    assert_eq!(unit.pous[1].name, "FbA.second");
    assert_eq!(unit.pous[0].properties.len(), 1);
    assert_eq!(unit.pous[0].properties[0].ident.name, "first");
}

#[test]
fn action_missing_end_recovers_at_next_action_declaration() {
    let src = r"
        PROGRAM Main
        END_PROGRAM

        ACTIONS Main
            ACTION first
                x := 1;

            ACTION second
                x := 2;
            END_ACTION
        END_ACTIONS
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_ACTION`
      ┌─ <internal>:9:13
      │
    9 │             ACTION second
      │             ^^^^^^ Missing expected Token `END_ACTION`
    ");
    assert_eq!(unit.implementations.len(), 3);
    assert_eq!(unit.implementations[0].name, "Main");
    assert_eq!(unit.implementations[1].name, "Main.first");
    assert_eq!(unit.implementations[2].name, "Main.second");
}

#[test]
fn actions_block_missing_end_recovers_at_next_top_level_declaration() {
    let src = r"
        PROGRAM Main
        END_PROGRAM

        ACTIONS Main
            ACTION first
                x := 1;
            END_ACTION

        FUNCTION other
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_ACTIONS`
       ┌─ <internal>:10:9
       │
    10 │         FUNCTION other
       │         ^^^^^^^^ Missing expected Token `END_ACTIONS`
    ");
    assert_eq!(unit.pous.len(), 2);
    assert_eq!(unit.pous[0].name, "Main");
    assert_eq!(unit.pous[1].name, "other");
    assert_eq!(unit.implementations.len(), 3);
    assert_eq!(unit.implementations[0].name, "Main");
    assert_eq!(unit.implementations[1].name, "Main.first");
    assert_eq!(unit.implementations[2].name, "other");
}

#[test]
fn interface_method_missing_end_recovers_at_interface_end() {
    let src = r"
        INTERFACE IFoo
            METHOD first

        END_INTERFACE

        FUNCTION main
        END_FUNCTION
    ";

    let (unit, diagnostics) = parse_buffered(src);

    assert_snapshot!(diagnostics, @"
    error[E006]: Missing expected Token `END_METHOD`
      ┌─ <internal>:5:9
      │
    5 │         END_INTERFACE
      │         ^^^^^^^^^^^^^ Missing expected Token `END_METHOD`
    ");
    assert_eq!(unit.interfaces.len(), 1);
    assert_eq!(unit.interfaces[0].ident.name, "IFoo");
    assert_eq!(unit.interfaces[0].methods.len(), 1);
    assert_eq!(unit.pous.len(), 1);
    assert_eq!(unit.pous[0].name, "main");
}
