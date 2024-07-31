use insta::assert_snapshot;
use plc_ast::provider::IdProvider;

use crate::{
    resolver::{const_evaluator, TypeAnnotator},
    test_utils::tests::{annotate_and_lower_with_ids, index_with_ids},
};

#[test]
fn function_block_init_fn_created() {
    let id_provider = IdProvider::default();
    // GIVEN a function block with a complex initializer
    // WHEN lowered
    let (unit, index) = index_with_ids(
        "
        FUNCTION_BLOCK foo
        VAR
            s : STRING;
            ps: REF_TO STRING := REF(s);
        END_VAR
        ",
        id_provider.clone(),
    );

    let (_, index, annotated_units) = annotate_and_lower_with_ids(unit, index, id_provider);

    // THEN we expect the index to now have a corresponding init function
    assert!(index.find_pou("__init_foo").is_some());
    // AND we expect a new unit to be created for it
    let units = annotated_units.iter().map(|(units, _, _)| units).collect::<Vec<_>>();
    let init_foo = &units[1].implementations[0];
    assert_eq!(init_foo.name, "__init_foo");

    // 
}


// TODO: rewrite previous index tests to test for hits after lowering - pou init functions are no longer registered in the index before the lowering stage
// #[test]
// fn tmp() {
//     // GIVEN a struct type definition
//     // WHEN it is indexed
//     let (_, index) = index(
//         "
//         TYPE STRUCT1 : STRUCT
//             value : DINT;
//         END_STRUCT END_TYPE
//         ",
//     );

//     // THEN we expect a corresponding init function to be declared for it
//     let init = index.type_has_init_function("STRUCT1");
//     assert!(init);
// }

// #[test]
// fn tmp2() {
//     // GIVEN a declared PROGRAM
//     // WHEN it is indexed
//     let (_, index) = index(
//         "
//         PROGRAM main
//         END_PROGRAM
//         ",
//     );

//     // THEN we expect a corresponding init function to be declared for it
//     let init = index.type_has_init_function("main");
//     assert!(init);
// }

// #[test]
// fn tmp3() {
//     // GIVEN a declared FUNCTION_BLOCK with an ACTION
//     // WHEN it is indexed
//     let (_, index) = index(
//         "
//         FUNCTION_BLOCK foo
//         END_FUNCTION_BLOCK

//         ACTION act1
//         END_ACTION
//         ",
//     );

//     // THEN we expect a corresponding init function to be declared for the FUNCTION_BLOCK
//     // but not for the ACTION
//     let init = index.type_has_init_function("foo");
//     assert!(init);

//     let init = index.type_has_init_function("act1");
//     assert!(!init);
// }

// #[test]
// fn tmp4() {
//     // GIVEN a declared FUNCTION
//     // WHEN it is indexed
//     let (_, index) = index(
//         "
//         FUNCTION foo
//         END_FUNCTION
//         ",
//     );

//     // THEN we DO NOT expect a corresponding init function to be declared for it
//     let init = index.type_has_init_function("foo");
//     assert!(!init);
// }
