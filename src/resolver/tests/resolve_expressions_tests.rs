use crate::{resolver::{tests::{annotate, parse}}};


#[test]
fn binary_expressions_resolves_types() {
    let (unit, index) = parse(
        "PROGRAM PRG
            1 + 2;
            1 + 2000;
            2000 + 1;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["BYTE", "UINT", "UINT"];
    for (i, s) in statements.iter().enumerate() {
        assert_eq!(
            Some(&expected_types[i].to_string()),
            annotations.type_map.get(&s.get_id()),
            "{:#?}",
            s
        );
    }
}

#[test]
fn binary_expressions_resolves_types_with_floats() {
    let (unit, index) = parse(
        "PROGRAM PRG
            1 + 2.2;
            1.1 + 2000;
            2000.0 + 1.0;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["REAL", "REAL", "REAL"];
    for (i, s) in statements.iter().enumerate() {
        assert_eq!(
            Some(&expected_types[i].to_string()),
            annotations.type_map.get(&s.get_id()),
            "{:#?}",
            s
        );
    }
}