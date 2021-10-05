// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use pretty_assertions::assert_eq;

use crate::lexer;
use crate::lexer::IdProvider;
use crate::parser;
use crate::parser::tests::literal_int;
use crate::typesystem::TypeSize;
use crate::{ast::*, index::VariableType, typesystem::DataTypeInformation};

macro_rules! index {
    ($code:tt) => {{
        let ids = crate::lexer::IdProvider::new();
        let lexer = crate::lexer::lex_with_ids($code, ids.clone());
        let (mut ast, ..) = crate::parser::parse(lexer);

        crate::ast::pre_process(&mut ast);
        crate::index::visitor::visit(&ast, ids.clone())
    }};
}

fn lex(source: &str) -> lexer::ParseSession {
    lexer::lex(source)
}

#[test]
fn index_not_case_sensitive() {
    let index = index!(
        r#"
        TYPE st : STRUCT
            x : INT;
            y : DINT;
        END_STRUCT
        END_TYPE

        VAR_GLOBAL
            a: INT;
            x : ST; 
        END_VAR
        FUNCTION foo : INT
        END_FUNCTION

        PROGRAM aProgram
            VAR
                c,d : INT;
            END_VAR
        END_PROGRAM
    "#
    );

    let entry = index.find_global_variable("A").unwrap();
    assert_eq!("a", entry.name);
    assert_eq!("INT", entry.information.data_type_name);
    let entry = index.find_global_variable("X").unwrap();
    assert_eq!("x", entry.name);
    assert_eq!("ST", entry.information.data_type_name);
    let entry = index.find_member("ST", "X").unwrap();
    assert_eq!("x", entry.name);
    assert_eq!("INT", entry.information.data_type_name);
    let entry = index.find_type("APROGRAM").unwrap();
    assert_eq!("aProgram", entry.name);
    let entry = index.find_implementation("Foo").unwrap();
    assert_eq!("foo", entry.call_name);
    assert_eq!("foo", entry.type_name);
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

    assert_eq!("myProgram", program_variable.information.data_type_name);
}

#[test]
fn actions_are_indexed() {
    let index = index!(
        r#"
        PROGRAM myProgram
        END_PROGRAM
        ACTIONS myProgram
            ACTION foo
            END_ACTION
        END_ACTIONS
        ACTION myProgram.bar
        END_ACTION
    "#
    );

    let foo_impl = index.find_implementation("myProgram.foo").unwrap();
    assert_eq!("myProgram.foo", foo_impl.call_name);
    assert_eq!("myProgram", foo_impl.type_name);
    let info = index
        .get_type("myProgram.foo")
        .unwrap()
        .get_type_information();
    if let crate::typesystem::DataTypeInformation::Alias {
        name,
        referenced_type,
    } = info
    {
        assert_eq!("myProgram.foo", name);
        assert_eq!("myProgram", referenced_type);
    } else {
        panic!("Wrong variant : {:#?}", info);
    }
    if let crate::typesystem::DataTypeInformation::Struct { name, .. } =
        index.find_effective_type_information(info).unwrap()
    {
        assert_eq!("myProgram_interface", name);
    } else {
        panic!("Wrong variant : {:#?}", info);
    }

    let bar = index.find_implementation("myProgram.bar").unwrap();
    assert_eq!("myProgram.bar", bar.call_name);
    assert_eq!("myProgram", bar.type_name);

    let info = index
        .get_type("myProgram.bar")
        .unwrap()
        .get_type_information();
    if let crate::typesystem::DataTypeInformation::Alias {
        name,
        referenced_type,
    } = info
    {
        assert_eq!("myProgram.bar", name);
        assert_eq!("myProgram", referenced_type);
    } else {
        panic!("Wrong variant : {:#?}", info);
    }
    if let crate::typesystem::DataTypeInformation::Struct { name, .. } =
        index.find_effective_type_information(info).unwrap()
    {
        assert_eq!("myProgram_interface", name);
    } else {
        panic!("Wrong variant : {:#?}", info);
    }
}

#[test]
fn fb_methods_are_indexed() {
    let index = index!(
        r#"
        FUNCTION_BLOCK myFuncBlock
            METHOD foo
                VAR x : SINT; END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
    "#
    );

    let foo_impl = index.find_implementation("myFuncBlock.foo").unwrap();
    assert_eq!("myFuncBlock.foo", foo_impl.call_name);
    assert_eq!("myFuncBlock.foo", foo_impl.type_name);
    let info = index
        .get_type("myFuncBlock.foo")
        .unwrap()
        .get_type_information();
    if let crate::typesystem::DataTypeInformation::Struct {
        name, member_names, ..
    } = info
    {
        assert_eq!("myFuncBlock.foo_interface", name);
        assert_eq!(&vec!["x"], member_names);
    } else {
        panic!("Wrong variant : {:#?}", info);
    }
}

