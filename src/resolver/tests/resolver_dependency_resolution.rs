use plc_ast::lib_sourcelocation::SourceCode;
use plc_ast::provider::IdProvider;

use crate::{
    resolver::{Dependency, TypeAnnotator},
    test_utils::tests::index_with_ids,
};

#[test]
fn primitive_datatypes_resolved() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        PROGRAM prog
            VAR
                x : DINT;
            END_VAR
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert_eq!(dependencies.len(), 2);
}

#[test]
fn implicit_primitive_datatypes_resolved() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        PROGRAM prog
            VAR
                x : USINT;
                y : LWORD;
            END_VAR
            x := 10 + 5; //the addition results in a DINT which is now a dependency for prog
            y := &x; //Pointer type auto generated
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("USINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("LWORD".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__POINTER_TO_USINT".into())));
    assert_eq!(dependencies.len(), 5);
}

#[test]
fn aggregate_type_resolved() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        TYPE myStruct : STRUCT
            x : REF_TO DINT;
            y : ARRAY[0..1] OF REAL;
        END_STRUCT
        END_TYPE
        PROGRAM prog
            VAR
                x : ARRAY[0..1] OF myStruct;
            END_VAR
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__prog_x".into())));
    assert!(dependencies.contains(&Dependency::Datatype("myStruct".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__myStruct_x".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__myStruct_y".into())));
    assert!(dependencies.contains(&Dependency::Datatype("REAL".into())));
    assert_eq!(dependencies.len(), 7);
}

#[test]
fn recursive_types_resolved() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "
        TYPE myStruct : STRUCT
            x : REF_TO DINT;
            y : REF_TO myStruct;
        END_STRUCT
        END_TYPE
        PROGRAM prog
            VAR
                x : myStruct;
            END_VAR
        END_PROGRAM
        ",
        id_provider.clone(),
    );

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("myStruct".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__myStruct_x".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__myStruct_y".into())));
    assert_eq!(dependencies.len(), 5);
}

