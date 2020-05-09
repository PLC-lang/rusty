
use pretty_assertions::{assert_eq, assert_ne};
use super::visitor::visit;
use super::{Index,PouKind, VariableType};

use crate::lexer;
use crate::parser;
use inkwell::context::Context;


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
    assert_eq!("Int", entry_a.information.data_type_name);

    let entry_b = index.find_global_variable("b").unwrap();
    assert_eq!("b", entry_b.name);
    assert_eq!("Bool", entry_b.information.data_type_name);
}

#[test]
fn program_is_indexed() {
    let index = index!(r#"
        PROGRAM myProgram
        END_PROGRAM
    "#);

    let program = index.find_pou("myProgram").unwrap();
    assert_eq!(PouKind::Program, program.information.pou_kind)
}

#[test]
fn function_is_indexed() {
    let index = index!(r#"
        FUNCTION myFunction : INT
        END_FUNCTION
    "#);

    let function = index.find_pou("myFunction").unwrap();
    assert_eq!(PouKind::Function, function.information.pou_kind)
}

#[test]
fn pous_are_indexed() {
    let index = index!(r#"
        PROGRAM myProgram
        END_PROGRAM
        FUNCTION myFunction : INT
        END_FUNCTION
    "#);

    let function = index.find_pou("myFunction").unwrap();
    assert_eq!(PouKind::Function, function.information.pou_kind);
    let program = index.find_pou("myProgram").unwrap();
    assert_eq!(PouKind::Program, program.information.pou_kind);

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
    assert_eq!("Int", variable.information.data_type_name);
    assert_eq!(VariableType::Local, variable.information.variable_type);

    let variable = index.find_member("myProgram", "b").unwrap();
    assert_eq!("b", variable.name);
    assert_eq!("Int", variable.information.data_type_name);
    assert_eq!(VariableType::Local, variable.information.variable_type);

    let variable = index.find_member("myProgram", "c").unwrap();
    assert_eq!("c", variable.name);
    assert_eq!("Bool", variable.information.data_type_name);
    assert_eq!(VariableType::Input, variable.information.variable_type);

    let variable = index.find_member("myProgram", "d").unwrap();
    assert_eq!("d", variable.name);
    assert_eq!("Bool", variable.information.data_type_name);
    assert_eq!(VariableType::Input, variable.information.variable_type);
}