#[test]
fn class_methods_are_indexed() {
    let index = index!(
        r#"
        CLASS myClass
            METHOD foo
                VAR y : DINT; END_VAR
            END_METHOD
        END_CLASS
    "#
    );

    let foo_impl = index.find_implementation("myClass.foo").unwrap();
    assert_eq!("myClass.foo", foo_impl.call_name);
    assert_eq!("myClass.foo", foo_impl.type_name);
    let info = index
        .get_type("myClass.foo")
        .unwrap()
        .get_type_information();
    if let crate::typesystem::DataTypeInformation::Struct {
        name, member_names, ..
    } = info
    {
        assert_eq!("myClass.foo_interface", name);
        assert_eq!(&vec!["y"], member_names);
    } else {
        panic!("Wrong variant : {:#?}", info);
    }
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
fn function_with_varargs_param_marked() {
    let index = index!(
        r#"
        FUNCTION myFunc : INT
        VAR_INPUT
            x : INT;
            y : ...;
        END_VAR
        END_FUNCTION
        "#
    );
    let function = index.find_type("myFunc").unwrap();
    assert!(function.get_type_information().is_variadic());
    assert_eq!(None, function.get_type_information().get_variadic_type());
}

#[test]
fn function_with_typed_varargs_param_marked() {
    let index = index!(
        r#"
        FUNCTION myFunc : INT
        VAR_INPUT
            x : INT;
            y : INT...;
        END_VAR
        END_FUNCTION
        "#
    );
    let function = index.find_type("myFunc").unwrap();
    assert!(function.get_type_information().is_variadic());
    assert_eq!(
        Some("INT"),
        function.get_type_information().get_variadic_type()
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
        FUNCTION_BLOCK myFunctionBlock : INT
        END_FUNCTION_BLOCK
        CLASS myClass
        END_CLASS
    "#
    );

    index.find_type("myFunction").unwrap();
    index.find_type("myProgram").unwrap();
    index.find_type("myFunctionBlock").unwrap();
    index.find_type("myClass").unwrap();
}

#[test]
fn implementations_are_indexed() {
    let index = index!(
        r#"
        PROGRAM myProgram
        END_PROGRAM
        PROGRAM prog2
        END_PROGRAM
        FUNCTION_BLOCK fb1
        END_FUNCTION_BLOCK
        FUNCTION foo : INT
        END_FUNCTION
        "#
    );

    let my_program = index.find_implementation("myProgram").unwrap();
    assert_eq!(my_program.call_name, "myProgram");
    assert_eq!(my_program.type_name, "myProgram");
    let prog2 = index.find_implementation("prog2").unwrap();
    assert_eq!(prog2.call_name, "prog2");
    assert_eq!(prog2.type_name, "prog2");
    let fb1 = index.find_implementation("fb1").unwrap();
    assert_eq!(fb1.call_name, "fb1");
    assert_eq!(fb1.type_name, "fb1");
    let foo_impl = index.find_implementation("foo").unwrap();
    assert_eq!(foo_impl.call_name, "foo");
    assert_eq!(foo_impl.type_name, "foo");
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
    let result = index.find_variable(None, &["a"]).unwrap();
    assert_eq!(VariableType::Global, result.information.variable_type);
    assert_eq!("a", result.name);
    assert_eq!(None, result.information.qualifier);
    //Asking for a variable with the POU  context finds a local variable
    let result = index.find_variable(Some("prg"), &["a"]).unwrap();
    assert_eq!(VariableType::Local, result.information.variable_type);
    assert_eq!("a", result.name);
    assert_eq!(Some("prg".to_string()), result.information.qualifier);
    //Asking for a variable with th POU context finds a global variable
    let result = index.find_variable(Some("prg"), &["b"]).unwrap();
    assert_eq!(VariableType::Global, result.information.variable_type);
    assert_eq!("b", result.name);
    assert_eq!(None, result.information.qualifier);
    //Asking for a variable with the function context finds the local variable
    let result = index.find_variable(Some("foo"), &["a"]).unwrap();
    assert_eq!(VariableType::Local, result.information.variable_type);
    assert_eq!("a", result.name);
    assert_eq!(Some("foo".to_string()), result.information.qualifier);
    //Asking for a variable with the function context finds the global variable
    let result = index.find_variable(Some("foo"), &["x"]).unwrap();
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
        .find_variable(Some("prg"), &["fb1_inst", "fb2_inst", "fb3_inst", "x"])
        .unwrap();
    assert_eq!(VariableType::Input, result.information.variable_type);
    assert_eq!("x", result.name);
    assert_eq!(Some("fb3".to_string()), result.information.qualifier);
}

#[test]
fn callable_instances_can_be_retreived() {
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

    FUNCTION foo : INT
    END_FUNCTION

    VAR_GLOBAL
        fb1_inst : fb1;
        fb2_inst : fb2;
        fb3_inst : fb3;
        a : INT;
        b : INT;
    END_VAR

    PROGRAM prg
    VAR
        fb1_local : fb1;
        c : INT;
        d : INT;
    END_VAR
        fb1_inst.fb2_inst.fb3_inst.x := 1;
    END_PROGRAM
    "#
    );

    assert_eq!(
        true,
        index
            .find_callable_instance_variable(Some("prg"), &["fb1_inst".into()])
            .is_some()
    );
    assert_eq!(
        true,
        index
            .find_callable_instance_variable(Some("prg"), &["fb2_inst".into()])
            .is_some()
    );
    assert_eq!(
        true,
        index
            .find_callable_instance_variable(Some("prg"), &["fb3_inst".into()])
            .is_some()
    );
    assert_eq!(
        true,
        index
            .find_callable_instance_variable(Some("prg"), &["fb1_local".into()])
            .is_some()
    );
    assert_eq!(
        true,
        index
            .find_callable_instance_variable(
                Some("prg"),
                &["fb1_local".into(), "fb2_inst".into(), "fb3_inst".into()]
            )
            .is_some()
    );
    assert_eq!(
        true,
        index
            .find_callable_instance_variable(Some("prg"), &["fb1_inst".into(), "fb2_inst".into()])
            .is_some()
    );
    assert_eq!(
        true,
        index
            .find_callable_instance_variable(
                Some("prg"),
                &["fb1_inst".into(), "fb2_inst".into(), "fb3_inst".into()]
            )
            .is_some()
    );
    assert_eq!(
        true,
        index
            .find_callable_instance_variable(Some("prg"), &["foo".into()])
            .is_none()
    );
    assert_eq!(
        true,
        index
            .find_callable_instance_variable(Some("prg"), &["a".into()])
            .is_none()
    );
    assert_eq!(
        true,
        index
            .find_callable_instance_variable(Some("prg"), &["b".into()])
            .is_none()
    );
    assert_eq!(
        true,
        index
            .find_callable_instance_variable(Some("prg"), &["c".into()])
            .is_none()
    );
    assert_eq!(
        true,
        index
            .find_callable_instance_variable(Some("prg"), &["d".into()])
            .is_none()
    );
}

