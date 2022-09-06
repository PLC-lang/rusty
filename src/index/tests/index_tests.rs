// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use pretty_assertions::assert_eq;

use crate::index::{ArgumentType, PouIndexEntry, VariableIndexEntry};
use crate::lexer::IdProvider;
use crate::parser::tests::literal_int;
use crate::test_utils::tests::{annotate, index, parse_and_preprocess};
use crate::typesystem::{TypeSize, INT_TYPE, VOID_TYPE};
use crate::{ast::*, index::VariableType, typesystem::DataTypeInformation};

#[test]
fn index_not_case_sensitive() {
    let (_, index) = index(
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
    "#,
    );

    let entry = index.find_global_variable("A").unwrap();
    assert_eq!("a", entry.name);
    assert_eq!("INT", entry.data_type_name);
    let entry = index.find_global_variable("X").unwrap();
    assert_eq!("x", entry.name);
    assert_eq!("ST", entry.data_type_name);
    let entry = index.find_member("ST", "X").unwrap();
    assert_eq!("x", entry.name);
    assert_eq!("INT", entry.data_type_name);
    let entry = index.find_effective_type_by_name("APROGRAM").unwrap();
    assert_eq!("aProgram", entry.name);
    let entry = index.find_implementation_by_name("Foo").unwrap();
    assert_eq!("foo", entry.call_name);
    assert_eq!("foo", entry.type_name);
}

#[test]
fn global_variables_are_indexed() {
    let (_, index) = index(
        r#"
        VAR_GLOBAL
            a: INT;
            b: BOOL;
        END_VAR
    "#,
    );

    let entry_a = index.find_global_variable("a").unwrap();
    assert_eq!("a", entry_a.name);
    assert_eq!("INT", entry_a.data_type_name);

    let entry_b = index.find_global_variable("b").unwrap();
    assert_eq!("b", entry_b.name);
    assert_eq!("BOOL", entry_b.data_type_name);
}

#[test]
fn program_is_indexed() {
    let (_, index) = index(
        r#"
        PROGRAM myProgram
        END_PROGRAM
    "#,
    );

    index.find_effective_type_by_name("myProgram").unwrap();
}

#[test]
fn actions_are_indexed() {
    let (_, index) = index(
        r#"
        PROGRAM myProgram
        END_PROGRAM
        ACTIONS myProgram
            ACTION foo
            END_ACTION
        END_ACTIONS
        ACTION myProgram.bar
        END_ACTION
    "#,
    );

    let foo_impl = index.find_implementation_by_name("myProgram.foo").unwrap();
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
        index.find_effective_type_info(info.get_name()).unwrap()
    {
        assert_eq!("myProgram_interface", name);
    } else {
        panic!("Wrong variant : {:#?}", info);
    }

    let bar = index.find_implementation_by_name("myProgram.bar").unwrap();
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
        index.find_effective_type_info(info.get_name()).unwrap()
    {
        assert_eq!("myProgram_interface", name);
    } else {
        panic!("Wrong variant : {:#?}", info);
    }
}

#[test]
fn fb_methods_are_indexed() {
    let (_, index) = index(
        r#"
        FUNCTION_BLOCK myFuncBlock
            METHOD foo
                VAR x : SINT; END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );

    let foo_impl = index
        .find_implementation_by_name("myFuncBlock.foo")
        .unwrap();
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
    let (_, index) = index(
        r#"
        CLASS myClass
            METHOD foo
                VAR y : DINT; END_VAR
            END_METHOD
        END_CLASS
    "#,
    );

    let foo_impl = index.find_implementation_by_name("myClass.foo").unwrap();
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
    let (_, index) = index(
        r#"
        FUNCTION myFunction : INT
        END_FUNCTION
    "#,
    );

    index.find_effective_type_by_name("myFunction").unwrap();

    let return_variable = index.find_member("myFunction", "myFunction").unwrap();
    assert_eq!("myFunction", return_variable.name);
    assert_eq!("INT", return_variable.data_type_name);
    assert_eq!(VariableType::Return, return_variable.get_variable_type());
}

