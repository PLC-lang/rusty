use super::{Index, VariableType};
/// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use pretty_assertions::assert_eq;

use crate::ast::*;
use crate::lexer;
use crate::parser;

macro_rules! index {
    ($code:tt) => {{
        let lexer = lexer::lex($code);
        let mut ast = parser::parse(lexer).unwrap();

        let mut index = Index::new();
        index.pre_process(&mut ast);
        index.visit(&mut ast);
        index
    }};
}

#[test]
fn global_variables_are_indexed() {
    let index = index!(
        r#"
        VAR_GLOBAL
            a: INT;
            b: BOOL;
        END_VAR
    "#
    );

    let entry_a = index.find_global_variable("a").unwrap();
    assert_eq!("a", entry_a.name);
    assert_eq!("INT", entry_a.information.data_type_name);

    let entry_b = index.find_global_variable("b").unwrap();
    assert_eq!("b", entry_b.name);
    assert_eq!("BOOL", entry_b.information.data_type_name);
}

#[test]
fn program_is_indexed() {
    let index = index!(
        r#"
        PROGRAM myProgram
        END_PROGRAM
    "#
    );

    index.find_type("myProgram").unwrap();
    let program_variable = index.find_global_variable("myProgram").unwrap();

    //TODO: type name should refer to my
    assert_eq!("myProgram", program_variable.information.data_type_name);
}

#[test]
fn function_is_indexed() {
    let index = index!(
        r#"
        FUNCTION myFunction : INT
        END_FUNCTION
    "#
    );

    index.find_type("myFunction").unwrap();

    let return_variable = index.find_member("myFunction", "myFunction").unwrap();
    assert_eq!("myFunction", return_variable.name);
    assert_eq!(
        Some("myFunction".to_string()),
        return_variable.information.qualifier
    );
    assert_eq!("INT", return_variable.information.data_type_name);
    assert_eq!(
        VariableType::Return,
        return_variable.information.variable_type
    );
}

#[test]
fn pous_are_indexed() {
    let index = index!(
        r#"
        PROGRAM myProgram
        END_PROGRAM
        FUNCTION myFunction : INT
        END_FUNCTION
    "#
    );

    index.find_type("myFunction").unwrap();
    index.find_type("myProgram").unwrap();
}

#[test]
fn program_members_are_indexed() {
    let index = index!(
        r#"
        PROGRAM myProgram
        VAR
            a : INT;
            b : INT;
        END_VAR
        VAR_INPUT
            c : BOOL;
            d : BOOL;
        END_VAR
        END_PROGRAM
    "#
    );

    let variable = index.find_member("myProgram", "a").unwrap();
    assert_eq!("a", variable.name);
    assert_eq!("INT", variable.information.data_type_name);
    assert_eq!(VariableType::Local, variable.information.variable_type);

    let variable = index.find_member("myProgram", "b").unwrap();
    assert_eq!("b", variable.name);
    assert_eq!("INT", variable.information.data_type_name);
    assert_eq!(VariableType::Local, variable.information.variable_type);

    let variable = index.find_member("myProgram", "c").unwrap();
    assert_eq!("c", variable.name);
    assert_eq!("BOOL", variable.information.data_type_name);
    assert_eq!(VariableType::Input, variable.information.variable_type);

    let variable = index.find_member("myProgram", "d").unwrap();
    assert_eq!("d", variable.name);
    assert_eq!("BOOL", variable.information.data_type_name);
    assert_eq!(VariableType::Input, variable.information.variable_type);
}

#[test]
fn given_set_of_local_global_and_functions_the_index_can_be_retrieved() {
    let index = index!(
        r#"
        VAR_GLOBAL
            a : INT;
            b : BOOL;
        END_VAR
        PROGRAM prg
        VAR
            a : INT;
            c : BOOL;
            d : INT;
        END_VAR
        END_PROGRAM
        VAR_GLOBAL
            d : BOOL;
            x : INT;
            foo : INT;
        END_VAR
        FUNCTION foo : INT
        VAR
            a : INT;
            b : INT;
        END_VAR
        END_FUNCTION
        "#
    );

    //Asking for a variable with no context returns global variables
    let result = index.find_variable(None, &["a".to_string()]).unwrap();
    assert_eq!(VariableType::Global, result.information.variable_type);
    assert_eq!("a", result.name);
    assert_eq!(None, result.information.qualifier);
    //Asking for a variable with the POU  context finds a local variable
    let result = index
        .find_variable(Some("prg"), &["a".to_string()])
        .unwrap();
    assert_eq!(VariableType::Local, result.information.variable_type);
    assert_eq!("a", result.name);
    assert_eq!(Some("prg".to_string()), result.information.qualifier);
    //Asking for a variable with th POU context finds a global variable
    let result = index
        .find_variable(Some("prg"), &["b".to_string()])
        .unwrap();
    assert_eq!(VariableType::Global, result.information.variable_type);
    assert_eq!("b", result.name);
    assert_eq!(None, result.information.qualifier);
    //Asking for a variable with the function context finds the local variable
    let result = index
        .find_variable(Some("foo"), &["a".to_string()])
        .unwrap();
    assert_eq!(VariableType::Local, result.information.variable_type);
    assert_eq!("a", result.name);
    assert_eq!(Some("foo".to_string()), result.information.qualifier);
    //Asking for a variable with the function context finds the global variable
    let result = index
        .find_variable(Some("foo"), &["x".to_string()])
        .unwrap();
    assert_eq!(VariableType::Global, result.information.variable_type);
    assert_eq!("x", result.name);
    assert_eq!(None, result.information.qualifier);
}