#[test]
fn find_type_retrieves_directly_registered_type() {
    let index = index!(
        r"
            TYPE MyAlias : INT;  END_TYPE
            TYPE MySecondAlias : MyAlias;  END_TYPE
            TYPE MyArray : ARRAY[0..10] OF INT;  END_TYPE
            TYPE MyArrayAlias : MyArray; END_TYPE
        "
    );

    let my_alias = index.find_type("MyAlias").unwrap();
    assert_eq!("MyAlias", my_alias.get_name());

    let my_alias = index.find_type("MySecondAlias").unwrap();
    assert_eq!("MySecondAlias", my_alias.get_name());

    let my_alias = index.find_type("MyArrayAlias").unwrap();
    assert_eq!("MyArrayAlias", my_alias.get_name());
}

#[test]
fn find_effective_type_finds_the_inner_effective_type() {
    let index = index!(
        r"
            TYPE MyAlias : INT;  END_TYPE
            TYPE MySecondAlias : MyAlias;  END_TYPE
            TYPE MyArray : ARRAY[0..10] OF INT;  END_TYPE
            TYPE MyArrayAlias : MyArray; END_TYPE
        "
    );

    let my_alias = index.find_type("MyAlias").unwrap().get_type_information();
    let int = index.find_effective_type_information(my_alias).unwrap();
    assert_eq!("INT", int.get_name());

    let my_alias = index
        .find_type("MySecondAlias")
        .unwrap()
        .get_type_information();
    let int = index.find_effective_type_information(my_alias).unwrap();
    assert_eq!("INT", int.get_name());

    let my_alias = index
        .find_type("MyArrayAlias")
        .unwrap()
        .get_type_information();
    let array = index.find_effective_type_information(my_alias).unwrap();
    assert_eq!("MyArray", array.get_name());

    let my_alias = index.find_type("MyArray").unwrap().get_type_information();
    let array = index.find_effective_type_information(my_alias).unwrap();
    assert_eq!("MyArray", array.get_name());
}

