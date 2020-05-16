
use pretty_assertions::{assert_eq};
use super::visitor::visit;
use super::{Index, VariableType};

use crate::lexer;
use crate::parser;


macro_rules! index {
    ($code:tt) => {{
        let lexer = lexer::lex($code);
        let ast = parser::parse(lexer).unwrap();


        let mut index = Index::new();
        visit(&mut index, &ast);
        index
    }};
}

#[test]
fn global_variables_are_indexed() {
    let index = index!(r#"
        VAR_GLOBAL
            a: INT;
            b: BOOL;
        END_VAR
    "#);

    let entry_a = index.find_global_variable("a").unwrap();
    assert_eq!("a", entry_a.name);
    assert_eq!("INT", entry_a.information.data_type_name);

    let entry_b = index.find_global_variable("b").unwrap();
    assert_eq!("b", entry_b.name);
    assert_eq!("BOOL", entry_b.information.data_type_name);
}

#[test]
fn program_is_indexed() {
    let index = index!(r#"
        PROGRAM myProgram
        END_PROGRAM
    "#);

    index.find_type("myProgram").unwrap();
    let program_variable = index.find_global_variable("myProgram").unwrap();

    //TODO: type name should refer to my
    assert_eq!("myProgram",program_variable.information.data_type_name);
}

#[test]
fn function_is_indexed() {
    let index = index!(r#"
        FUNCTION myFunction : INT
        END_FUNCTION
    "#);

    index.find_type("myFunction").unwrap();

    let return_variable = index.find_member("myFunction", "myFunction").unwrap();
    assert_eq!("myFunction", return_variable.name);
    assert_eq!(Some("myFunction".to_string()), return_variable.information.qualifier);
    assert_eq!("INT", return_variable.information.data_type_name);
    assert_eq!(VariableType::Return, return_variable.information.variable_type);
}

#[test]
fn pous_are_indexed() {
    let index = index!(r#"
        PROGRAM myProgram
        END_PROGRAM
        FUNCTION myFunction : INT
        END_FUNCTION
    "#);

    index.find_type("myFunction").unwrap();
    index.find_type("myProgram").unwrap();

}


#[test]
fn program_members_are_indexed() {
    let index = index!(r#"
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
    "#);


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
    let result = index.find_variable(None, "a").unwrap();
    assert_eq!(VariableType::Global,result.information.variable_type);
    assert_eq!("a",result.name);
    assert_eq!(None, result.information.qualifier);
    //Asking for a variable with the POU  context finds a local variable
    let result = index.find_variable(Some("prg"),"a").unwrap();
    assert_eq!(VariableType::Local,result.information.variable_type);
    assert_eq!("a",result.name);
    assert_eq!(Some("prg".to_string()),result.information.qualifier);
    //Asking for a variable with th POU context finds a global variable
    let result = index.find_variable(Some("prg"),"b").unwrap();
    assert_eq!(VariableType::Global,result.information.variable_type);
    assert_eq!("b",result.name);
    assert_eq!(None, result.information.qualifier);
    //Asking for a variable with the function context finds the local variable
    let result= index.find_variable(Some("foo"),"a").unwrap();
    assert_eq!(VariableType::Local,result.information.variable_type);
    assert_eq!("a",result.name);
    assert_eq!(Some("foo".to_string()),result.information.qualifier);
    //Asking for a variable with the function context finds the global variable
    let result = index.find_variable(Some("foo"),"x").unwrap();
    assert_eq!(VariableType::Global,result.information.variable_type);
    assert_eq!("x",result.name);
    assert_eq!(None,result.information.qualifier);
}

