use crate::resolver::const_resolver::{resolve_constants, LiteralValue};
use crate::resolver::tests::{annotate, parse};

#[test]
fn binary_expressions_resolves_types() {
    let (unit, index) = parse(
        "VAR_GLOBAL CONSTANT
            x : INT := 3;
            y : INT := 4;
            z : INT := y;
        END_VAR
        
        VAR_GLOBAL CONSTANT
            a : INT := x;
            b : INT := y;
            c : INT := z;
        END_VAR
        ",
    );

    let (constants, unresolvable) = resolve_constants(&index);

    let empty: Vec<String> = vec![];
    assert_eq!(empty, unresolvable);
    println!("{:#?}", constants);
    assert_eq!(constants.get("a"), Some(&LiteralValue::IntLiteral(3)));
    assert_eq!(constants.get("b"), Some(&LiteralValue::IntLiteral(4)));
    assert_eq!(constants.get("c"), Some(&LiteralValue::IntLiteral(4)));
}