#[test]
fn multiple_units_aggregate_resolved() {
    let units = [
        "
        TYPE myStruct : STRUCT
            x : DINT;
            z : REF_TO INT;
        END_STRUCT
        END_TYPE
        ",
        "
        PROGRAM prog
            VAR
                x : myStruct;
            END_VAR
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (unit1, index1) = index_with_ids(units[0], id_provider.clone());
    let (unit2, index2) = index_with_ids(units[1], id_provider.clone());
    let mut index = index1;
    index.import(index2);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit1, id_provider.clone());
    assert!(dependencies.contains(&Dependency::Datatype("__myStruct_z".into())));
    assert!(dependencies.contains(&Dependency::Datatype("INT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("myStruct".into())));
    assert_eq!(dependencies.len(), 4);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit2, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Datatype("myStruct".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__myStruct_z".into())));
    assert!(dependencies.contains(&Dependency::Datatype("INT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert_eq!(dependencies.len(), 5);
}

#[test]
fn multiple_units_aggregate_resolved_recursive() {
    let units = [
        "
        TYPE myStruct : STRUCT
            x : DINT;
            y : REF_TO fb;
            z : REF_TO INT;
        END_STRUCT
        END_TYPE
        ",
        "
        FUNCTION_BLOCK fb
            VAR
                x : myStruct;
            END_VAR
        END_FUNCTION_BLOCK
        ",
    ];

    let id_provider = IdProvider::default();
    let (_, index1) = index_with_ids(units[0], id_provider.clone());
    let (unit2, index2) = index_with_ids(units[1], id_provider.clone());
    let mut index = index1;
    index.import(index2);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit2, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("fb".into())));
    assert!(dependencies.contains(&Dependency::Datatype("myStruct".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__myStruct_y".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__myStruct_z".into())));
    assert!(dependencies.contains(&Dependency::Datatype("INT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert_eq!(dependencies.len(), 6);
}

#[test]
fn enum_dependency_resolution() {
    let unit1 = SourceCode::new(
        "
        TYPE myEnum : LINT(a, b, c) END_TYPE
        TYPE myEnum2 : (d, e, f) END_TYPE
        ",
        "enum.st",
    );
    let unit2 = SourceCode::new(
        "
        PROGRAM prog
            VAR
                x : myEnum;
                y : myEnum2;
            END_VAR
        END_PROGRAM
        ",
        "prog.st",
    );
    let unit3 = SourceCode::new(
        "
        PROGRAM prog2
            VAR
            END_VAR
        END_PROGRAM
        ",
        "prog2.st",
    );

    let id_provider = IdProvider::default();
    let (_, index1) = index_with_ids(unit1, id_provider.clone());
    let (unit2, index2) = index_with_ids(unit2, id_provider.clone());
    let (unit3, index3) = index_with_ids(unit3, id_provider.clone());
    let mut index = index1;
    index.import(index2);
    index.import(index3);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit2, id_provider.clone());
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Datatype("myEnum".into())));
    assert!(dependencies.contains(&Dependency::Datatype("LINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("myEnum2".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert_eq!(dependencies.len(), 5);
    // Make sure prog2 does not have enum depedencies
    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit3, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog2".into())));
    assert_eq!(dependencies.len(), 1);
}

#[test]
fn alias_dependency_resolution() {
    let units = [
        "
        TYPE myAlias : LINT END_TYPE
        TYPE myAlias2 : REAL END_TYPE
        ",
        "
        PROGRAM prog
            VAR
                x : myAlias;
                y : myAlias2;
            END_VAR
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (_, index1) = index_with_ids(units[0], id_provider.clone());
    let (unit2, index2) = index_with_ids(units[1], id_provider.clone());
    let mut index = index1;
    index.import(index2);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit2, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Datatype("myAlias".into())));
    assert!(dependencies.contains(&Dependency::Datatype("LINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("myAlias2".into())));
    assert!(dependencies.contains(&Dependency::Datatype("REAL".into())));
    assert_eq!(dependencies.len(), 5);
}

#[test]
fn subrange_dependency_resolution() {
    let units = [
        "
        TYPE myRange : LINT (0..10) END_TYPE
        TYPE myRange2 : INT (-10..10) END_TYPE
        ",
        "
        PROGRAM prog
            VAR
                x : myRange;
                y : myRange2;
            END_VAR
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (_, index1) = index_with_ids(units[0], id_provider.clone());
    let (unit2, index2) = index_with_ids(units[1], id_provider.clone());
    let mut index = index1;
    index.import(index2);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit2, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Datatype("myRange".into())));
    assert!(dependencies.contains(&Dependency::Datatype("LINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("myRange2".into())));
    assert!(dependencies.contains(&Dependency::Datatype("INT".into())));
    assert_eq!(dependencies.len(), 5);
}

#[test]
fn function_dependency_resolution() {
    let units = [
        "
        FUNCTION foo : DINT
        END_FUNCTION
        ",
        "
        PROGRAM prog
            foo();
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (_, index1) = index_with_ids(units[0], id_provider.clone());
    let (unit2, index2) = index_with_ids(units[1], id_provider.clone());
    let mut index = index1;
    index.import(index2);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit2, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Call("foo".into())));
    assert!(dependencies.contains(&Dependency::Datatype("foo".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert_eq!(dependencies.len(), 4);
}

#[test]
fn function_params_dependency_resolution() {
    let units = [
        "
        FUNCTION foo : BYTE
        VAR_INPUT
           a : DINT;
        END_VAR
        VAR_INPUT {ref}
           b : INT;
        END_VAR
        VAR_IN_OUT
           c : REAL;
        END_VAR
        VAR_OUTPUT
            d : LREAL;
        END_VAR
        VAR
           e : WORD;
        END_VAR
        END_FUNCTION
        ",
        "
        PROGRAM prog
            foo();
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (_, index1) = index_with_ids(units[0], id_provider.clone());
    let (unit2, index2) = index_with_ids(units[1], id_provider.clone());
    let mut index = index1;
    index.import(index2);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit2, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Call("foo".into())));
    assert!(dependencies.contains(&Dependency::Datatype("foo".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("INT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__auto_pointer_to_INT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("REAL".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__auto_pointer_to_REAL".into())));
    assert!(dependencies.contains(&Dependency::Datatype("LREAL".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__auto_pointer_to_LREAL".into())));
    assert!(dependencies.contains(&Dependency::Datatype("WORD".into())));
    assert!(dependencies.contains(&Dependency::Datatype("BYTE".into())));
    assert_eq!(dependencies.len(), 12);
}

#[test]
fn program_params_dependency_resolution() {
    let units = [
        "
        PROGRAM foo
        VAR_INPUT
           a : DINT;
        END_VAR
        VAR_IN_OUT
           c : REAL;
        END_VAR
        VAR_OUTPUT
            d : LREAL;
        END_VAR
        VAR
           e : WORD;
        END_VAR
        END_PROGRAM
        ",
        "
        PROGRAM prog
            foo();
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (_, index1) = index_with_ids(units[0], id_provider.clone());
    let (unit2, index2) = index_with_ids(units[1], id_provider.clone());
    let mut index = index1;
    index.import(index2);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit2, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Call("foo".into())));
    assert!(dependencies.contains(&Dependency::Datatype("foo".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("REAL".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__auto_pointer_to_REAL".into())));
    assert!(dependencies.contains(&Dependency::Datatype("LREAL".into())));
    assert!(dependencies.contains(&Dependency::Datatype("WORD".into())));
    assert_eq!(dependencies.len(), 8);
}

#[test]
fn function_block_params_dependency_resolution() {
    let units = [
        "
        FUNCTION_BLOCK fb
        VAR_INPUT
           a : DINT;
        END_VAR
        VAR_INPUT {ref}
           b : INT;
        END_VAR
        VAR_IN_OUT
           c : REAL;
        END_VAR
        VAR_OUTPUT
            d : LREAL;
        END_VAR
        VAR
           e : WORD;
        END_VAR
        END_FUNCTION_BLOCK
        ",
        "
        PROGRAM prog
        VAR
            myFb : fb;
        END_VAR
            myFb();
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (_, index1) = index_with_ids(units[0], id_provider.clone());
    let (unit2, index2) = index_with_ids(units[1], id_provider.clone());
    let mut index = index1;
    index.import(index2);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit2, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Datatype("fb".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("INT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__auto_pointer_to_INT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("REAL".into())));
    assert!(dependencies.contains(&Dependency::Datatype("__auto_pointer_to_REAL".into())));
    assert!(dependencies.contains(&Dependency::Datatype("LREAL".into())));
    assert!(dependencies.contains(&Dependency::Datatype("WORD".into())));
    assert_eq!(dependencies.len(), 9);
}

#[test]
fn action_dependency_resolution() {
    let units = [
        "
        ACTION prog.foo
        END_ACTION
        ",
        "
        PROGRAM prog
            foo();
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (unit1, index1) = index_with_ids(units[0], id_provider.clone());
    let (_, index2) = index_with_ids(units[1], id_provider.clone());
    let mut index = index1;
    index.import(index2);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit1, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog.foo".into())));
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert_eq!(dependencies.len(), 2);
}

#[test]
fn action_dependency_resolution_with_variables() {
    let units = [
        "
        ACTION prog.foo
        END_ACTION
        ",
        "
        TYPE MyRange : DINT(0..5) END_TYPE
        ",
        "
        PROGRAM prog
            VAR
                x : LINT;
                y : MyRange;
            END_VAR
            foo();
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (unit, index1) = index_with_ids(units[0], id_provider.clone());
    let (_, index2) = index_with_ids(units[1], id_provider.clone());
    let (_, index3) = index_with_ids(units[2], id_provider.clone());
    let mut index = index1;
    index.import(index2);
    index.import(index3);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog.foo".into())));
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("LINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("MyRange".into())));
    assert_eq!(dependencies.len(), 5);
}

#[test]
fn action_dependency_resolution_with_function_block() {
    let units = [
        "
        FUNCTION_BLOCK fb
        END_FUNCTION_BLOCK
        ",
        "
        PROGRAM prog
            VAR
                myFb : fb;
            END_VAR

            myFb();
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (_, index1) = index_with_ids(units[0], id_provider.clone());
    let (unit, index2) = index_with_ids(units[1], id_provider.clone());
    let mut index = index1;
    index.import(index2);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Datatype("fb".into())));
    assert_eq!(dependencies.len(), 2);
}

#[test]
fn chained_function_dependency_resoltion() {
    let units = [
        "
        FUNCTION foo : DINT
            foo2();
        END_FUNCTION
        ",
        "
        FUNCTION foo2 : LINT
        END_FUNCTION
        ",
        "
        PROGRAM prog
            foo();
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (_, index1) = index_with_ids(units[0], id_provider.clone());
    let (_, index2) = index_with_ids(units[1], id_provider.clone());
    let (unit3, index3) = index_with_ids(units[2], id_provider.clone());
    let mut index = index1;
    index.import(index2);
    index.import(index3);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit3, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Call("foo".into())));
    assert!(dependencies.contains(&Dependency::Datatype("foo".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert_eq!(dependencies.len(), 4);
}

#[test]
fn generic_function_concrete_type_resolved() {
    let units = [
        "
        FUNCTION foo<T: ANY_NUMBER> : DINT
        VAR_INPUT
            x : T;
        END_VAR
        END_FUNCTION
        ",
        "
        PROGRAM prog
            foo(1);
            foo(INT#1);
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (_, index1) = index_with_ids(units[0], id_provider.clone());
    let (unit2, index2) = index_with_ids(units[1], id_provider.clone());
    let mut index = index1;
    index.import(index2);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit2, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Call("foo".into())));
    assert!(dependencies.contains(&Dependency::Datatype("foo".into())));
    assert!(dependencies.contains(&Dependency::Call("foo__DINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("foo__DINT".into())));
    assert!(dependencies.contains(&Dependency::Call("foo__INT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("foo__INT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("DINT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("INT".into())));
    assert_eq!(dependencies.len(), 10);
}

#[test]
fn global_variables_dependencies_resolved() {
    let units = [
        "
        VAR_GLOBAL
            x : INT;
        END_VAR
        ",
        "
        VAR_GLOBAL CONSTANT
            y : REAL := 3.14;
        END_VAR
        ",
        "
        PROGRAM prog
            x;
            y;
        END_PROGRAM
        ",
    ];

    let id_provider = IdProvider::default();
    let (unit1, index1) = index_with_ids(units[0], id_provider.clone());
    let (unit2, index2) = index_with_ids(units[1], id_provider.clone());
    let (unit3, index3) = index_with_ids(units[2], id_provider.clone());
    let mut index = index1;
    index.import(index2);
    index.import(index3);

    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit1, id_provider.clone());
    assert!(dependencies.contains(&Dependency::Variable("x".into())));
    assert!(dependencies.contains(&Dependency::Datatype("INT".into())));
    assert_eq!(dependencies.len(), 2);
    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit2, id_provider.clone());
    assert!(dependencies.contains(&Dependency::Variable("y".into())));
    assert!(dependencies.contains(&Dependency::Datatype("REAL".into())));
    assert_eq!(dependencies.len(), 2);
    let (_, dependencies, _) = TypeAnnotator::visit_unit(&index, &unit3, id_provider);
    assert!(dependencies.contains(&Dependency::Datatype("prog".into())));
    assert!(dependencies.contains(&Dependency::Datatype("INT".into())));
    assert!(dependencies.contains(&Dependency::Datatype("REAL".into())));
    assert!(dependencies.contains(&Dependency::Variable("x".into())));
    assert!(dependencies.contains(&Dependency::Variable("y".into())));
    assert_eq!(dependencies.len(), 5);
}