#[test]
fn pre_processing_generates_inline_enums_global() {
    // GIVEN a global inline enum
    let src = r#"
        VAR_GLOBAL
            inline_enum : (a,b,c);
        END_VAR
        "#;
    let lexer = lex(src);
    let (mut ast, ..) = parser::parse(lexer);

    // WHEN the AST ist pre-processed
    crate::ast::pre_process(&mut ast);

    //ENUM
    // THEN an implicit datatype should have been generated for the enum
    let new_enum_type = &ast.types[0].data_type;
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
            location: (46..53).into(),
        },
        var_data_type
    );
    assert_eq!(
        src[var_data_type.get_location().to_range()].to_string(),
        "(a,b,c)".to_string()
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
    let lexer = lex(r#"
        VAR_GLOBAL
            inline_struct: STRUCT a: INT; END_STRUCT
        END_VAR
        "#);
    let (mut ast, ..) = parser::parse(lexer);

    // WHEN the AST ist pre-processed
    crate::ast::pre_process(&mut ast);

    //STRUCT
    //THEN an implicit datatype should have been generated for the struct
    let new_struct_type = &ast.types[0].data_type;

    if let DataType::StructType { variables, .. } = new_struct_type {
        assert_eq!(variables[0].location, SourceRange::new(54..55));
    } else {
        panic!("expected struct")
    }

    assert_eq!(
        &DataType::StructType {
            name: Some("__global_inline_struct".to_string()),
            variables: vec![Variable {
                name: "a".to_string(),
                data_type: DataTypeDeclaration::DataTypeReference {
                    referenced_type: "INT".to_string(),
                    location: (57..60).into(),
                },
                location: (54..55).into(),
                initializer: None,
            }]
        },
        new_struct_type
    );

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.global_vars[0].variables[0].data_type;
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__global_inline_struct".to_string(),
            location: (47..72).into(),
        },
        var_data_type
    );
}

#[test]
fn pre_processing_generates_inline_enums() {
    // GIVEN a global inline enum
    let lexer = lex(r#"
        PROGRAM foo
        VAR
            inline_enum : (a,b,c);
        END_VAR
        END_PROGRAM
        "#);
    let (mut ast, ..) = parser::parse(lexer);

    // WHEN the AST ist pre-processed
    crate::ast::pre_process(&mut ast);

    //ENUM
    // THEN an implicit datatype should have been generated for the enum
    let new_enum_type = &ast.types[0].data_type;
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
            location: (59..66).into(),
        },
        var_data_type
    );
}

#[test]
fn pre_processing_generates_inline_structs() {
    // GIVEN a global inline enum
    let lexer = lex(r#"
        PROGRAM foo
        VAR
            inline_struct: STRUCT a: INT; END_STRUCT
        END_VAR
        END_PROGRAM
        "#);
    let (mut ast, ..) = parser::parse(lexer);

    // WHEN the AST ist pre-processed
    crate::ast::pre_process(&mut ast);

    //STRUCT
    //THEN an implicit datatype should have been generated for the struct

    let new_struct_type = &ast.types[0].data_type;
    if let DataType::StructType { variables, .. } = new_struct_type {
        assert_eq!(variables[0].location, SourceRange::new(67..68));
    } else {
        panic!("expected struct")
    }

    assert_eq!(
        &DataType::StructType {
            name: Some("__foo_inline_struct".to_string()),
            variables: vec![Variable {
                name: "a".to_string(),
                data_type: DataTypeDeclaration::DataTypeReference {
                    referenced_type: "INT".to_string(),
                    location: (70..73).into(),
                },
                location: (67..68).into(),
                initializer: None,
            }]
        },
        new_struct_type
    );

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.units[0].variable_blocks[0].variables[0].data_type;
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__foo_inline_struct".to_string(),
            location: (60..85).into(),
        },
        var_data_type
    );
}