#[test]
fn index_can_be_retrieved_from_qualified_name() {
    let index = index!(
        r#"
    FUNCTION_BLOCK fb1
    VAR_INPUT
        fb2_inst : fb2;
    END_VAR
    END_FUNCTION_BLOCK
    
    FUNCTION_BLOCK fb2
    VAR_INPUT
        fb3_inst : fb3;
    END_VAR
    END_FUNCTION_BLOCK

    FUNCTION_BLOCK fb3
    VAR_INPUT
        x : INT;
    END_VAR
    END_FUNCTION_BLOCK

    VAR_GLOBAL
        fb1_inst : fb1;
    END_VAR

    PROGRAM prg
        fb1_inst.fb2_inst.fb3_inst.x := 1;
    END_PROGRAM
    "#
    );

    let result = index
        .find_variable(
            Some("prg"),
            &[
                "fb1_inst".to_string(),
                "fb2_inst".to_string(),
                "fb3_inst".to_string(),
                "x".to_string(),
            ],
        )
        .unwrap();
    assert_eq!(VariableType::Input, result.information.variable_type);
    assert_eq!("x", result.name);
    assert_eq!(Some("fb3".to_string()), result.information.qualifier);
}

#[test]
fn pre_processing_generates_inline_enums_global() {
    // GIVEN a global inline enum
    let lexer = lexer::lex(
        r#"
        VAR_GLOBAL
            inline_enum : (a,b,c);
        END_VAR
        "#,
    );
    let mut ast = parser::parse(lexer).unwrap();

    // WHEN the AST ist pre-processed
    let mut index = Index::new();
    index.pre_process(&mut ast);

    //ENUM
    // THEN an implicit datatype should have been generated for the enum
    let new_enum_type = &ast.types[0];
    assert_eq!(
        &DataType::EnumType {
            name: Some("__global_inline_enum".to_string()),
            elements: ["a".to_string(), "b".to_string(), "c".to_string()].to_vec()
        },
        new_enum_type
    );

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.global_vars[0].variables[0].data_type;
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__global_inline_enum".to_string(),
        },
        var_data_type
    );

    assert_eq!(
        &"__global_inline_enum".to_string(),
        &ast.global_vars[0].variables[0]
            .data_type
            .get_name()
            .unwrap()
            .to_string()
    )
}

#[test]
fn pre_processing_generates_inline_structs_global() {
    // GIVEN a global inline enum
    let lexer = lexer::lex(
        r#"
        VAR_GLOBAL
            inline_struct: STRUCT a: INT; END_STRUCT
        END_VAR
        "#,
    );
    let mut ast = parser::parse(lexer).unwrap();

    // WHEN the AST ist pre-processed
    let mut index = Index::new();
    index.pre_process(&mut ast);

    //STRUCT
    //THEN an implicit datatype should have been generated for the struct
    let new_struct_type = &ast.types[0];
    assert_eq!(
        &DataType::StructType {
            name: Some("__global_inline_struct".to_string()),
            variables: vec![Variable {
                name: "a".to_string(),
                data_type: DataTypeDeclaration::DataTypeReference {
                    referenced_type: "INT".to_string()
                },
                location: 0..0,
            }]
        },
        new_struct_type
    );

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.global_vars[0].variables[0].data_type;
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__global_inline_struct".to_string(),
        },
        var_data_type
    );
}