#[test]
fn function_with_varargs_param_marked() {
    let (_, index) = index(
        r#"
        FUNCTION myFunc : INT
        VAR_INPUT
            x : INT;
            y : ...;
        END_VAR
        END_FUNCTION
        "#,
    );
    let function = index.find_pou("myFunc").unwrap();
    assert!(function.is_variadic());
    assert_eq!(
        VOID_TYPE,
        index.get_variadic_member("myFunc").unwrap().get_type_name()
    );
}

#[test]
fn function_with_typed_varargs_param_marked() {
    let (_, index) = index(
        r#"
        FUNCTION myFunc : INT
        VAR_INPUT
            x : INT;
            y : INT...;
        END_VAR
        END_FUNCTION
        "#,
    );
    let function = index.find_pou("myFunc").unwrap();
    assert!(function.is_variadic());
    assert_eq!(
        INT_TYPE,
        index.get_variadic_member("myFunc").unwrap().get_type_name()
    );
}

#[test]
fn pous_are_indexed() {
    let (_, index) = index(
        r#"
        PROGRAM myProgram
        END_PROGRAM
        
        FUNCTION myFunction : INT
        END_FUNCTION
        
        FUNCTION_BLOCK myFunctionBlock : INT
        END_FUNCTION_BLOCK
        
        CLASS myClass
        END_CLASS

        ACTIONS myProgram
            ACTION act
            END_ACTION
        END_ACTIONS
    "#,
    );

    index.find_effective_type_by_name("myFunction").unwrap();
    index.find_effective_type_by_name("myProgram").unwrap();
    index
        .find_effective_type_by_name("myFunctionBlock")
        .unwrap();
    index.find_effective_type_by_name("myClass").unwrap();
    index.find_effective_type_by_name("myProgram.act").unwrap();
}

#[test]
fn implementations_are_indexed() {
    let (_, index) = index(
        r#"
        PROGRAM myProgram
        END_PROGRAM
        PROGRAM prog2
        END_PROGRAM
        FUNCTION_BLOCK fb1
        END_FUNCTION_BLOCK
        FUNCTION foo : INT
        END_FUNCTION
        "#,
    );

    let my_program = index.find_implementation_by_name("myProgram").unwrap();
    assert_eq!(my_program.call_name, "myProgram");
    assert_eq!(my_program.type_name, "myProgram");
    let prog2 = index.find_implementation_by_name("prog2").unwrap();
    assert_eq!(prog2.call_name, "prog2");
    assert_eq!(prog2.type_name, "prog2");
    let fb1 = index.find_implementation_by_name("fb1").unwrap();
    assert_eq!(fb1.call_name, "fb1");
    assert_eq!(fb1.type_name, "fb1");
    let foo_impl = index.find_implementation_by_name("foo").unwrap();
    assert_eq!(foo_impl.call_name, "foo");
    assert_eq!(foo_impl.type_name, "foo");
}

#[test]
fn program_members_are_indexed() {
    let (_, index) = index(
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
        VAR_TEMP
            e : INT;
            f : INT;
        END_VAR
        END_PROGRAM
    "#,
    );

    let variable = index.find_member("myProgram", "a").unwrap();
    assert_eq!("a", variable.name);
    assert_eq!("INT", variable.data_type_name);
    assert_eq!(VariableType::Local, variable.get_variable_type());

    let variable = index.find_member("myProgram", "b").unwrap();
    assert_eq!("b", variable.name);
    assert_eq!("INT", variable.data_type_name);
    assert_eq!(VariableType::Local, variable.get_variable_type());

    let variable = index.find_member("myProgram", "c").unwrap();
    assert_eq!("c", variable.name);
    assert_eq!("BOOL", variable.data_type_name);
    assert_eq!(VariableType::Input, variable.get_variable_type());

    let variable = index.find_member("myProgram", "d").unwrap();
    assert_eq!("d", variable.name);
    assert_eq!("BOOL", variable.data_type_name);
    assert_eq!(VariableType::Input, variable.get_variable_type());

    let variable = index.find_member("myProgram", "e").unwrap();
    assert_eq!("e", variable.name);
    assert_eq!("INT", variable.data_type_name);
    assert_eq!(VariableType::Temp, variable.get_variable_type());

    let variable = index.find_member("myProgram", "f").unwrap();
    assert_eq!("f", variable.name);
    assert_eq!("INT", variable.data_type_name);
    assert_eq!(VariableType::Temp, variable.get_variable_type());
}

