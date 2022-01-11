use core::panic;

use crate::{assert_type_and_hint, ast::AstStatement, test_utils::tests::index, TypeAnnotator};

#[test]
fn binary_expressions_resolves_types() {
    let (unit, index) = index(
        "PROGRAM PRG
                VAR x : INT; END_VAR
                FOR x := 3 TO 10 BY 2 DO 
                    x;
                END_FOR
        END_PROGRAM",
    );
    let annotations = TypeAnnotator::visit_unit(&index, &unit);
    let statements = &unit.implementations[0].statements;

    if let AstStatement::ForLoopStatement {
        counter,
        start,
        end,
        by_step: Some(by_step),
        ..
    } = &statements[0]
    {
        assert_type_and_hint!(&annotations, &index, counter, "INT", None);
        assert_type_and_hint!(&annotations, &index, start, "DINT", Some("INT"));
        assert_type_and_hint!(&annotations, &index, end, "DINT", Some("INT"));
        assert_type_and_hint!(&annotations, &index, by_step, "DINT", Some("INT"));
    } else {
        panic!("no for loop statement");
    }
}
