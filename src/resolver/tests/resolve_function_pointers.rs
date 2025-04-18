use crate::resolver::AnnotationMap;
use insta::assert_debug_snapshot;
use plc_ast::{ast::AstStatement, provider::IdProvider};

use crate::test_utils::tests::{annotate_with_ids, index_with_ids};

#[test]
fn function_pointer_call_resolved_to_function_prototype() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r"{external}
FUNCTION prot : DINT
VAR_INPUT
    a : DINT;
END_VAR
END_FUNCTION

FUNCTION test : DINT
VAR
    f : REF_TO prot := REF(prot);
END_VAR
    f := REF(prot);
    //  vvvv prot
    f^(1);
END_FUNCTION
    ",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let init = unit.pous[1].variable_blocks[0].variables[0].initializer.as_ref().unwrap();

    assert_debug_snapshot!(annotations.get(init), @r#"
    Some(
        Value {
            resulting_type: "__POINTER_TO_prot",
        },
    )
    "#);
    let statements = &unit.implementations[1].statements;
    let &AstStatement::Assignment(assignment) = &statements[0].get_stmt() else {
        panic!("Expected assignment statement");
    };
    assert_debug_snapshot!(annotations.get(&assignment.right), @r###"
    Some(
        Value {
            resulting_type: "__POINTER_TO_prot",
        },
    )
    "###);
    assert_debug_snapshot!(annotations.get_hint(&assignment.right), @r###"
    Some(
        Value {
            resulting_type: "__test_f",
        },
    )
    "###);
    let &AstStatement::CallStatement(call) = &statements[1].get_stmt() else {
        panic!("Expected call statement");
    };
    assert_debug_snapshot!(annotations.get(&call.operator), @r###"
    Some(
        Value {
            resulting_type: "prot",
        },
    )
    "###);
}

#[test]
fn cast_void_to_function_resolves_to_prototype() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r"
        {external}
FUNCTION prot : DINT
VAR_INPUT
    a : DINT;
END_VAR
END_FUNCTION

FUNCTION test : DINT
VAR
    f : REF_TO __VOID := REF(prot);
END_VAR
        //  vvvv type_hit: ref to void
    f := REF(prot);
        //  vvvv prot
    prot#(f^)(1);
END_FUNCTION
    ",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);
    let statements = &unit.implementations[1].statements;
    let &AstStatement::Assignment(assignment) = &statements[0].get_stmt() else {
        panic!("Expected assignment statement");
    };
    assert_debug_snapshot!(annotations.get(&assignment.right), @r###"
    Some(
        Value {
            resulting_type: "__POINTER_TO_prot",
        },
    )
    "###);
    assert_debug_snapshot!(annotations.get_hint(&assignment.right), @r###"
    Some(
        Value {
            resulting_type: "__test_f",
        },
    )
    "###);
    let &AstStatement::CallStatement(call) = &statements[1].get_stmt() else {
        panic!("Expected call statement");
    };
    assert_debug_snapshot!(annotations.get(&call.operator), @r###"
    Some(
        Value {
            resulting_type: "prot",
        },
    )
    "###);
}

#[test]
fn function_pointer_reference_different_function_resolved() {
    let id_provider = IdProvider::default();
    let (unit, mut index) = index_with_ids(
        r"{external}
VAR_GLOBAL
    gf : REF_TO prot := REF(myFunc);
END_VAR
FUNCTION prot : DINT
VAR_INPUT
    a : DINT;
END_VAR
END_FUNCTION
FUNCTION myFunc : DINT
VAR_INPUT
    a : DINT;
END_VAR
END_FUNCTION

FUNCTION test : DINT
VAR
    f : REF_TO prot := REF(myFunc);
END_VAR
    f := REF(myFunc);
    //  vvvv prot
    f^(1);
END_FUNCTION
    ",
        id_provider.clone(),
    );
    let annotations = annotate_with_ids(&unit, &mut index, id_provider);

    let init = &unit.global_vars[0].variables[0].initializer.as_ref().unwrap();

    assert_debug_snapshot!(annotations.get(init), @r#"
    Some(
        Value {
            resulting_type: "__POINTER_TO_myFunc",
        },
    )
    "#);
    assert_debug_snapshot!(annotations.get_hint(init), @r#"
    Some(
        Value {
            resulting_type: "__global_gf",
        },
    )
    "#);
    let init = unit.pous[2].variable_blocks[0].variables[0].initializer.as_ref().unwrap();

    assert_debug_snapshot!(annotations.get(init), @r#"
    Some(
        Value {
            resulting_type: "__POINTER_TO_myFunc",
        },
    )
    "#);
    assert_debug_snapshot!(annotations.get_hint(init), @r#"
    Some(
        Value {
            resulting_type: "__test_f",
        },
    )
    "#);
    let statements = &unit.implementations[2].statements;
    let &AstStatement::Assignment(assignment) = &statements[0].get_stmt() else {
        panic!("Expected assignment statement");
    };
    assert_debug_snapshot!(annotations.get(&assignment.right), @r#"
    Some(
        Value {
            resulting_type: "__POINTER_TO_myFunc",
        },
    )
    "#);
    assert_debug_snapshot!(annotations.get_hint(&assignment.right), @r###"
    Some(
        Value {
            resulting_type: "__test_f",
        },
    )
    "###);
    let &AstStatement::CallStatement(call) = &statements[1].get_stmt() else {
        panic!("Expected call statement");
    };
    assert_debug_snapshot!(annotations.get(&call.operator), @r###"
    Some(
        Value {
            resulting_type: "prot",
        },
    )
    "###);
}
