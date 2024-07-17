use core::panic;

use plc_ast::{
    ast::AstStatement,
    control_statements::{AstControlStatement, ForLoopStatement},
    provider::IdProvider,
};

use crate::{assert_type_and_hint, test_utils::tests::index_with_ids};

use super::helper::visit_unit;

#[test]
fn binary_expressions_resolves_types() {
    let id_provider = IdProvider::default();
    let (unit, index) = index_with_ids(
        "PROGRAM PRG
                VAR x : INT; END_VAR
                FOR x := 3 TO 10 BY 2 DO
                    x;
                END_FOR
        END_PROGRAM",
        id_provider.clone(),
    );
    let (annotations, ..) = visit_unit(&index, &unit, id_provider);
    let statements = &unit.implementations[0].statements;

    if let AstStatement::ControlStatement(AstControlStatement::ForLoop(ForLoopStatement {
        counter,
        start,
        end,
        by_step: Some(by_step),
        ..
    })) = statements[0].get_stmt()
    {
        assert_type_and_hint!(&annotations, &index, counter, "INT", None);
        assert_type_and_hint!(&annotations, &index, start, "DINT", Some("INT"));
        assert_type_and_hint!(&annotations, &index, end, "DINT", Some("INT"));
        assert_type_and_hint!(&annotations, &index, by_step, "DINT", Some("INT"));
    } else {
        panic!("no for loop statement");
    }
}