#[test]
fn given_set_of_local_global_and_functions_the_index_can_be_retrieved() {
    let (_, index) = index(
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
        "#,
    );

    //Asking for a variable with no context returns global variables
    let result = index.find_variable(None, &["a"]).unwrap();
    assert_eq!(VariableType::Global, result.get_variable_type());
    assert_eq!("a", result.name);
    //Asking for a variable with the POU  context finds a local variable
    let result = index.find_variable(Some("prg"), &["a"]).unwrap();
    assert_eq!(VariableType::Local, result.get_variable_type());
    assert_eq!("a", result.name);
    //Asking for a variable with th POU context finds a global variable
    let result = index.find_variable(Some("prg"), &["b"]).unwrap();
    assert_eq!(VariableType::Global, result.get_variable_type());
    assert_eq!("b", result.name);
    //Asking for a variable with the function context finds the local variable
    let result = index.find_variable(Some("foo"), &["a"]).unwrap();
    assert_eq!(VariableType::Local, result.get_variable_type());
    assert_eq!("a", result.name);
    //Asking for a variable with the function context finds the global variable
    let result = index.find_variable(Some("foo"), &["x"]).unwrap();
    assert_eq!(VariableType::Global, result.get_variable_type());
    assert_eq!("x", result.name);
}

#[test]
fn index_can_be_retrieved_from_qualified_name() {
    let (_, index) = index(
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
    "#,
    );

    let result = index
        .find_variable(Some("prg"), &["fb1_inst", "fb2_inst", "fb3_inst", "x"])
        .unwrap();
    assert_eq!(VariableType::Input, result.get_variable_type());
    assert_eq!("x", result.name);
}

#[test]
fn callable_instances_can_be_retreived() {
    let (_, index) = index(
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
    "#,
    );

    assert!(index
        .find_callable_instance_variable(Some("prg"), &["fb1_inst"])
        .is_some());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["fb2_inst"])
        .is_some());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["fb3_inst"])
        .is_some());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["fb1_local"])
        .is_some());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["fb1_local", "fb2_inst", "fb3_inst"])
        .is_some());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["fb1_inst", "fb2_inst"])
        .is_some());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["fb1_inst", "fb2_inst", "fb3_inst"])
        .is_some());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["foo"])
        .is_none());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["a"])
        .is_none());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["b"])
        .is_none());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["c"])
        .is_none());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["d"])
        .is_none());
}

#[test]
fn get_type_retrieves_directly_registered_type() {
    let (_, index) = index(
        r"
            TYPE MyAlias : INT;  END_TYPE
            TYPE MySecondAlias : MyAlias;  END_TYPE
            TYPE MyArray : ARRAY[0..10] OF INT;  END_TYPE
            TYPE MyArrayAlias : MyArray; END_TYPE
        ",
    );

    let my_alias = index.get_type("MyAlias").unwrap();
    assert_eq!("MyAlias", my_alias.get_name());

    let my_alias = index.get_type("MySecondAlias").unwrap();
    assert_eq!("MySecondAlias", my_alias.get_name());

    let my_alias = index.get_type("MyArrayAlias").unwrap();
    assert_eq!("MyArrayAlias", my_alias.get_name());
}

#[test]
fn find_effective_type_finds_the_inner_effective_type() {
    let (_, index) = index(
        r"
            TYPE MyAlias : INT;  END_TYPE
            TYPE MySecondAlias : MyAlias;  END_TYPE
            TYPE MyArray : ARRAY[0..10] OF INT;  END_TYPE
            TYPE MyArrayAlias : MyArray; END_TYPE
        ",
    );

    let my_alias = "MyAlias";
    let int = index.find_effective_type_by_name(my_alias).unwrap();
    assert_eq!("INT", int.get_name());

    let my_alias = "MySecondAlias";
    let int = index.find_effective_type_by_name(my_alias).unwrap();
    assert_eq!("INT", int.get_name());

    let my_alias = "MyArrayAlias";
    let array = index.find_effective_type_by_name(my_alias).unwrap();
    assert_eq!("MyArray", array.get_name());

    let my_alias = "MyArray";
    let array = index.find_effective_type_by_name(my_alias).unwrap();
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
    let (ast, ..) = parse_and_preprocess(src);

    //ENUM
    // THEN an implicit datatype should have been generated for the enum
    insta::assert_debug_snapshot!(ast.types[0].data_type);

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
    let src = r#"
        VAR_GLOBAL
            inline_struct: STRUCT a: INT; END_STRUCT
        END_VAR
        "#;
    let (ast, ..) = parse_and_preprocess(src);

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
                address: None,
            },],
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
    let src = r#"
        PROGRAM foo
        VAR
            inline_enum : (a,b,c);
        END_VAR
        END_PROGRAM
        "#;
    let (ast, ..) = parse_and_preprocess(src);

    //ENUM
    //
    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.units[0].variable_blocks[0].variables[0].data_type;
    assert_eq!(
        &DataTypeDeclaration::DataTypeReference {
            referenced_type: "__foo_inline_enum".to_string(),
            location: (59..66).into(),
        },
        var_data_type
    );

    // THEN an implicit datatype should have been generated for the enum
    let new_enum_type = &ast.types[0].data_type;
    insta::assert_debug_snapshot!(new_enum_type);
}