#[test]
fn pre_processing_generates_inline_enums() {
    // GIVEN a global inline enum
    let lexer = lexer::lex(
        r#"
        PROGRAM foo
        VAR
            inline_enum : (a,b,c);
        END_VAR
        END_PROGRAM
        "#,
    );
    let mut ast = parser::parse(lexer).unwrap();

    // WHEN the AST ist pre-processed
    let mut index = Index::new();
    index.pre_process(&mut ast);

    //ENUM
    // THEN an implicit datatype should have been generated for the enum
    let new_enum_type = &ast.types[0];
    assert_eq!(
        &DataType::EnumType {
            name: Some("__foo_inline_enum".to_string()),
            elements: ["a".to_string(), "b".to_string(), "c".to_string()].to_vec()
        },
        new_enum_type
    );

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.units[0].variable_blocks[0].variables[0].data_type;
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__foo_inline_enum".to_string(),
        },
        var_data_type
    );
}

#[test]
fn pre_processing_generates_inline_structs() {
    // GIVEN a global inline enum
    let lexer = lexer::lex(
        r#"
        PROGRAM foo
        VAR
            inline_struct: STRUCT a: INT; END_STRUCT
        END_VAR
        END_PROGRAM
        "#,
    );
    let mut ast = parser::parse(lexer).unwrap();

    // WHEN the AST ist pre-processed
    let mut index = Index::new();
    index.pre_process(&mut ast);

    //STRUCT
    //THEN an implicit datatype should have been generated for the struct
    let new_struct_type = &ast.types[0];
    assert_eq!(
        &DataType::StructType {
            name: Some("__foo_inline_struct".to_string()),
            variables: vec![Variable {
                name: "a".to_string(),
                data_type: DataTypeDeclaration::DataTypeReference {
                    referenced_type: "INT".to_string()
                },
                location: 0..0
            }]
        },
        new_struct_type
    );

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.units[0].variable_blocks[0].variables[0].data_type;
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__foo_inline_struct".to_string(),
        },
        var_data_type
    );
}


#[test]
fn pre_processing_generates_inline_arrays() {
    // GIVEN an inline array is declared
    let lexer = lexer::lex(
        r#"
        PROGRAM foo
        VAR
            inline_array: ARRAY[0..1] OF INT;
        END_VAR
        END_PROGRAM
        "#,
    );
    let mut ast = parser::parse(lexer).unwrap();

    // WHEN the AST ist pre-processed
    let mut index = Index::new();
    index.pre_process(&mut ast);

    //ARRAY
    //THEN an implicit datatype should have been generated for the array
    let new_array_type = &ast.types[0];
    
    let expected = 
        &DataType::ArrayType {
            name: Some("__foo_inline_array".to_string()),
            bounds: Statement::RangeStatement{
                start: Box::new(Statement::LiteralInteger { value: "0".to_string(), location:0..0}),
                end: Box::new(Statement::LiteralInteger {value: "1".to_string(), location: 0..0}),
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference{
                referenced_type: "INT".to_string(),
            }),
        };
    assert_eq!(
        format!("{:?}", expected),
        format!("{:?}", new_array_type)
    );

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.units[0].variable_blocks[0].variables[0].data_type;
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__foo_inline_array".to_string(),
        },
        var_data_type
    );
}


#[test]
fn pre_processing_generates_inline_array_of_array() {
    // GIVEN an inline array is declared
    let lexer = lexer::lex(
        r#"
        PROGRAM foo
        VAR
            inline_array: ARRAY[0..1] OF ARRAY[0..1] OF INT;
        END_VAR
        END_PROGRAM
        "#,
    );
    let mut ast = parser::parse(lexer).unwrap();

    // WHEN the AST ist pre-processed
    let mut index = Index::new();
    index.pre_process(&mut ast);

    //ARRAY
    //THEN an implicit datatype should have been generated for the array

    // ARRAY OF INT
    let new_array_type = &ast.types[0];
    let expected = 
        &DataType::ArrayType {
            name: Some("__foo_inline_array_".to_string()),
            bounds: Statement::RangeStatement{
                start: Box::new(Statement::LiteralInteger { value: "0".to_string(), location:0..0}),
                end: Box::new(Statement::LiteralInteger {value: "1".to_string(), location: 0..0}),
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference{
                referenced_type: "INT".to_string(),
            }),
        };
    assert_eq!(
        format!("{:?}", expected),
        format!("{:?}", new_array_type)
    );

    // ARRAY OF ARRAY
    let new_array_type = &ast.types[1];
    let expected = 
        &DataType::ArrayType {
            name: Some("__foo_inline_array".to_string()),
            bounds: Statement::RangeStatement{
                start: Box::new(Statement::LiteralInteger { value: "0".to_string(), location:0..0}),
                end: Box::new(Statement::LiteralInteger {value: "1".to_string(), location: 0..0}),
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference{
                referenced_type: "__foo_inline_array_".to_string(),
            }),
        };
    assert_eq!(
        format!("{:?}", expected),
        format!("{:?}", new_array_type)
    );

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.units[0].variable_blocks[0].variables[0].data_type;
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__foo_inline_array".to_string(),
        },
        var_data_type
    );
}

