// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use insta::assert_debug_snapshot;
use plc_ast::ast::{
    pre_process, AstFactory, AutoDerefType, DataType, GenericBinding, LinkageType, Operator, TypeNature,
    UserTypeDeclaration,
};
use plc_ast::provider::IdProvider;
use plc_source::source_location::{SourceLocation, SourceLocationFactory};
use pretty_assertions::assert_eq;
use rustc_hash::FxHashMap;

use crate::index::{ArgumentType, PouIndexEntry, VariableIndexEntry};
use crate::parser::tests::literal_int;
use crate::test_utils::tests::{annotate_with_ids, index, index_with_ids, parse_and_preprocess};
use crate::typesystem::{InternalType, StructSource, TypeSize, INT_TYPE, VOID_TYPE};
use crate::{index::VariableType, typesystem::DataTypeInformation};

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
    let info = index.get_type("myProgram.foo").unwrap().get_type_information();
    if let crate::typesystem::DataTypeInformation::Alias { name, referenced_type } = info {
        assert_eq!("myProgram.foo", name);
        assert_eq!("myProgram", referenced_type);
    } else {
        panic!("Wrong variant : {info:#?}");
    }
    if let crate::typesystem::DataTypeInformation::Struct { name, .. } =
        index.find_effective_type_info(info.get_name()).unwrap()
    {
        assert_eq!("myProgram", name);
    } else {
        panic!("Wrong variant : {info:#?}");
    }

    let bar = index.find_implementation_by_name("myProgram.bar").unwrap();
    assert_eq!("myProgram.bar", bar.call_name);
    assert_eq!("myProgram", bar.type_name);

    let info = index.get_type("myProgram.bar").unwrap().get_type_information();
    if let crate::typesystem::DataTypeInformation::Alias { name, referenced_type } = info {
        assert_eq!("myProgram.bar", name);
        assert_eq!("myProgram", referenced_type);
    } else {
        panic!("Wrong variant : {info:#?}");
    }
    if let crate::typesystem::DataTypeInformation::Struct { name, .. } =
        index.find_effective_type_info(info.get_name()).unwrap()
    {
        assert_eq!("myProgram", name);
    } else {
        panic!("Wrong variant : {info:#?}");
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

    let foo_impl = index.find_implementation_by_name("myFuncBlock.foo").unwrap();
    assert_eq!("myFuncBlock.foo", foo_impl.call_name);
    assert_eq!("myFuncBlock.foo", foo_impl.type_name);
    let info = index.get_type("myFuncBlock.foo").unwrap().get_type_information();
    if let crate::typesystem::DataTypeInformation::Struct { name, members, .. } = info {
        assert_eq!("myFuncBlock.foo", name);
        assert_eq!(1, members.len());
        assert_eq!("x", members[0].get_name());
    } else {
        panic!("Wrong variant : {info:#?}");
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
    let info = index.get_type("myClass.foo").unwrap().get_type_information();
    if let crate::typesystem::DataTypeInformation::Struct { name, members, .. } = info {
        assert_eq!("myClass.foo", name);
        assert_eq!(1, members.len());
        assert_eq!("y", members[0].get_name());
    } else {
        panic!("Wrong variant : {info:#?}");
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
    assert_eq!(VOID_TYPE, index.get_variadic_member("myFunc").unwrap().get_type_name());
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
    assert_eq!(INT_TYPE, index.get_variadic_member("myFunc").unwrap().get_type_name());
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
    index.find_effective_type_by_name("myFunctionBlock").unwrap();
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

    let result = index.find_variable(Some("prg"), &["fb1_inst", "fb2_inst", "fb3_inst", "x"]).unwrap();
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

    assert!(index.find_callable_instance_variable(Some("prg"), &["fb1_inst"]).is_some());
    assert!(index.find_callable_instance_variable(Some("prg"), &["fb2_inst"]).is_some());
    assert!(index.find_callable_instance_variable(Some("prg"), &["fb3_inst"]).is_some());
    assert!(index.find_callable_instance_variable(Some("prg"), &["fb1_local"]).is_some());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["fb1_local", "fb2_inst", "fb3_inst"])
        .is_some());
    assert!(index.find_callable_instance_variable(Some("prg"), &["fb1_inst", "fb2_inst"]).is_some());
    assert!(index
        .find_callable_instance_variable(Some("prg"), &["fb1_inst", "fb2_inst", "fb3_inst"])
        .is_some());
    assert!(index.find_callable_instance_variable(Some("prg"), &["foo"]).is_none());
    assert!(index.find_callable_instance_variable(Some("prg"), &["a"]).is_none());
    assert!(index.find_callable_instance_variable(Some("prg"), &["b"]).is_none());
    assert!(index.find_callable_instance_variable(Some("prg"), &["c"]).is_none());
    assert!(index.find_callable_instance_variable(Some("prg"), &["d"]).is_none());
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
    insta::assert_debug_snapshot!(ast.user_types[0].data_type);

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.global_vars[0].variables[0].data_type_declaration;
    assert_debug_snapshot!(var_data_type);
    assert_eq!(src[var_data_type.get_location().to_range().unwrap()].to_string(), "(a,b,c)".to_string());

    assert_eq!(
        &"__global_inline_enum".to_string(),
        &ast.global_vars[0].variables[0].data_type_declaration.get_name().unwrap().to_string()
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
    let new_struct_type = &ast.user_types[0].data_type;
    assert_debug_snapshot!(new_struct_type);
    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.global_vars[0].variables[0].data_type_declaration;
    assert_debug_snapshot!(var_data_type);
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
    let var_data_type = &ast.pous[0].variable_blocks[0].variables[0].data_type_declaration;
    assert_debug_snapshot!(var_data_type);

    // THEN an implicit datatype should have been generated for the enum
    let new_enum_type = &ast.user_types[0].data_type;
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

    let new_struct_type = &ast.user_types[0].data_type;
    assert_debug_snapshot!(new_struct_type);

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.pous[0].variable_blocks[0].variables[0].data_type_declaration;
    assert_debug_snapshot!(var_data_type);
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
    let new_pointer_type = &ast.user_types[0];
    assert_debug_snapshot!(new_pointer_type);

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.pous[0].variable_blocks[0].variables[0].data_type_declaration;
    assert_debug_snapshot!(var_data_type);
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
    let new_pointer_type = &ast.user_types[1];
    assert_debug_snapshot!(new_pointer_type);
    // AND the original variable should now point to the new DataType
    let original = &ast.user_types[0];
    assert_debug_snapshot!(original);
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
    let new_pointer_type = &ast.user_types[0];
    assert_debug_snapshot!(new_pointer_type);

    // Pointer OF Pointer
    let new_pointer_type = &ast.user_types[1];
    assert_debug_snapshot!(new_pointer_type);

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.pous[0].variable_blocks[0].variables[0].data_type_declaration;
    assert_debug_snapshot!(var_data_type);
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
    let new_array_type = &ast.user_types[0];
    assert_debug_snapshot!(new_array_type);

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.pous[0].variable_blocks[0].variables[0].data_type_declaration;
    assert_debug_snapshot!(var_data_type);
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
    let new_array_type = &ast.user_types[0];
    assert_debug_snapshot!(new_array_type);

    // ARRAY OF ARRAY
    let new_array_type = &ast.user_types[1];
    assert_debug_snapshot!(new_array_type);

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.pous[0].variable_blocks[0].variables[0].data_type_declaration;
    assert_debug_snapshot!(var_data_type);
}

#[test]
fn pre_processing_generates_array_of_array_type() {
    // GIVEN an inline pointer is declared
    let src = r#"
        TYPE arr_arr: ARRAY[0..1] OF ARRAY[0..1] OF INT; END_TYPE
        "#;
    let (ast, ..) = parse_and_preprocess(src);

    let new_type = &ast.user_types[1];
    assert_debug_snapshot!(new_type);

    // AND the original variable should now point to the new DataType
    let original = &ast.user_types[0];
    assert_debug_snapshot!(original);
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
    let new_array_type = &ast.user_types[0];
    assert_debug_snapshot!(new_array_type);
    // ARRAY OF INT
    let new_array_type = &ast.user_types[1];
    assert_debug_snapshot!(new_array_type);
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
    let new_array_type = &ast.user_types[0];
    assert_debug_snapshot!(new_array_type);

    // ARRAY OF ARRAY
    let new_array_type = &ast.user_types[1];
    assert_debug_snapshot!(new_array_type);

    // ARRAY OF ARRAY
    let new_array_type = &ast.user_types[2];
    assert_debug_snapshot!(new_array_type);

    // AND the original variable should now point to the new DataType
    let var_data_type = &ast.pous[0].variable_blocks[0].variables[0].data_type_declaration;
    assert_debug_snapshot!(var_data_type);
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

    assert_eq!(1, ast.user_types.len());
    //A type __myFunc__G is created
    let expected = UserTypeDeclaration {
        data_type: DataType::GenericType {
            name: "__myFunc__G".into(),
            generic_symbol: "G".into(),
            nature: TypeNature::Any,
        },
        initializer: None,
        location: SourceLocation::internal(),
        scope: Some("myFunc".into()),
    };

    assert_eq!(format!("{expected:?}"), format!("{:?}", ast.user_types[0]));

    //The variables with type G now have type __myFunc__G
    let pou = &ast.pous[0];
    assert_eq!(pou.variable_blocks[0].variables[0].data_type_declaration.get_name().unwrap(), "__myFunc__G");
    assert_eq!(pou.variable_blocks[0].variables[1].data_type_declaration.get_name().unwrap(), "INT");
    assert_eq!(pou.return_type.as_ref().unwrap().get_name().unwrap(), "__myFunc__G");
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
        location: SourceLocation::internal(),
        scope: Some("myFunc".into()),
    };

    assert_eq!(format!("{expected:?}"), format!("{:?}", ast.user_types[0]));
    //Additional types created
    assert_eq!(3, ast.user_types.len());
    //referenced types of additional types are the new type
    if let DataType::ArrayType { referenced_type, .. } = &ast.user_types[1].data_type {
        assert_eq!(referenced_type.get_name().unwrap(), "__myFunc__G");
    } else {
        panic!("expected array");
    }
    if let DataType::PointerType { referenced_type, .. } = &ast.user_types[2].data_type {
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
        sub_range: TypeSize::from_literal(7)..TypeSize::from_literal(1000),
    };

    assert_eq!(format!("{expected:?}"), format!("{my_int:?}"));

    // THEN I expect the index to contain the defined range-information for the given type
    let my_int = &index.get_type("MyAliasInt").unwrap().information;
    let expected =
        &DataTypeInformation::Alias { name: "MyAliasInt".to_string(), referenced_type: "MyInt".to_string() };

    assert_eq!(format!("{expected:?}"), format!("{my_int:?}"));
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
        crate::lexer::lex_with_ids(src, ids.clone(), SourceLocationFactory::internal(src)),
        LinkageType::Internal,
        "test.st",
    );

    pre_process(&mut ast, ids);
    let index = crate::index::indexer::index(&ast);

    // THEN I expect the index to contain cosntant expressions (x+1), (y+1) and (z+1) as const expressions
    // associated with the initial values of the globals
    let variables = &ast.global_vars[0].variables;
    let initializer = index
        .find_global_variable("a")
        .and_then(|g| index.get_const_expressions().maybe_get_constant_statement(&g.initial_value));
    assert_eq!(variables[0].initializer.as_ref(), initializer);

    let initializer = index
        .find_global_variable("b")
        .and_then(|g| index.get_const_expressions().maybe_get_constant_statement(&g.initial_value));
    assert_eq!(variables[1].initializer.as_ref(), initializer);

    let initializer = index
        .find_global_variable("c")
        .and_then(|g| index.get_const_expressions().maybe_get_constant_statement(&g.initial_value));
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
        crate::lexer::lex_with_ids(src, ids.clone(), SourceLocationFactory::internal(src)),
        LinkageType::Internal,
        "test.st",
    );

    pre_process(&mut ast, ids);
    let index = crate::index::indexer::index(&ast);

    // THEN I expect the index to contain cosntant expressions (x+1), (y+1) and (z+1) as const expressions
    // associated with the initial values of the members
    let variables = &ast.pous[0].variable_blocks[0].variables;
    let initializer = index
        .find_member("prg", "a")
        .and_then(|g| index.get_const_expressions().maybe_get_constant_statement(&g.initial_value));
    assert_eq!(variables[0].initializer.as_ref(), initializer);

    let initializer = index
        .find_member("prg", "b")
        .and_then(|g| index.get_const_expressions().maybe_get_constant_statement(&g.initial_value));
    assert_eq!(variables[1].initializer.as_ref(), initializer);

    let initializer = index
        .find_member("prg", "c")
        .and_then(|g| index.get_const_expressions().maybe_get_constant_statement(&g.initial_value));
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
        crate::lexer::lex_with_ids(src, ids.clone(), SourceLocationFactory::internal(src)),
        LinkageType::Internal,
        "test.st",
    );

    pre_process(&mut ast, ids);
    let index = crate::index::indexer::index(&ast);

    // THEN I expect the index to contain cosntant expressions (7+x) as const expressions
    // associated with the initial values of the type
    let data_type = &ast.user_types[0];
    let initializer = index
        .get_type("MyInt")
        .map(|g| index.get_const_expressions().maybe_get_constant_statement(&g.initial_value))
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
        crate::lexer::lex_with_ids(src, ids.clone(), SourceLocationFactory::internal(src)),
        LinkageType::Internal,
        "test.st",
    );

    pre_process(&mut ast, ids);
    let index = crate::index::indexer::index(&ast);

    // THEN I expect the index to contain constants expressions used in the array-dimensions

    // check first dimensions 0 .. LEN-1
    let (start_0, end_0) = index
        .find_effective_type_info("MyInt")
        .map(|it| {
            if let DataTypeInformation::Array { dimensions, .. } = it {
                //return the pair (start, end)
                (
                    dimensions[0].start_offset.as_int_value(&index).unwrap(),
                    dimensions[0].end_offset.as_const_expression(&index).unwrap(),
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
            AstFactory::create_binary_expression(
                crate::parser::tests::ref_to("LEN"),
                Operator::Minus,
                crate::parser::tests::literal_int(1),
                0
            )
        ),
        format!("{end_0:#?}")
    );

    //check 2nd dimension MIN .. MAX
    let (start_1, end_1) = index
        .find_effective_type_info("MyInt")
        .map(|it| {
            if let DataTypeInformation::Array { dimensions, .. } = it {
                //return the pair (start, end)
                (
                    dimensions[1].start_offset.as_const_expression(&index).unwrap(),
                    dimensions[1].end_offset.as_const_expression(&index).unwrap(),
                )
            } else {
                unreachable!()
            }
        })
        .unwrap();

    assert_eq!(format!("{:#?}", crate::parser::tests::ref_to("MIN")), format!("{start_1:#?}"));

    assert_eq!(format!("{:#?}", crate::parser::tests::ref_to("MAX")), format!("{end_1:#?}"));
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
        crate::lexer::lex_with_ids(src, ids.clone(), SourceLocationFactory::internal(src)),
        LinkageType::Internal,
        "test.st",
    );

    pre_process(&mut ast, ids);
    let index = crate::index::indexer::index(&ast);

    // THEN I expect the index to contain constants expressions used in the string-len

    let data_type = &ast.user_types[0].data_type;
    let actual_len_expression = if let DataType::StringType { size, .. } = data_type {
        size.as_ref().unwrap()
    } else {
        unreachable!()
    };
    if let Some(DataTypeInformation::String { size: TypeSize::ConstExpression(expr), .. }) =
        index.find_effective_type_info("MyString")
    {
        assert_eq!(
            format!(
                "{:#?}",
                AstFactory::create_binary_expression(
                    actual_len_expression.clone(),
                    Operator::Plus,
                    literal_int(1),
                    actual_len_expression.get_id()
                )
            ),
            format!("{:#?}", index.get_const_expressions().get_constant_statement(expr).unwrap())
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
    assert!(matches!(data_type.get_type_information(), DataTypeInformation::Struct { .. }));
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
    let global_var = index.find_global_initializer("____main_x__init");
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
    let x_type = index.get_type(x.get_type_name()).unwrap().get_type_information();
    assert_eq!(
        x_type,
        &DataTypeInformation::Pointer {
            name: "__main_x".to_string(),
            inner_type_name: "INT".to_string(),
            auto_deref: None,
            type_safe: true,
            is_function: false,
        }
    );

    let y = index.find_member("main", "y").expect("main.y not found");
    let y_type = index.get_type(y.get_type_name()).unwrap().get_type_information();
    assert_eq!(
        y_type,
        &DataTypeInformation::Pointer {
            name: "__auto_pointer_to_INT".to_string(),
            inner_type_name: "INT".to_string(),
            auto_deref: Some(AutoDerefType::Default),
            type_safe: true,
            is_function: false,
        }
    );
}

#[test]
fn pointer_and_in_out_pointer_should_not_conflict_2() {
    // GIVEN an IN-OUT INT and a POINTER TO INT
    // AND a address-of INT operation

    // WHEN the program is indexed
    let id_provider = IdProvider::default();
    let (result, mut index) = index_with_ids(
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
        id_provider.clone(),
    );

    annotate_with_ids(&result, &mut index, id_provider);

    // THEN x should be a normal pointer
    // AND y should be an auto-deref pointer
    let x = index.find_member("main", "x").expect("main.x not found");
    let x_type = index.get_type(x.get_type_name()).unwrap().get_type_information();
    assert_eq!(
        x_type,
        &DataTypeInformation::Pointer {
            name: "__main_x".to_string(),
            inner_type_name: "INT".to_string(),
            auto_deref: None,
            type_safe: true,
            is_function: false,
        }
    );

    let y = index.find_member("main", "y").expect("main.y not found");
    let y_type = index.get_type(y.get_type_name()).unwrap().get_type_information();
    assert_eq!(
        y_type,
        &DataTypeInformation::Pointer {
            name: "__auto_pointer_to_INT".to_string(),
            inner_type_name: "INT".to_string(),
            auto_deref: Some(AutoDerefType::Default),
            type_safe: true,
            is_function: false,
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
    let source_location_factory = SourceLocationFactory::for_source(&src.into());

    // WHEN the code is indexed
    let (_, index) = index(src);

    // THEN I expect an entry for the program
    assert_eq!(
        Some(&PouIndexEntry::Program {
            name: "myProgram".into(),
            instance_struct_name: "myProgram".into(),
            linkage: LinkageType::Internal,
            location: source_location_factory.create_range(17..26),
            properties: FxHashMap::default(),

            instance_variable: Box::new(VariableIndexEntry {
                name: "myProgram_instance".into(),
                qualified_name: "myProgram".into(),
                initial_value: None,
                argument_type: ArgumentType::ByVal(VariableType::Global),
                is_constant: false,
                is_var_external: false,
                data_type_name: "myProgram".into(),
                location_in_parent: 0,
                linkage: LinkageType::Internal,
                binding: None,
                source_location: source_location_factory.create_range(17..26),
                varargs: None,
            })
        }),
        index.find_pou("myProgram"),
    );

    assert_eq!(
        Some(&PouIndexEntry::Function {
            name: "myFunction".into(),
            linkage: LinkageType::Internal,
            generics: [GenericBinding { name: "A".into(), nature: TypeNature::Int }].to_vec(),
            return_type: "INT".into(),
            is_variadic: false,
            location: source_location_factory.create_range(65..75),
            is_generated: false,
            is_const: false,
        }),
        index.find_pou("myFunction"),
    );

    assert_eq!(
        Some(&PouIndexEntry::FunctionBlock {
            name: "myFunctionBlock".into(),
            linkage: LinkageType::Internal,
            instance_struct_name: "myFunctionBlock".into(),
            location: source_location_factory.create_range(139..154),
            super_class: None,
            interfaces: vec![],
            properties: FxHashMap::default(),
        }),
        index.find_pou("myFunctionBlock"),
    );

    assert_eq!(
        Some(&PouIndexEntry::Class {
            name: "myClass".into(),
            linkage: LinkageType::Internal,
            instance_struct_name: "myClass".into(),
            location: source_location_factory.create_range(197..204),
            super_class: None,
            interfaces: vec![],
            properties: FxHashMap::default(),
        }),
        index.find_pou("myClass"),
    );

    assert_eq!(
        Some(&PouIndexEntry::Action {
            name: "myProgram.act".into(),
            parent_name: "myProgram".into(),
            linkage: LinkageType::Internal,
            instance_struct_name: "myProgram".into(),
            location: source_location_factory.create_range(269..272),
        }),
        index.find_pou("myProgram.act"),
    );
}

#[test]
fn program_parameters_variable_type() {
    // GIVEN PROGRAM with some parameters
    // WHEN the PROGRAM is indexed
    let (_, index) = index(
        "
        PROGRAM main
        VAR_INPUT
            input1 : INT;
        END_VAR
        VAR_OUTPUT
            output1 : INT;
        END_VAR
        VAR_IN_OUT
            inout1 : INT;
        END_VAR
        END_PROGRAM
        ",
    );

    // THEN the parameters should have the correct VariableType
    let members = index.get_container_members("main");
    assert_eq!(members.len(), 3);

    // INPUT => ByVal
    // OUTPUT => ByVal
    // IN_OUT => ByRef
    insta::assert_debug_snapshot!(members);
}

#[test]
fn fb_parameters_variable_type() {
    // GIVEN FB with some parameters
    // WHEN the FB is indexed
    let (_, index) = index(
        "
        FUNCTION_BLOCK fb
        VAR_INPUT
            input1 : INT;
        END_VAR
        VAR_OUTPUT
            output1 : INT;
        END_VAR
        VAR_IN_OUT
            inout1 : INT;
        END_VAR
        END_FUNCTION_BLOCK
        ",
    );

    // THEN the parameters should have the correct VariableType
    let members = index.get_container_members("fb");
    assert_eq!(members.len(), 3);

    // INPUT => ByVal
    // OUTPUT => ByVal
    // IN_OUT => ByRef
    insta::assert_debug_snapshot!(members);
}

#[test]
fn function_parameters_variable_type() {
    // GIVEN FUNCTION with some parameters
    // WHEN the FUNCTION is indexed
    let (_, index) = index(
        "
        FUNCTION foo : INT
        VAR_INPUT
            input1 : INT;
        END_VAR
        VAR_OUTPUT
            output1 : INT;
        END_VAR
        VAR_IN_OUT
            inout1 : INT;
        END_VAR
        END_FUNCTION
        ",
    );

    // THEN the parameters should have the correct VariableType
    let members = index.get_container_members("foo");
    assert_eq!(members.len(), 4);
    // 4th entry is the return type

    // INPUT => ByVal
    // OUTPUT => ByRef
    // IN_OUT => ByRef
    insta::assert_debug_snapshot!(members);
}

#[test]
fn pou_duplicates_are_indexed() {
    // GIVEN 2 POUs with the same name
    // WHEN the code is indexed
    let (_, index) = index(
        "
        PROGRAM foo
        VAR_INPUT
            input1 : INT;
        END_VAR
        END_PROGRAM

        PROGRAM foo
        VAR_INPUT
            input2 : INT;
        END_VAR
        END_PROGRAM
        ",
    );

    //THEN I expect both PouIndexEntries
    let pous = index.get_pous().values().filter(|it| it.get_name().eq("foo")).collect::<Vec<_>>();

    let foo1 = pous.first().unwrap();
    assert_eq!(foo1.get_name(), "foo");

    let foo2 = pous.get(1).unwrap();
    assert_eq!(foo2.get_name(), "foo");
}

#[test]
fn type_duplicates_are_indexed() {
    // GIVEN 3 types with the same name
    // WHEN the code is indexed
    let (_, index) = index(
        "
        TYPE MyStruct:
        STRUCT
          field1 : INT;
        END_STRUCT
        END_TYPE

        TYPE MyStruct:
        STRUCT
          field2 : INT;
        END_STRUCT
        END_TYPE

        TYPE MyStruct:
        STRUCT
          field3 : INT;
        END_STRUCT
        END_TYPE
        ",
    );

    //THEN I expect all 3 DataTypes
    let types = index.get_types().values().filter(|it| it.get_name().eq("MyStruct")).collect::<Vec<_>>();

    let mystruct1 = types.first().unwrap();
    assert_eq!(mystruct1.get_name(), "MyStruct");

    let mystruct2 = types.get(1).unwrap();
    assert_eq!(mystruct2.get_name(), "MyStruct");

    let mystruct3 = types.get(2).unwrap();
    assert_eq!(mystruct3.get_name(), "MyStruct");
}

#[test]
fn global_variables_duplicates_are_indexed() {
    // GIVEN 2 global variables with the same name
    // WHEN the code is indexed
    let (_, index) = index(
        "
            VAR_GLOBAL
                x : INT;
            END_VAR

            VAR_GLOBAL
                x : BOOL;
            END_VAR
        ",
    );

    //THEN I expect both globals
    let globals = index.get_globals().values().filter(|it| it.get_name().eq("x")).collect::<Vec<_>>();

    let x1 = globals.first().unwrap();
    assert_eq!(x1.get_name(), "x");
    assert_eq!(x1.get_type_name(), "INT");

    let x2 = globals.get(1).unwrap();
    assert_eq!(x2.get_name(), "x");
    assert_eq!(x2.get_type_name(), "BOOL");
}

#[test]
fn internal_vla_struct_type_is_indexed_correctly() {
    let id_provider = IdProvider::default();
    let (_, index) = index_with_ids(
        r"
        FUNCTION foo : DINT
        VAR_INPUT
            arr: ARRAY[*] OF INT;
        END_VAR
        END_FUNCTION
    ",
        id_provider,
    );
    assert_eq!(
        *index.get_type("__foo_arr").unwrap().get_type_information(),
        DataTypeInformation::Struct {
            name: "__foo_arr".to_string(),
            members: vec![
                VariableIndexEntry {
                    name: "struct_vla_int_1".to_string(),
                    qualified_name: "__foo_arr.struct_vla_int_1".to_string(),
                    initial_value: None,
                    argument_type: ArgumentType::ByVal(VariableType::Input),
                    is_constant: false,
                    is_var_external: false,
                    data_type_name: "__ptr_to___foo_arr_vla_1_int".to_string(),
                    location_in_parent: 0,
                    linkage: LinkageType::Internal,
                    binding: None,
                    source_location: SourceLocation::internal(),
                    varargs: None
                },
                VariableIndexEntry {
                    name: "dimensions".to_string(),
                    qualified_name: "__foo_arr.dimensions".to_string(),
                    initial_value: None,
                    argument_type: ArgumentType::ByVal(VariableType::Input),
                    is_constant: false,
                    is_var_external: false,
                    data_type_name: "__bounds___foo_arr_vla_1_int".to_string(),
                    location_in_parent: 1,
                    linkage: LinkageType::Internal,
                    binding: None,
                    source_location: SourceLocation::internal(),
                    varargs: None
                }
            ],
            source: StructSource::Internal(InternalType::VariableLengthArray {
                inner_type_name: "INT".to_string(),
                ndims: 1
            })
        }
    );
}

#[test]
fn string_type_alias_without_size_is_indexed() {
    let (_, index) = index(
        r"
            TYPE MyString : STRING; END_TYPE
            TYPE MyWideString: WSTRING; END_TYPE
        ",
    );

    let my_alias = index.get_type("MyString").unwrap();
    assert_eq!("MyString", my_alias.get_name());

    let my_alias = "MyString";
    let dt = index.find_effective_type_by_name(my_alias).unwrap();
    assert_eq!("STRING", dt.get_name());

    let my_alias = index.get_type("MyWideString").unwrap();
    assert_eq!("MyWideString", my_alias.get_name());

    let my_alias = "MyWideString";
    let dt = index.find_effective_type_by_name(my_alias).unwrap();
    assert_eq!("WSTRING", dt.get_name());
}

#[test]
fn aliased_hardware_access_variable_has_implicit_initial_value_declaration() {
    // Given some aliased hardware access variable like `foo AT %IX1.2.3.4 : BOOL` we expect the index to have
    // two variables: (1) a pointer variable named foo and (2) an internally created global variable named
    // `1.2.3.4` of type BOOL that is being pointed at by (1)
    let (_, index) = index(
        r"
            VAR_GLOBAL
            foo AT %IX1.2.3.4 : BOOL;
            END_VAR
        ",
    );

    // Although foo has no initial value in its declaration, we inject one in the pre-processor
    assert_debug_snapshot!(index.find_global_variable("foo").unwrap(), @r#"
    VariableIndexEntry {
        name: "foo",
        qualified_name: "foo",
        initial_value: Some(
            Index {
                index: 0,
                generation: 0,
            },
        ),
        argument_type: ByVal(
            Global,
        ),
        is_constant: false,
        is_var_external: false,
        data_type_name: "__global_foo",
        location_in_parent: 0,
        linkage: Internal,
        binding: Some(
            HardwareBinding {
                direction: Input,
                access: Bit,
                entries: [
                    Index {
                        index: 1,
                        generation: 0,
                    },
                    Index {
                        index: 2,
                        generation: 0,
                    },
                    Index {
                        index: 3,
                        generation: 0,
                    },
                    Index {
                        index: 4,
                        generation: 0,
                    },
                ],
                location: SourceLocation {
                    span: Range(2:16 - 2:29),
                    file: Some(
                        "<internal>",
                    ),
                },
            },
        ),
        source_location: SourceLocation {
            span: Range(2:12 - 2:15),
            file: Some(
                "<internal>",
            ),
        },
        varargs: None,
    }
    "#);
}

#[test]
fn aliased_hardware_access_variable_creates_global_var_for_address() {
    // Given some aliased hardware access variable like `foo AT %IX1.2.3.4 : BOOL` we expect the index to have
    // two variables: (1) a pointer variable named foo and (2) an internally created global variable named
    // `1.2.3.4` of type BOOL that is being pointed at by (1)
    let (_, index) = index(
        r"
            VAR_GLOBAL
            foo AT %IX1.2.3.4 : BOOL;
            END_VAR
        ",
    );

    assert_debug_snapshot!(index.find_global_variable("__PI_1_2_3_4").unwrap(), @r#"
    VariableIndexEntry {
        name: "__PI_1_2_3_4",
        qualified_name: "__PI_1_2_3_4",
        initial_value: None,
        argument_type: ByVal(
            Global,
        ),
        is_constant: false,
        is_var_external: false,
        data_type_name: "BOOL",
        location_in_parent: 0,
        linkage: Internal,
        binding: None,
        source_location: SourceLocation {
            span: Range(2:16 - 2:29),
            file: Some(
                "<internal>",
            ),
        },
        varargs: None,
    }
    "#);

    assert_debug_snapshot!(index.find_type("__global_foo"), @r#"
    Some(
        DataType {
            name: "__global_foo",
            initial_value: None,
            information: Pointer {
                name: "__global_foo",
                inner_type_name: "BOOL",
                auto_deref: Some(
                    Alias,
                ),
                type_safe: true,
                is_function: false,
            },
            nature: Any,
            location: SourceLocation {
                span: Range(2:30 - 2:36),
                file: Some(
                    "<internal>",
                ),
            },
        },
    )
    "#);
}

#[test]
fn aliased_hardware_access_variable_is_initialized_with_the_address_as_ref() {
    // Given some aliased hardware access variable like `foo AT %IX1.2.3.4 : BOOL` we expect the index to have
    // two variables: (1) a pointer variable named foo and (2) an internally created global variable named
    // `1.2.3.4` of type BOOL that is being pointed at by (1)
    let (_, index) = index(
        r"
            VAR_GLOBAL
            foo AT %IX1.2.3.4 : BOOL;
            END_VAR
        ",
    );

    let foo = index.find_global_variable("foo").unwrap();
    let foo_init_id = foo.initial_value.unwrap();

    // ...the injected initial value is simply the internally created global mangled variabled
    assert_debug_snapshot!(index.get_const_expressions().get_constant_statement(&foo_init_id), @r#"
    Some(
        ReferenceExpr {
            kind: Member(
                Identifier {
                    name: "__PI_1_2_3_4",
                },
            ),
            base: None,
        },
    )
    "#);
}

#[test]
fn aliased_hardware_access_variable_is_indexed_as_a_pointer() {
    // Given some aliased hardware access variable like `foo AT %IX1.2.3.4 : BOOL` we expect the index to have
    // two variables: (1) a pointer variable named foo and (2) an internally created global variable named
    // `1.2.3.4` of type BOOL that is being pointed at by (1)
    let (_, index) = index(
        r"
            VAR_GLOBAL
            foo AT %IX1.2.3.4 : BOOL;
            END_VAR
        ",
    );

    assert_debug_snapshot!(index.find_type("__global_foo"), @r#"
    Some(
        DataType {
            name: "__global_foo",
            initial_value: None,
            information: Pointer {
                name: "__global_foo",
                inner_type_name: "BOOL",
                auto_deref: Some(
                    Alias,
                ),
                type_safe: true,
                is_function: false,
            },
            nature: Any,
            location: SourceLocation {
                span: Range(2:30 - 2:36),
                file: Some(
                    "<internal>",
                ),
            },
        },
    )
    "#);
}

#[test]
fn address_used_in_2_aliases_only_created_once() {
    // Given two aliased hardware access variable like `foo AT %IX1.2.3.4 : BOOL` we expect the index to have
    // two variables: (1) a pointer variable named foo and (2) an internally created global variable named
    // `__I_1.2.3.4` of type BOOL that is being pointed at by (1)
    let (_, index) = index(
        r"
            VAR_GLOBAL
            foo AT %IX1.2.3.4 : BOOL;
            baz AT %IX1.2.3.4 : BOOL;
            END_VAR
        ",
    );

    assert_debug_snapshot!(index.get_globals().get("__pi_1_2_3_4"), @r#"
    Some(
        VariableIndexEntry {
            name: "__PI_1_2_3_4",
            qualified_name: "__PI_1_2_3_4",
            initial_value: None,
            argument_type: ByVal(
                Global,
            ),
            is_constant: false,
            is_var_external: false,
            data_type_name: "BOOL",
            location_in_parent: 0,
            linkage: Internal,
            binding: None,
            source_location: SourceLocation {
                span: Range(2:16 - 2:29),
                file: Some(
                    "<internal>",
                ),
            },
            varargs: None,
        },
    )
    "#);
}

#[test]
fn aliased_variable_with_in_or_out_directions_create_the_same_variable() {
    // Given some aliased hardware access variable like `foo AT %IX1.2.3.4 : BOOL` we expect the index to have
    // two variables: (1) a pointer variable named foo and (2) an internally created global variable named
    // `1.2.3.4` of type BOOL that is being pointed at by (1)
    let (_, index) = index(
        r"
            VAR_GLOBAL
            foo AT %IX1.2.3.4 : BOOL;
            bar AT %QX1.2.3.4 : BOOL;
            foo1 AT %IW1.2.3.5 : WORD;
            bar2 AT %QW1.2.3.5 : WORD;
            END_VAR
        ",
    );

    assert_debug_snapshot!(index.get_globals().get("__pi_1_2_3_4"), @r#"
    Some(
        VariableIndexEntry {
            name: "__PI_1_2_3_4",
            qualified_name: "__PI_1_2_3_4",
            initial_value: None,
            argument_type: ByVal(
                Global,
            ),
            is_constant: false,
            is_var_external: false,
            data_type_name: "BOOL",
            location_in_parent: 0,
            linkage: Internal,
            binding: None,
            source_location: SourceLocation {
                span: Range(2:16 - 2:29),
                file: Some(
                    "<internal>",
                ),
            },
            varargs: None,
        },
    )
    "#);
    assert_debug_snapshot!(index.get_globals().get("__pi_1_2_3_5"), @r#"
    Some(
        VariableIndexEntry {
            name: "__PI_1_2_3_5",
            qualified_name: "__PI_1_2_3_5",
            initial_value: None,
            argument_type: ByVal(
                Global,
            ),
            is_constant: false,
            is_var_external: false,
            data_type_name: "WORD",
            location_in_parent: 0,
            linkage: Internal,
            binding: None,
            source_location: SourceLocation {
                span: Range(4:17 - 4:30),
                file: Some(
                    "<internal>",
                ),
            },
            varargs: None,
        },
    )
    "#);
}

#[test]
fn if_two_aliased_var_of_different_types_use_the_same_address_the_first_wins() {
    // Given some aliased hardware access variable like `foo AT %IX1.2.3.4 : BOOL` we expect the index to have
    // two variables: (1) a pointer variable named foo and (2) an internally created global variable named
    // `1.2.3.4` of type BOOL that is being pointed at by (1)
    let (_, index) = index(
        r"
            VAR_GLOBAL
            foo AT %IX1.2.3.4 : BOOL;
            foo AT %ID1.2.3.4 : DWORD;
            END_VAR
        ",
    );

    assert_debug_snapshot!(index.get_globals().get("__pi_1_2_3_4"), @r#"
    Some(
        VariableIndexEntry {
            name: "__PI_1_2_3_4",
            qualified_name: "__PI_1_2_3_4",
            initial_value: None,
            argument_type: ByVal(
                Global,
            ),
            is_constant: false,
            is_var_external: false,
            data_type_name: "BOOL",
            location_in_parent: 0,
            linkage: Internal,
            binding: None,
            source_location: SourceLocation {
                span: Range(2:16 - 2:29),
                file: Some(
                    "<internal>",
                ),
            },
            varargs: None,
        },
    )
    "#);
}

#[test]
fn var_config_hardware_address_creates_global_variable() {
    // Given some configured hardware access variable like `foo.bar AT %IX1.2.3.4 : BOOL` we expect the index to have
    // an internally created global variable named `__PI_1.2.3.4` of type BOOL.
    let (_, index) = index(
        r"
            VAR_CONFIG
                foo.bar AT %IX1.2.3.4 : BOOL;
            END_VAR
        ",
    );

    assert_debug_snapshot!(index.find_global_variable("__PI_1_2_3_4").unwrap(), @r#"
    VariableIndexEntry {
        name: "__PI_1_2_3_4",
        qualified_name: "__PI_1_2_3_4",
        initial_value: None,
        argument_type: ByVal(
            Global,
        ),
        is_constant: false,
        is_var_external: false,
        data_type_name: "BOOL",
        location_in_parent: 0,
        linkage: Internal,
        binding: None,
        source_location: SourceLocation {
            span: Range(2:24 - 2:37),
            file: Some(
                "<internal>",
            ),
        },
        varargs: None,
    }
    "#);
}

#[test]
fn var_externals_are_distinctly_indexed() {
    let (_, index) = index(
        "
            VAR_GLOBAL
                arr: ARRAY [0..100] OF INT;
            END_VAR

            FUNCTION foo
            VAR_EXTERNAL
                arr : ARRAY [0..100] OF INT;
            END_VAR
            END_FUNCTION
        ",
    );

    let external = &index.get_pou_members("foo")[0];
    let global = index.get_globals().get("arr").expect("global 'arr' must exist");
    assert!(external.is_var_external());
    assert_eq!(external.get_name(), global.get_name());
    assert_eq!(external.get_variable_type(), VariableType::External);
    assert_ne!(external, global);
}

#[test]
fn var_externals_constants_are_both_flagged_as_external_and_constant() {
    let (_, index) = index(
        "
            VAR_GLOBAL
                arr: ARRAY [0..100] OF INT;
            END_VAR

            FUNCTION foo
            VAR_EXTERNAL CONSTANT
                arr : ARRAY [0..100] OF INT;
            END_VAR
            END_FUNCTION
        ",
    );

    let external = &index.get_pou_members("foo")[0];
    assert!(external.is_var_external() && external.is_constant());
}

#[test]
fn inheritance_chain_correctly_finds_parents() {
    let (_, index) = index(
        "
        FUNCTION_BLOCK grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK parent EXTENDS grandparent
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK child EXTENDS parent
        END_FUNCTION_BLOCK
        ",
    );

    let inheritance_chain = index.get_inheritance_chain("child", "child");
    assert_eq!(inheritance_chain, vec![index.find_pou("child").unwrap()]);
    let inheritance_chain = index.get_inheritance_chain("child", "parent");
    assert_eq!(inheritance_chain, vec![index.find_pou("parent").unwrap(), index.find_pou("child").unwrap()]);
    let inheritance_chain = index.get_inheritance_chain("child", "grandparent");
    assert_eq!(
        inheritance_chain,
        vec![
            index.find_pou("grandparent").unwrap(),
            index.find_pou("parent").unwrap(),
            index.find_pou("child").unwrap()
        ]
    );
    let inheritance_chain = index.get_inheritance_chain("parent", "child");
    assert_eq!(inheritance_chain, Vec::<&PouIndexEntry>::new());
    let inheritance_chain = index.get_inheritance_chain("grandparent", "parent");
    assert_eq!(inheritance_chain, Vec::<&PouIndexEntry>::new());
    let inheritance_chain = index.get_inheritance_chain("grandparent", "child");
    assert_eq!(inheritance_chain, Vec::<&PouIndexEntry>::new());
}

#[test]
fn pou_with_two_types_not_considered_recursive() {
    let (_, index) = index(
        "
        FUNCTION_BLOCK fb
        VAR x : DINT; END_VAR
        END_FUNCTION_BLOCK
        PROGRAM p
        VAR
            x : fb;
            y : fb;
        END_VAR
            METHOD x : fb
            END_METHOD
        END_PROGRAM

        ACTION p.y
        END_ACTION",
    );

    let pou_type = index.find_pou_type("p").unwrap();
    assert_eq!(pou_type.get_type_information().get_size(&index).unwrap().bits(), 64);

    assert!(index.find_local_member("p", "x").is_some());
    assert!(index.find_local_member("p", "y").is_some());
}

#[test]
fn pou_with_recursive_type_fails() {
    let (_, index) = index(
        "
        FUNCTION_BLOCK fb
        VAR x : fb; END_VAR
        END_FUNCTION_BLOCK
        ",
    );

    let pou_type = index.find_pou_type("fb").unwrap();
    assert!(pou_type.get_type_information().get_size(&index).is_err());
}

#[test]
fn fixed_order() {
    let (_, index) = index(
        r#"
        FUNCTION_BLOCK A
            METHOD foo
            END_METHOD

            METHOD bar
            END_METHOD

            METHOD baz
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK B EXTENDS A
            METHOD bar
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK C EXTENDS A
            METHOD qux
            END_METHOD

            METHOD foo
            END_METHOD

            METHOD baz
            END_METHOD

            METHOD whateverComesAfterQux
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK D EXTENDS C
            METHOD idk
            END_METHOD

            METHOD baz
            END_METHOD

            METHOD qux
            END_METHOD
        END_FUNCTION_BLOCK
        "#,
    );

    let methods_a = index.get_methods_in_fixed_order("A").iter().map(|p| p.get_name()).collect::<Vec<_>>();
    assert_eq!(methods_a, vec!["A.foo", "A.bar", "A.baz"]);

    let methods_b = index.get_methods_in_fixed_order("B").iter().map(|p| p.get_name()).collect::<Vec<_>>();
    assert_eq!(methods_b, vec!["A.foo", "B.bar", "A.baz"]);

    let methods_c = index.get_methods_in_fixed_order("C").iter().map(|p| p.get_name()).collect::<Vec<_>>();
    assert_eq!(methods_c, vec!["C.foo", "A.bar", "C.baz", "C.qux", "C.whateverComesAfterQux"]);

    let methods_d = index.get_methods_in_fixed_order("D").iter().map(|p| p.get_name()).collect::<Vec<_>>();
    assert_eq!(methods_d, vec!["C.foo", "A.bar", "D.baz", "D.qux", "C.whateverComesAfterQux", "D.idk"]);
}

#[test]
fn enum_ensure_standard_variable_can_be_assigned_in_function_block() {
    let (_, index) = index(
        r#"
        TYPE EnumType : (
                red,
                green,
                blue
            );
        END_TYPE

        FUNCTION_BLOCK fb
        VAR
            myVar	: EnumType;
        END_VAR
        myVar := red;
        END_FUNCTION_BLOCK
        "#,
    );

    let pou_type = index.find_pou_type("fb").unwrap();
    assert!(pou_type.get_type_information().get_size(&index).is_ok());

    assert!(index.find_local_member("fb", "myVar").is_some());

    // Evaluate the enum reference
    assert!(index.find_local_member("fb", "red").is_some());
    assert_eq!(index.find_local_member("fb", "red").unwrap().data_type_name, "EnumType");
}

#[test]
fn enum_ensure_output_variable_can_be_assigned_in_function_block() {
    let (_, index) = index(
        r#"
        TYPE EnumType : (
                red,
                green,
                blue
            );
        END_TYPE

        FUNCTION_BLOCK fb
        VAR_OUTPUT
            outVar	: EnumType;
        END_VAR
        outVar := green;
        END_FUNCTION_BLOCK
        "#,
    );

    let pou_type = index.find_pou_type("fb").unwrap();
    assert!(pou_type.get_type_information().get_size(&index).is_ok());

    assert!(index.find_local_member("fb", "outVar").is_some());

    // Evaluate the enum reference
    assert!(index.find_local_member("fb", "green").is_some());
    assert_eq!(index.find_local_member("fb", "green").unwrap().data_type_name, "EnumType");
}

#[test]
fn enum_ensure_in_out_variable_can_be_assigned_in_function_block() {
    let (_, index) = index(
        r#"
        TYPE EnumType : (
                red,
                green,
                blue
            );
        END_TYPE

        FUNCTION_BLOCK fb
        VAR_IN_OUT
            inOutVar	: EnumType;
        END_VAR
        inOutVar := blue;
        END_FUNCTION_BLOCK
        "#,
    );

    let pou_type = index.find_pou_type("fb").unwrap();
    assert!(pou_type.get_type_information().get_size(&index).is_ok());

    assert!(index.find_local_member("fb", "inOutVar").is_some());

    // Evaluate the enum reference
    assert!(index.find_local_member("fb", "blue").is_some());
    assert_eq!(index.find_local_member("fb", "blue").unwrap().data_type_name, "EnumType");
}

#[test]
fn enum_ensure_a_combination_of_variables_can_be_assigned_in_function_block() {
    let (_, index) = index(
        r#"
        TYPE EnumType : (
                red,
                green,
                blue
            );
        END_TYPE

        FUNCTION_BLOCK fb
        VAR
            myVar	: EnumType;
        END_VAR
        VAR_OUTPUT
            outVar	: EnumType;
        END_VAR
        VAR_IN_OUT
            inOutVar	: EnumType;
        END_VAR
        myVar := red;
        outVar := green;
        inOutVar := blue;
        END_FUNCTION_BLOCK
        "#,
    );

    let pou_type = index.find_pou_type("fb").unwrap();
    assert!(pou_type.get_type_information().get_size(&index).is_ok());

    assert!(index.find_local_member("fb", "myVar").is_some());
    assert!(index.find_local_member("fb", "outVar").is_some());
    assert!(index.find_local_member("fb", "inOutVar").is_some());

    // Evaluate the enum reference
    assert!(index.find_local_member("fb", "red").is_some());
    assert_eq!(index.find_local_member("fb", "red").unwrap().data_type_name, "EnumType");
    assert!(index.find_local_member("fb", "green").is_some());
    assert_eq!(index.find_local_member("fb", "green").unwrap().data_type_name, "EnumType");
    assert!(index.find_local_member("fb", "blue").is_some());
    assert_eq!(index.find_local_member("fb", "blue").unwrap().data_type_name, "EnumType");
}

#[test]
fn enum_ensure_standard_variable_can_be_assigned_in_function() {
    let (_, index) = index(
        r#"
        TYPE EnumType : (
                red,
                green,
                blue
            );
        END_TYPE

        FUNCTION fn : INT
        VAR
            myVar	: EnumType;
        END_VAR
        myVar := red;
        END_FUNCTION
        "#,
    );

    let pou_type = index.find_pou_type("fn").unwrap();
    assert!(pou_type.get_type_information().get_size(&index).is_ok());

    assert!(index.find_local_member("fn", "myVar").is_some());

    // Evaluate the enum reference
    assert!(index.find_local_member("fn", "red").is_some());
    assert_eq!(index.find_local_member("fn", "red").unwrap().data_type_name, "EnumType");
}

#[test]
fn enum_ensure_output_variable_can_be_assigned_in_function() {
    let (_, index) = index(
        r#"
        TYPE EnumType : (
                red,
                green,
                blue
            );
        END_TYPE

        FUNCTION fn : INT
        VAR_OUTPUT
            outVar	: EnumType;
        END_VAR
        outVar := green;
        END_FUNCTION
        "#,
    );

    let pou_type = index.find_pou_type("fn").unwrap();
    assert!(pou_type.get_type_information().get_size(&index).is_ok());

    assert!(index.find_local_member("fn", "outVar").is_some());

    // Evaluate the enum reference
    assert!(index.find_local_member("fn", "green").is_some());
    assert_eq!(index.find_local_member("fn", "green").unwrap().data_type_name, "EnumType");
}

#[test]
fn enum_ensure_in_out_variable_can_be_assigned_in_function() {
    let (_, index) = index(
        r#"
        TYPE EnumType : (
                red,
                green,
                blue
            );
        END_TYPE

        FUNCTION fn : INT
        VAR_IN_OUT
            inOutVar	: EnumType;
        END_VAR
        inOutVar := blue;
        END_FUNCTION
        "#,
    );

    let pou_type = index.find_pou_type("fn").unwrap();
    assert!(pou_type.get_type_information().get_size(&index).is_ok());

    assert!(index.find_local_member("fn", "inOutVar").is_some());

    // Evaluate the enum reference
    assert!(index.find_local_member("fn", "blue").is_some());
    assert_eq!(index.find_local_member("fn", "blue").unwrap().data_type_name, "EnumType");
}

#[test]
fn enum_ensure_a_combination_of_variables_can_be_assigned_in_function() {
    let (_, index) = index(
        r#"
        TYPE EnumType : (
                red,
                green,
                blue
            );
        END_TYPE

        FUNCTION fn : INT
        VAR
            myVar	: EnumType;
        END_VAR
        VAR_OUTPUT
            outVar	: EnumType;
        END_VAR
        VAR_IN_OUT
            inOutVar	: EnumType;
        END_VAR
        myVar := red;
        outVar := green;
        inOutVar := blue;
        END_FUNCTION
        "#,
    );

    let pou_type = index.find_pou_type("fn").unwrap();
    assert!(pou_type.get_type_information().get_size(&index).is_ok());

    assert!(index.find_local_member("fn", "myVar").is_some());
    assert!(index.find_local_member("fn", "outVar").is_some());
    assert!(index.find_local_member("fn", "inOutVar").is_some());

    // Evaluate the enum reference
    assert!(index.find_local_member("fn", "red").is_some());
    assert_eq!(index.find_local_member("fn", "red").unwrap().data_type_name, "EnumType");
    assert!(index.find_local_member("fn", "green").is_some());
    assert_eq!(index.find_local_member("fn", "green").unwrap().data_type_name, "EnumType");
    assert!(index.find_local_member("fn", "blue").is_some());
    assert_eq!(index.find_local_member("fn", "blue").unwrap().data_type_name, "EnumType");
}

#[test]
fn declared_parameters() {
    let (_, index) = index(
        r#"
        FUNCTION_BLOCK FbA
            VAR
                localA: DINT;
            END_VAR

            VAR_INPUT
                inA: DINT;
            END_VAR

            VAR_OUTPUT
                outA: DINT;
            END_VAR

            VAR_IN_OUT
                inoutA: DINT;
            END_VAR

            METHOD methA
            END_METHOD
        END_FUNCTION_BLOCK

        FUNCTION_BLOCK FbB EXTENDS FbA
            VAR
                localB: DINT;
            END_VAR

            VAR_INPUT
                inB: DINT;
            END_VAR

            VAR_OUTPUT
                outB: DINT;
            END_VAR

            VAR_IN_OUT
                inoutB: DINT;
            END_VAR

            METHOD methB
                VAR_INPUT
                    inB_meth: DINT;
                END_VAR
            END_METHOD
        END_FUNCTION_BLOCK
    "#,
    );

    let members = index.get_available_parameters("FbA").iter().map(|var| &var.name).collect::<Vec<_>>();
    assert_eq!(members, vec!["inA", "outA", "inoutA"]);

    let members = index.get_available_parameters("FbB").iter().map(|var| &var.name).collect::<Vec<_>>();
    assert_eq!(members, vec!["inA", "outA", "inoutA", "inB", "outB", "inoutB",]);

    let members = index.get_available_parameters("methA").iter().map(|var| &var.name).collect::<Vec<_>>();
    assert!(members.is_empty());

    let members = index.get_available_parameters("FbB.methB").iter().map(|var| &var.name).collect::<Vec<_>>();
    assert_eq!(members, vec!["inB_meth"]);
}