#[test]
fn pre_processing_generates_inline_structs() {
    // GIVEN a global inline enum
    let src = r#"
        PROGRAM foo
        VAR
            inline_struct: STRUCT a: INT; END_STRUCT
        END_VAR
        END_PROGRAM
        "#;
    let (ast, ..) = parse_and_preprocess(src);

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
                address: None,
            }],
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
    let src = r#"
        PROGRAM foo
        VAR
            inline_pointer: REF_TO INT;
        END_VAR
        END_PROGRAM
        "#;
    let (ast, ..) = parse_and_preprocess(src);

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
        scope: Some("foo".into()),
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
    let src = r#"
        TYPE pointer_to_pointer: REF_TO REF_TO INT; END_TYPE
        "#;
    let (ast, ..) = parse_and_preprocess(src);

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
        scope: None,
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
        scope: None,
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", original));
}

#[test]
fn pre_processing_generates_inline_pointer_to_pointer() {
    // GIVEN an inline pointer is declared
    let src = r#"
        PROGRAM foo
        VAR
            inline_pointer: REF_TO REF_TO INT;
        END_VAR
        END_PROGRAM
        "#;
    let (ast, ..) = parse_and_preprocess(src);

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
        scope: Some("foo".into()),
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
        scope: Some("foo".into()),
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
    let src = r#"
        PROGRAM foo
        VAR
            inline_array: ARRAY[0..1] OF INT;
        END_VAR
        END_PROGRAM
        "#;
    let (ast, ..) = parse_and_preprocess(src);

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
        scope: Some("foo".into()),
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
    let src = r#"
        PROGRAM foo
        VAR
            inline_array: ARRAY[0..1] OF ARRAY[0..1] OF INT;
        END_VAR
        END_PROGRAM
        "#;
    let (ast, ..) = parse_and_preprocess(src);

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
        scope: Some("foo".into()),
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
        scope: Some("foo".into()),
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
    let src = r#"
        TYPE arr_arr: ARRAY[0..1] OF ARRAY[0..1] OF INT; END_TYPE
        "#;
    let (ast, ..) = parse_and_preprocess(src);

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
        scope: None,
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
        scope: None,
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", original));
}

#[test]
fn pre_processing_nested_array_in_struct() {
    let src = r#"
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
        "#;

    let (ast, ..) = parse_and_preprocess(src);

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
                address: None,
            }],
        },
        initializer: None,
        location: (14..97).into(),
        scope: None,
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
        scope: None,
    };
    assert_eq!(format!("{:?}", expected), format!("{:?}", new_array_type));
}