#[test]
fn pre_processing_nested_array_in_struct() {
    let lexer = lexer::lex(
        r#"
        TYPE MyStruct:
        STRUCT 
          field1 : ARRAY[0..4] OF INT;
        END_STRUCT
        END_TYPE
        
        PROGRAM Main
        VAR
          m : MyStruct;
        END_VAR
          m.field1[3] := 7;
        END_PROGRAM
        "#
    );

    let mut ast = parser::parse(lexer).unwrap();

     // WHEN the AST ist pre-processed
    let mut index = Index::new();
    index.pre_process(&mut ast);

    //THEN an implicit datatype should have been generated for the array

    // Struct Type 
    let new_array_type = &ast.types[0];
    let expected = 
        &DataType::StructType {
            name: Some("MyStruct".to_string()),
            variables: vec![
                Variable {
                    name: "field1".to_string(),
                    data_type: DataTypeDeclaration::DataTypeReference { referenced_type: "__MyStruct_field1".to_string() },
                    location : 0..0,
                }
            ],
        };
    assert_eq!(
        format!("{:?}", expected),
        format!("{:?}", new_array_type)
    );

// ARRAY OF INT
    let new_array_type = &ast.types[1];
    let expected = 
        &DataType::ArrayType {
            name: Some("__MyStruct_field1".to_string()),
            bounds: Statement::RangeStatement{
                start: Box::new(Statement::LiteralInteger { value: "0".to_string(), location:0..0}),
                end: Box::new(Statement::LiteralInteger {value: "4".to_string(), location: 0..0}),
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference{
                referenced_type: "INT".to_string(),
            }),
        };
    assert_eq!(
        format!("{:?}", expected),
        format!("{:?}", new_array_type)
    );


}

#[test]
fn pre_processing_generates_inline_array_of_array_of_array() {
    // GIVEN an inline array is declared
    let lexer = lexer::lex(
        r#"
        PROGRAM foo
        VAR
            inline_array: ARRAY[0..1] OF ARRAY[0..1] OF ARRAY[0..1] OF INT;
        END_VAR
        END_PROGRAM
        "#,
    );
    let mut ast = parser::parse(lexer).unwrap();

    // WHEN the AST ist pre-processed
    let mut index = Index::new();
    index.pre_process(&mut ast);

    //ARRAY
    //THEN an implicit datatype should have been generated for the array

    // ARRAY OF INT
    let new_array_type = &ast.types[0];
    let expected = 
        &DataType::ArrayType {
            name: Some("__foo_inline_array__".to_string()),
            bounds: Statement::RangeStatement{
                start: Box::new(Statement::LiteralInteger { value: "0".to_string(), location:0..0}),
                end: Box::new(Statement::LiteralInteger {value: "1".to_string(), location: 0..0}),
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference{
                referenced_type: "INT".to_string(),
            }),
        };
    assert_eq!(
        format!("{:?}", expected),
        format!("{:?}", new_array_type)
    );

    // ARRAY OF ARRAY
    let new_array_type = &ast.types[1];
    let expected = 
        &DataType::ArrayType {
            name: Some("__foo_inline_array_".to_string()),
            bounds: Statement::RangeStatement{
                start: Box::new(Statement::LiteralInteger { value: "0".to_string(), location:0..0}),
                end: Box::new(Statement::LiteralInteger {value: "1".to_string(), location: 0..0}),
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference{
                referenced_type: "__foo_inline_array__".to_string(),
            }),
        };
    assert_eq!(
        format!("{:?}", expected),
        format!("{:?}", new_array_type)
    );

    // ARRAY OF ARRAY
    let new_array_type = &ast.types[2];
    let expected = 
        &DataType::ArrayType {
            name: Some("__foo_inline_array".to_string()),
            bounds: Statement::RangeStatement{
                start: Box::new(Statement::LiteralInteger { value: "0".to_string(), location:0..0}),
                end: Box::new(Statement::LiteralInteger {value: "1".to_string(), location: 0..0}),
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference{
                referenced_type: "__foo_inline_array_".to_string(),
            }),
        };
    assert_eq!(
        format!("{:?}", expected),
        format!("{:?}", new_array_type)
    );




    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.units[0].variable_blocks[0].variables[0].data_type;
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__foo_inline_array".to_string(),
        },
        var_data_type
    );
}