#[test]
fn pre_processing_generates_inline_pointers() {
    // GIVEN an inline pointer is declared
    let lexer = lex(r#"
        PROGRAM foo
        VAR
            inline_pointer: REF_TO INT;
        END_VAR
        END_PROGRAM
        "#);
    let (mut ast, ..) = parser::parse(lexer);

    // WHEN the AST ist pre-processed
    crate::ast::pre_process(&mut ast);

    //Pointer
    //THEN an implicit datatype should have been generated for the array
    let new_pointer_type = &ast.types[0];

    let expected = &UserTypeDeclaration {
        data_type: DataType::PointerType {
            name: Some("__foo_inline_pointer".to_string()),
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "INT".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        location: SourceRange::undefined(),
        initializer: None,
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_pointer_type));

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.units[0].variable_blocks[0].variables[0].data_type;
    let expected = &DataTypeDeclaration::DataTypeReference {
        referenced_type: "__foo_inline_pointer".to_string(),
        location: SourceRange::undefined(),
    };

    assert_eq!(format!("{:?}", expected), format!("{:?}", var_data_type));
}

#[test]
fn pre_processing_generates_pointer_to_pointer_type() {
    // GIVEN an inline pointer is declared
    let lexer = lex(r#"
        TYPE pointer_to_pointer: REF_TO REF_TO INT; END_TYPE
        "#);
    let (mut ast, ..) = parser::parse(lexer);

    // WHEN the AST ist pre-processed
    crate::ast::pre_process(&mut ast);

    //Pointer
    //THEN an implicit datatype should have been generated for the pointer

    // POINTER TO INT
    let new_pointer_type = &ast.types[1];
    let expected = &UserTypeDeclaration {
        data_type: DataType::PointerType {
            name: Some("__pointer_to_pointer".to_string()),
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "INT".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        location: SourceRange::undefined(),
        initializer: None,
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_pointer_type));

    // AND the original variable should now point to the new DataType
    let original = &ast.types[0];
    let expected = &UserTypeDeclaration {
        data_type: DataType::PointerType {
            name: Some("pointer_to_pointer".to_string()),
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "__pointer_to_pointer".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        location: SourceRange::undefined(),
        initializer: None,
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", original));
}

#[test]
fn pre_processing_generates_inline_pointer_to_pointer() {
    // GIVEN an inline pointer is declared
    let lexer = lex(r#"
        PROGRAM foo
        VAR
            inline_pointer: REF_TO REF_TO INT;
        END_VAR
        END_PROGRAM
        "#);
    let (mut ast, ..) = parser::parse(lexer);

    // WHEN the AST ist pre-processed
    crate::ast::pre_process(&mut ast);

    //Pointer
    //THEN an implicit datatype should have been generated for the pointer

    // POINTER TO INT
    let new_pointer_type = &ast.types[0];
    let expected = &UserTypeDeclaration {
        data_type: DataType::PointerType {
            name: Some("__foo_inline_pointer_".to_string()),
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "INT".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        location: SourceRange::undefined(),
        initializer: None,
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_pointer_type));

    // Pointer OF Pointer
    let new_pointer_type = &ast.types[1];
    let expected = &UserTypeDeclaration {
        data_type: DataType::PointerType {
            name: Some("__foo_inline_pointer".to_string()),
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "__foo_inline_pointer_".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        location: SourceRange::undefined(),
        initializer: None,
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_pointer_type));

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.units[0].variable_blocks[0].variables[0].data_type;

    let expected = &DataTypeDeclaration::DataTypeReference {
        referenced_type: "__foo_inline_pointer".to_string(),
        location: SourceRange::undefined(),
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", var_data_type));
}

#[test]
fn pre_processing_generates_inline_arrays() {
    // GIVEN an inline array is declared
    let lexer = lex(r#"
        PROGRAM foo
        VAR
            inline_array: ARRAY[0..1] OF INT;
        END_VAR
        END_PROGRAM
        "#);
    let (mut ast, ..) = parser::parse(lexer);

    // WHEN the AST ist pre-processed
    crate::ast::pre_process(&mut ast);

    //ARRAY
    //THEN an implicit datatype should have been generated for the array
    let new_array_type = &ast.types[0];

    let expected = &UserTypeDeclaration {
        data_type: DataType::ArrayType {
            name: Some("__foo_inline_array".to_string()),
            bounds: AstStatement::RangeStatement {
                start: Box::new(AstStatement::LiteralInteger {
                    value: 0,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                end: Box::new(AstStatement::LiteralInteger {
                    value: 1,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                id: 0,
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "INT".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        initializer: None,
        location: (59..77).into(),
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_array_type));

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.units[0].variable_blocks[0].variables[0].data_type;
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__foo_inline_array".to_string(),
            location: (59..77).into(),
        },
        var_data_type
    );
}

#[test]
fn pre_processing_generates_inline_array_of_array() {
    // GIVEN an inline array is declared
    let lexer = lex(r#"
        PROGRAM foo
        VAR
            inline_array: ARRAY[0..1] OF ARRAY[0..1] OF INT;
        END_VAR
        END_PROGRAM
        "#);
    let (mut ast, ..) = parser::parse(lexer);

    // WHEN the AST ist pre-processed
    crate::ast::pre_process(&mut ast);

    //ARRAY
    //THEN an implicit datatype should have been generated for the array

    // ARRAY OF INT
    let new_array_type = &ast.types[0];
    let expected = &UserTypeDeclaration {
        data_type: DataType::ArrayType {
            name: Some("__foo_inline_array_".to_string()),
            bounds: AstStatement::RangeStatement {
                start: Box::new(AstStatement::LiteralInteger {
                    value: 0,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                end: Box::new(AstStatement::LiteralInteger {
                    value: 1,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                id: 0,
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "INT".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        initializer: None,
        location: (59..92).into(),
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_array_type));

    // ARRAY OF ARRAY
    let new_array_type = &ast.types[1];
    let expected = &UserTypeDeclaration {
        data_type: DataType::ArrayType {
            name: Some("__foo_inline_array".to_string()),
            bounds: AstStatement::RangeStatement {
                start: Box::new(AstStatement::LiteralInteger {
                    value: 0,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                end: Box::new(AstStatement::LiteralInteger {
                    value: 1,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                id: 0,
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "__foo_inline_array_".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        initializer: None,
        location: (59..92).into(),
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_array_type));

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.units[0].variable_blocks[0].variables[0].data_type;
    println!("{:#?}", var_data_type.get_location());
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__foo_inline_array".to_string(),
            location: (59..92).into(),
        },
        var_data_type
    );
}

#[test]
fn pre_processing_generates_array_of_array_type() {
    // GIVEN an inline pointer is declared
    let lexer = lex(r#"
        TYPE arr_arr: ARRAY[0..1] OF ARRAY[0..1] OF INT; END_TYPE
        "#);
    let (mut ast, ..) = parser::parse(lexer);

    // WHEN the AST ist pre-processed
    crate::ast::pre_process(&mut ast);

    let new_type = &ast.types[1];
    let expected = &UserTypeDeclaration {
        data_type: DataType::ArrayType {
            name: Some("__arr_arr".to_string()),
            bounds: AstStatement::RangeStatement {
                id: 0,
                start: Box::new(literal_int(0)),
                end: Box::new(literal_int(1)),
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "INT".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        location: SourceRange::undefined(),
        initializer: None,
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_type));

    // AND the original variable should now point to the new DataType
    let original = &ast.types[0];
    let expected = &UserTypeDeclaration {
        data_type: DataType::ArrayType {
            name: Some("arr_arr".to_string()),
            bounds: AstStatement::RangeStatement {
                id: 0,
                start: Box::new(literal_int(0)),
                end: Box::new(literal_int(1)),
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "__arr_arr".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        location: SourceRange::undefined(),
        initializer: None,
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", original));
}

#[test]
fn pre_processing_nested_array_in_struct() {
    let lexer = lex(r#"
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
        "#);

    let (mut ast, ..) = parser::parse(lexer);

    // WHEN the AST ist pre-processed
    crate::ast::pre_process(&mut ast);

    //THEN an implicit datatype should have been generated for the array

    // Struct Type
    let new_array_type = &ast.types[0];
    let expected = &UserTypeDeclaration {
        data_type: DataType::StructType {
            name: Some("MyStruct".to_string()),
            variables: vec![Variable {
                name: "field1".to_string(),
                data_type: DataTypeDeclaration::DataTypeReference {
                    referenced_type: "__MyStruct_field1".to_string(),
                    location: SourceRange::undefined(),
                },
                location: SourceRange::undefined(),
                initializer: None,
            }],
        },
        initializer: None,
        location: (14..97).into(),
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_array_type));

    // ARRAY OF INT
    let new_array_type = &ast.types[1];
    let expected = &UserTypeDeclaration {
        data_type: DataType::ArrayType {
            name: Some("__MyStruct_field1".to_string()),
            bounds: AstStatement::RangeStatement {
                start: Box::new(AstStatement::LiteralInteger {
                    value: 0,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                end: Box::new(AstStatement::LiteralInteger {
                    value: 4,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                id: 0,
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "INT".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        initializer: None,
        location: (59..77).into(),
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_array_type));
}

#[test]
fn pre_processing_generates_inline_array_of_array_of_array() {
    // GIVEN an inline array is declared
    let lexer = lex(r#"
        PROGRAM foo
        VAR
            inline_array: ARRAY[0..1] OF ARRAY[0..1] OF ARRAY[0..1] OF INT;
        END_VAR
        END_PROGRAM
        "#);
    let (mut ast, ..) = parser::parse(lexer);

    // WHEN the AST ist pre-processed
    crate::ast::pre_process(&mut ast);

    //ARRAY
    //THEN an implicit datatype should have been generated for the array

    // ARRAY OF INT
    let new_array_type = &ast.types[0];
    let expected = &UserTypeDeclaration {
        data_type: DataType::ArrayType {
            name: Some("__foo_inline_array__".to_string()),
            bounds: AstStatement::RangeStatement {
                start: Box::new(AstStatement::LiteralInteger {
                    value: 0,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                end: Box::new(AstStatement::LiteralInteger {
                    value: 1,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                id: 0,
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "INT".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        initializer: None,
        location: (74..107).into(),
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_array_type));

    // ARRAY OF ARRAY
    let new_array_type = &ast.types[1];
    let expected = UserTypeDeclaration {
        data_type: DataType::ArrayType {
            name: Some("__foo_inline_array_".to_string()),
            bounds: AstStatement::RangeStatement {
                start: Box::new(AstStatement::LiteralInteger {
                    value: 0,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                end: Box::new(AstStatement::LiteralInteger {
                    value: 1,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                id: 0,
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "__foo_inline_array__".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        initializer: None,
        location: (59..107).into(),
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_array_type));

    // ARRAY OF ARRAY
    let new_array_type = &ast.types[2];
    let expected = UserTypeDeclaration {
        data_type: DataType::ArrayType {
            name: Some("__foo_inline_array".to_string()),
            bounds: AstStatement::RangeStatement {
                start: Box::new(AstStatement::LiteralInteger {
                    value: 0,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                end: Box::new(AstStatement::LiteralInteger {
                    value: 1,
                    location: SourceRange::undefined(),
                    id: 0,
                }),
                id: 0,
            },
            referenced_type: Box::new(DataTypeDeclaration::DataTypeReference {
                referenced_type: "__foo_inline_array_".to_string(),
                location: SourceRange::undefined(),
            }),
        },
        initializer: None,
        location: (59..107).into(),
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_array_type));

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.units[0].variable_blocks[0].variables[0].data_type;
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__foo_inline_array".to_string(),
            location: (59..107).into(),
        },
        var_data_type
    );
}

#[test]
fn sub_range_boundaries_are_registered_at_the_index() {
    // GIVEN a Subrange INT from 7 to 1000
    let src = "
        TYPE MyInt: INT(7..1000); END_TYPE 
        TYPE MyAliasInt: MyInt; END_TYPE 
        ";
    // WHEN the program is indexed
    let index = index!(src);

    // THEN I expect the index to contain the defined range-information for the given type
    let my_int = &index.get_type("MyInt").unwrap().information;
    let expected = &DataTypeInformation::SubRange {
        name: "MyInt".to_string(),
        referenced_type: "INT".to_string(),
        sub_range: AstStatement::LiteralInteger {
            value: 7,
            location: SourceRange::undefined(),
            id: 0,
        }..AstStatement::LiteralInteger {
            value: 1000,
            location: SourceRange::undefined(),
            id: 0,
        },
    };

    assert_eq!(format!("{:?}", expected), format!("{:?}", my_int));

    // THEN I expect the index to contain the defined range-information for the given type
    let my_int = &index.get_type("MyAliasInt").unwrap().information;
    let expected = &DataTypeInformation::Alias {
        name: "MyAliasInt".to_string(),
        referenced_type: "MyInt".to_string(),
    };

    assert_eq!(format!("{:?}", expected), format!("{:?}", my_int));
}

#[test]
fn global_initializers_are_stored_in_the_const_expression_arena() {
    // GIVEN some globals with initial value expressions
    let src = "
        VAR_GLOBAL
            a : INT := x + 1;
            b : INT := y + 1;
            c : INT := z + 1;
        END_VAR
        ";
    // WHEN the program is indexed
    let ids = IdProvider::new();
    let (mut ast, ..) = crate::parser::parse(crate::lexer::lex_with_ids(src, ids.clone()));

    crate::ast::pre_process(&mut ast);
    let index = crate::index::visitor::visit(&ast, ids.clone());

    // THEN I expect the index to contain cosntant expressions (x+1), (y+1) and (z+1) as const expressions
    // associated with the initial values of the globals
    let variables = &ast.global_vars[0].variables;
    let initializer = index.find_global_variable("a").and_then(|g| {
        index
            .get_const_expressions()
            .maybe_get_constant_statement(&g.initial_value)
    });
    assert_eq!(variables[0].initializer.as_ref(), initializer);

    let initializer = index.find_global_variable("b").and_then(|g| {
        index
            .get_const_expressions()
            .maybe_get_constant_statement(&g.initial_value)
    });
    assert_eq!(variables[1].initializer.as_ref(), initializer);

    let initializer = index.find_global_variable("c").and_then(|g| {
        index
            .get_const_expressions()
            .maybe_get_constant_statement(&g.initial_value)
    });
    assert_eq!(variables[2].initializer.as_ref(), initializer);
}

#[test]
fn local_initializers_are_stored_in_the_const_expression_arena() {
    // GIVEN some local members with initial value expressions
    let src = "
        PROGRAM prg
        VAR_INPUT
            a : INT := x + 1;
            b : INT := y + 1;
            c : INT := z + 1;
        END_VAR
        END_PROGRAM
        ";
    // WHEN the program is indexed
    let ids = IdProvider::new();
    let (mut ast, ..) = crate::parser::parse(crate::lexer::lex_with_ids(src, ids.clone()));

    crate::ast::pre_process(&mut ast);
    let index = crate::index::visitor::visit(&ast, ids.clone());

    // THEN I expect the index to contain cosntant expressions (x+1), (y+1) and (z+1) as const expressions
    // associated with the initial values of the members
    let variables = &ast.units[0].variable_blocks[0].variables;
    let initializer = index.find_member("prg", "a").and_then(|g| {
        index
            .get_const_expressions()
            .maybe_get_constant_statement(&g.initial_value)
    });
    assert_eq!(variables[0].initializer.as_ref(), initializer);

    let initializer = index.find_member("prg", "b").and_then(|g| {
        index
            .get_const_expressions()
            .maybe_get_constant_statement(&g.initial_value)
    });
    assert_eq!(variables[1].initializer.as_ref(), initializer);

    let initializer = index.find_member("prg", "c").and_then(|g| {
        index
            .get_const_expressions()
            .maybe_get_constant_statement(&g.initial_value)
    });
    assert_eq!(variables[2].initializer.as_ref(), initializer);
}

#[test]
fn datatype_initializers_are_stored_in_the_const_expression_arena() {
    // GIVEN a datatype with an initial value expression
    let src = "
        TYPE MyInt : INT := 7 + x;
        ";
    // WHEN the program is indexed
    let ids = IdProvider::new();
    let (mut ast, ..) = crate::parser::parse(crate::lexer::lex_with_ids(src, ids.clone()));

    crate::ast::pre_process(&mut ast);
    let index = crate::index::visitor::visit(&ast, ids.clone());

    // THEN I expect the index to contain cosntant expressions (7+x) as const expressions
    // associated with the initial values of the type
    let data_type = &ast.types[0];
    let initializer = index.find_type("MyInt").and_then(|g| {
        index
            .get_const_expressions()
            .maybe_get_constant_statement(&g.initial_value)
    });
    assert_eq!(data_type.initializer.as_ref(), initializer);
}

#[test]
fn array_dimensions_are_stored_in_the_const_expression_arena() {
    // GIVEN an array-datatype with constant expressions used in the dimensions
    let src = "
        TYPE MyInt : ARRAY[0 .. LEN-1, MIN .. MAX] OF INT;
        ";
    // WHEN the program is indexed
    let ids = IdProvider::new();
    let (mut ast, ..) = crate::parser::parse(crate::lexer::lex_with_ids(src, ids.clone()));

    crate::ast::pre_process(&mut ast);
    let index = crate::index::visitor::visit(&ast, ids.clone());

    // THEN I expect the index to contain constants expressions used in the array-dimensions

    // check first dimensions 0 .. LEN-1
    let (start_0, end_0) = index
        .find_type_information("MyInt")
        .map(|it| {
            if let DataTypeInformation::Array { dimensions, .. } = &it {
                //return the pair (start, end)
                (
                    dimensions[0].start_offset.as_int_value(&index).unwrap(),
                    dimensions[0]
                        .end_offset
                        .as_const_expression(&index)
                        .unwrap(),
                )
            } else {
                unreachable!()
            }
        })
        .unwrap();

    assert_eq!(start_0, 0);
    assert_eq!(
        format!(
            "{:#?}",
            AstStatement::BinaryExpression {
                id: 0,
                operator: Operator::Minus,
                left: Box::new(crate::parser::tests::ref_to("LEN")),
                right: Box::new(crate::parser::tests::literal_int(1))
            }
        ),
        format!("{:#?}", end_0)
    );

    //check 2nd dimension MIN .. MAX
    let (start_1, end_1) = index
        .find_type_information("MyInt")
        .map(|it| {
            if let DataTypeInformation::Array { dimensions, .. } = it {
                //return the pair (start, end)
                (
                    dimensions[1]
                        .start_offset
                        .as_const_expression(&index)
                        .unwrap(),
                    dimensions[1]
                        .end_offset
                        .as_const_expression(&index)
                        .unwrap(),
                )
            } else {
                unreachable!()
            }
        })
        .unwrap();

    assert_eq!(
        format!("{:#?}", crate::parser::tests::ref_to("MIN")),
        format!("{:#?}", start_1)
    );

    assert_eq!(
        format!("{:#?}", crate::parser::tests::ref_to("MAX")),
        format!("{:#?}", end_1)
    );
}

#[test]
fn string_dimensions_are_stored_in_the_const_expression_arena() {
    // GIVEN a string type with a const expression as length
    let src = "
        TYPE MyString : STRING[LEN-1];
        ";
    // WHEN the program is indexed
    let ids = IdProvider::new();
    let (mut ast, ..) = crate::parser::parse(crate::lexer::lex_with_ids(src, ids.clone()));

    crate::ast::pre_process(&mut ast);
    let index = crate::index::visitor::visit(&ast, ids.clone());

    // THEN I expect the index to contain constants expressions used in the string-len

    let data_type = &ast.types[0].data_type;
    let actual_len_expression = if let DataType::StringType { size, .. } = data_type {
        size.as_ref().unwrap()
    } else {
        unreachable!()
    };
    if let Some(DataTypeInformation::String {
        size: TypeSize::ConstExpression(expr),
        ..
    }) = index.find_type_information("MyString")
    {
        assert_eq!(
            format!(
                "{:#?}",
                &AstStatement::BinaryExpression {
                    id: actual_len_expression.get_id(),
                    left: Box::new(actual_len_expression.clone()),
                    operator: Operator::Plus,
                    right: Box::new(crate::parser::tests::literal_int(1))
                }
            ),
            format!(
                "{:#?}",
                index
                    .get_const_expressions()
                    .get_constant_statement(&expr)
                    .unwrap()
            )
        );
    } else {
        unreachable!()
    }
}