#[test]
fn pre_processing_generates_inline_array_of_array_of_array() {
    // GIVEN an inline array is declared
    let src = r#"
        PROGRAM foo
        VAR
            inline_array: ARRAY[0..1] OF ARRAY[0..1] OF ARRAY[0..1] OF INT;
        END_VAR
        END_PROGRAM
        "#;
    let (ast, ..) = parse_and_preprocess(src);

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
        scope: Some("foo".into()),
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
        scope: Some("foo".into()),
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
        scope: Some("foo".into()),
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
fn pre_processing_generates_generic_types() {
    // GIVEN a function with a generic type G: ANY
    let src = "
        FUNCTION myFunc<G : ANY> : G
        VAR_INPUT
            in1 : G;
            in2 : INT;
        END_VAR
        END_FUNCTION
        ";
    let (ast, ..) = parse_and_preprocess(src);

    assert_eq!(1, ast.types.len());
    //A type __myFunc__G is created
    let expected = UserTypeDeclaration {
        data_type: DataType::GenericType {
            name: "__myFunc__G".into(),
            generic_symbol: "G".into(),
            nature: TypeNature::Any,
        },
        initializer: None,
        location: SourceRange::undefined(),
        scope: Some("myFunc".into()),
    };

    assert_eq!(format!("{:?}", expected), format!("{:?}", ast.types[0]));

    //The variables with type G now have type __myFunc__G
    let pou = &ast.units[0];
    assert_eq!(
        pou.variable_blocks[0].variables[0]
            .data_type
            .get_name()
            .unwrap(),
        "__myFunc__G"
    );
    assert_eq!(
        pou.variable_blocks[0].variables[1]
            .data_type
            .get_name()
            .unwrap(),
        "INT"
    );
    assert_eq!(
        pou.return_type.as_ref().unwrap().get_name().unwrap(),
        "__myFunc__G"
    );
}

#[test]
fn pre_processing_generates_nested_generic_types() {
    // GIVEN a function with a generic type G: ANY
    let src = "
        FUNCTION myFunc<G : ANY> : REF_TO G
        VAR_INPUT
            in1 : ARRAY[0..1] OF G;
            in2 : INT;
        END_VAR
        END_FUNCTION
        ";
    let (ast, ..) = parse_and_preprocess(src);

    //A type __myFunc__G is created
    let expected = UserTypeDeclaration {
        data_type: DataType::GenericType {
            name: "__myFunc__G".into(),
            generic_symbol: "G".into(),
            nature: TypeNature::Any,
        },
        initializer: None,
        location: SourceRange::undefined(),
        scope: Some("myFunc".into()),
    };

    assert_eq!(format!("{:?}", expected), format!("{:?}", ast.types[0]));
    //Additional types created
    assert_eq!(3, ast.types.len());
    //referenced types of additional types are the new type
    if let DataType::ArrayType {
        referenced_type, ..
    } = &ast.types[1].data_type
    {
        assert_eq!(referenced_type.get_name().unwrap(), "__myFunc__G");
    } else {
        panic!("expected array");
    }
    if let DataType::PointerType {
        referenced_type, ..
    } = &ast.types[2].data_type
    {
        assert_eq!(referenced_type.get_name().unwrap(), "__myFunc__G");
    } else {
        panic!("expected pointer");
    }
}

#[test]
fn sub_range_boundaries_are_registered_at_the_index() {
    // GIVEN a Subrange INT from 7 to 1000
    let src = "
        TYPE MyInt: INT(7..1000); END_TYPE 
        TYPE MyAliasInt: MyInt; END_TYPE 
        ";
    // WHEN the program is indexed
    let (_, index) = index(src);

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
    let ids = IdProvider::default();
    let (mut ast, ..) = crate::parser::parse(
        crate::lexer::lex_with_ids(src, ids.clone()),
        LinkageType::Internal,
    );

    crate::ast::pre_process(&mut ast, ids.clone());
    let index = crate::index::visitor::visit(&ast, ids);

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
    let ids = IdProvider::default();
    let (mut ast, ..) = crate::parser::parse(
        crate::lexer::lex_with_ids(src, ids.clone()),
        LinkageType::Internal,
    );

    crate::ast::pre_process(&mut ast, ids.clone());
    let index = crate::index::visitor::visit(&ast, ids);

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
    let ids = IdProvider::default();
    let (mut ast, ..) = crate::parser::parse(
        crate::lexer::lex_with_ids(src, ids.clone()),
        LinkageType::Internal,
    );

    crate::ast::pre_process(&mut ast, ids.clone());
    let index = crate::index::visitor::visit(&ast, ids);

    // THEN I expect the index to contain cosntant expressions (7+x) as const expressions
    // associated with the initial values of the type
    let data_type = &ast.types[0];
    let initializer = index
        .get_type("MyInt")
        .map(|g| {
            index
                .get_const_expressions()
                .maybe_get_constant_statement(&g.initial_value)
        })
        .unwrap();
    assert_eq!(data_type.initializer.as_ref(), initializer);
}

#[test]
fn array_dimensions_are_stored_in_the_const_expression_arena() {
    // GIVEN an array-datatype with constant expressions used in the dimensions
    let src = "
        TYPE MyInt : ARRAY[0 .. LEN-1, MIN .. MAX] OF INT;
        ";
    // WHEN the program is indexed
    let ids = IdProvider::default();
    let (mut ast, ..) = crate::parser::parse(
        crate::lexer::lex_with_ids(src, ids.clone()),
        LinkageType::Internal,
    );

    crate::ast::pre_process(&mut ast, ids.clone());
    let index = crate::index::visitor::visit(&ast, ids);

    // THEN I expect the index to contain constants expressions used in the array-dimensions

    // check first dimensions 0 .. LEN-1
    let (start_0, end_0) = index
        .find_effective_type_info("MyInt")
        .map(|it| {
            if let DataTypeInformation::Array { dimensions, .. } = it {
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
        .find_effective_type_info("MyInt")
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
    let ids = IdProvider::default();
    let (mut ast, ..) = crate::parser::parse(
        crate::lexer::lex_with_ids(src, ids.clone()),
        LinkageType::Internal,
    );

    crate::ast::pre_process(&mut ast, ids.clone());
    let index = crate::index::visitor::visit(&ast, ids);

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
    }) = index.find_effective_type_info("MyString")
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
                    .get_constant_statement(expr)
                    .unwrap()
            )
        );
    } else {
        unreachable!()
    }
}

