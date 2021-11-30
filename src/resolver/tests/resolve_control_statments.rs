use core::panic;

use crate::{ast::AstStatement, test_utils::tests::index, TypeAnnotator};

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
        let types = vec![
            annotations.get_type_or_void(counter, &index).get_name(),
            annotations.get_type_or_void(start, &index).get_name(),
            annotations.get_type_or_void(end, &index).get_name(),
            annotations.get_type_or_void(by_step, &index).get_name(),
        ];

        assert_eq!(vec!["INT", "DINT", "DINT", "DINT"], types);
    } else {
        panic!("no for loop statement");
    }
}