#[test]
fn generic_datatypes_indexed() {
    let source = "FUNCTION gen<G: ANY, X : ANY_BIT> : INT END_FUNCTION";
    let (_, index) = index(source);

    //Expecting a datatype entry for G and a datatype entry for X
    let g = index.get_type("__gen__G").unwrap();
    assert_eq!(
        g.get_type_information(),
        &DataTypeInformation::Generic {
            name: "__gen__G".into(),
            generic_symbol: "G".into(),
            nature: TypeNature::Any,
        }
    );
    let g = index.get_type("__gen__X").unwrap();
    assert_eq!(
        g.get_type_information(),
        &DataTypeInformation::Generic {
            name: "__gen__X".into(),
            generic_symbol: "X".into(),
            nature: TypeNature::Bit,
        }
    );
}

#[test]
fn function_name_equals_return_type() {
    // GIVEN function with the same name as the return type
    // WHEN the function is indexed
    let (_, index) = index(
        "
		FUNCTION TIME : TIME
		END_FUNCTION",
    );

    // THEN there should be a indexed pou_type
    let data_type = index.type_index.find_pou_type("TIME").unwrap();
    // with the name "time"
    assert_eq!(data_type.get_name(), "TIME");
    // and DataTypeInformation of the type struct
    assert!(matches!(
        data_type.get_type_information(),
        DataTypeInformation::Struct { .. }
    ));
}

#[test]
fn global_vars_for_structs() {
    // GIVEN a program with a struct variable
    // WHEN the program is indexed
    let (_, index) = index(
        "
		PROGRAM main
		VAR
			x : STRUCT var1 : INT; END_STRUCT
		END_VAR
		END_PROGRAM
		",
    );

    // THEN there should be a global variable for the struct
    let global_var = index.find_global_initializer("__main_x__init");
    assert!(global_var.is_some());
}

#[test]
fn pointer_and_in_out_pointer_should_not_conflict() {
    // GIVEN an IN-OUT INT and a POINTER TO INT
    // WHEN the program is indexed
    let (_, index) = index(
        "
		PROGRAM main
		VAR_INPUT
			x : REF_TO INT;
		END_VAR
        VAR_IN_OUT
            y : INT;
        END_VAR
		END_PROGRAM
		",
    );

    // THEN x and y whould be different pointer types
    let x = index.find_member("main", "x").expect("main.x not found");
    let x_type = index
        .get_type(x.get_type_name())
        .unwrap()
        .get_type_information();
    assert_eq!(
        x_type,
        &DataTypeInformation::Pointer {
            name: "__main_x".to_string(),
            inner_type_name: "INT".to_string(),
            auto_deref: false,
        }
    );

    let y = index.find_member("main", "y").expect("main.y not found");
    let y_type = index
        .get_type(y.get_type_name())
        .unwrap()
        .get_type_information();
    assert_eq!(
        y_type,
        &DataTypeInformation::Pointer {
            name: "auto_pointer_to_INT".to_string(),
            inner_type_name: "INT".to_string(),
            auto_deref: true,
        }
    );
}

#[test]
fn pointer_and_in_out_pointer_should_not_conflict_2() {
    // GIVEN an IN-OUT INT and a POINTER TO INT
    // AND a address-of INT operation

    // WHEN the program is indexed
    let (result, mut index) = index(
        "
		PROGRAM main
		VAR_INPUT
			x : REF_TO INT;
		END_VAR
        VAR_IN_OUT
            y : INT;
        END_VAR

        &y; //this will add another pointer_to_int type to the index (autoderef = false)
		END_PROGRAM
		",
    );

    annotate(&result, &mut index);

    // THEN x should be a normal pointer
    // AND y should be an auto-deref pointer
    let x = index.find_member("main", "x").expect("main.x not found");
    let x_type = index
        .get_type(x.get_type_name())
        .unwrap()
        .get_type_information();
    assert_eq!(
        x_type,
        &DataTypeInformation::Pointer {
            name: "__main_x".to_string(),
            inner_type_name: "INT".to_string(),
            auto_deref: false,
        }
    );

    let y = index.find_member("main", "y").expect("main.y not found");
    let y_type = index
        .get_type(y.get_type_name())
        .unwrap()
        .get_type_information();
    assert_eq!(
        y_type,
        &DataTypeInformation::Pointer {
            name: "auto_pointer_to_INT".to_string(),
            inner_type_name: "INT".to_string(),
            auto_deref: true,
        }
    );
}

#[test]
fn a_program_pou_is_indexed() {
    // GIVEN some pous
    let src = r#"
        PROGRAM myProgram
        END_PROGRAM

        FUNCTION myFunction<A: ANY_INT> : INT
        END_FUNCTION

        FUNCTION_BLOCK myFunctionBlock
        END_FUNCTION_BLOCK

        CLASS myClass
        END_CLASS

        ACTIONS myProgram
            ACTION act
            END_ACTION
        END_ACTIONS
    "#;

    // WHEN the code is indexed
    let (_, index) = index(src);

    // THEN I expect an entry for the program
    assert_eq!(
        Some(&PouIndexEntry::Program {
            name: "myProgram".into(),
            instance_struct_name: "myProgram".into(),
            linkage: LinkageType::Internal,
            instance_variable: VariableIndexEntry {
                name: "myProgram_instance".into(),
                qualified_name: "myProgram".into(),
                initial_value: None,
                variable_type: ArgumentType::ByVal(VariableType::Global),
                is_constant: false,
                data_type_name: "myProgram".into(),
                location_in_parent: 0,
                linkage: LinkageType::Internal,
                binding: None,
                source_location: SourceRange::new(9..46),
                varargs: None,
            }
        }),
        index.find_pou("myProgram"),
    );

    assert_eq!(
        Some(&PouIndexEntry::Function {
            name: "myFunction".into(),
            linkage: LinkageType::Internal,
            instance_struct_name: "myFunction".into(),
            generics: [GenericBinding {
                name: "A".into(),
                nature: TypeNature::Int
            }]
            .to_vec(),
            return_type: "INT".into(),
            is_variadic: false
        }),
        index.find_pou("myFunction"),
    );

    assert_eq!(
        Some(&PouIndexEntry::FunctionBlock {
            name: "myFunctionBlock".into(),
            linkage: LinkageType::Internal,
            instance_struct_name: "myFunctionBlock".into()
        }),
        index.find_pou("myFunctionBlock"),
    );

    assert_eq!(
        Some(&PouIndexEntry::Class {
            name: "myClass".into(),
            linkage: LinkageType::Internal,
            instance_struct_name: "myClass".into()
        }),
        index.find_pou("myClass"),
    );

    assert_eq!(
        Some(&PouIndexEntry::Action {
            name: "myProgram.act".into(),
            parent_pou_name: "myProgram".into(),
            linkage: LinkageType::Internal,
            instance_struct_name: "myProgram".into()
        }),
        index.find_pou("myProgram.act"),
    );
}
