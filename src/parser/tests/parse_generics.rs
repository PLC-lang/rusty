use crate::ast::{DataTypeDeclaration, GenericBinding, Variable};
use crate::test_utils::tests::parse;
use crate::SourceRange;

#[test]
fn generic_markers_on_pou_added() {
    let src = "FUNCTION test<T: ANY, R : ANY_NUMBER> : INT END_FUNCTION";
    let (parse_result, _) = dbg!(parse(src));
    let function = &parse_result.units[0];
    //Make sure the function has the generic parametes T: ANY, R : ANY_NUMBER
    let generics = &function.generics;
    assert!(!generics.is_empty());
    let t = &generics[0];
    assert_eq!(&GenericBinding{ name: "T".into(), nature: "ANY".into()}, t);
    let r = &generics[1];
    assert_eq!(&GenericBinding{ name: "R".into(), nature: "ANY_NUMBER".into()}, r);
}

#[test]
fn generic_markers_on_method_added() {
    let src = "CLASS xx METHOD test<T: ANY, R : ANY_NUMBER> : INT END_METHOD END_CLASS";
    let (parse_result, _) = dbg!(parse(src));
    let function = &parse_result.units[1];
    //Make sure the function has the generic parametes T: ANY, R : ANY_NUMBER
    let generics = &function.generics;
    assert!(!generics.is_empty());
    let t = &generics[0];
    assert_eq!(&GenericBinding{ name: "T".into(), nature: "ANY".into()}, t);
    let r = &generics[1];
    assert_eq!(&GenericBinding{ name: "R".into(), nature: "ANY_NUMBER".into()}, r);
}

#[test]
fn generic_parameters_are_datatypes() {
    let src =
        "FUNCTION test<T: ANY, R : ANY_NUMBER> : R VAR_INPUT x : T; y : R; END_VAR END_FUNCTION";
    let (parse_result, _) = dbg!(parse(src));
    let function = &parse_result.units[0];
    let variables = &function.variable_blocks[0].variables;
    assert_eq!(
        &vec![
            Variable {
                name: "x".into(),
                data_type: DataTypeDeclaration::DataTypeReference {
                    referenced_type: "T".into(),
                    location: SourceRange::new(56..57),
                },
                initializer: None,
                location: SourceRange::new(52..53),
            },
            Variable {
                name: "y".into(),
                data_type: DataTypeDeclaration::DataTypeReference {
                    referenced_type: "R".into(),
                    location: SourceRange::new(63..64),
                },
                initializer: None,
                location: SourceRange::new(59..60),
            },
        ],
        variables
    );
}

#[test]
fn generic_method_parameters_are_datatypes() {
    let src = "CLASS cls METHOD test<T: ANY, R : ANY_NUMBER> : R VAR_INPUT x : T; y : R; END_VAR END_METHOD END_CLASS";
    let (parse_result, _) = dbg!(parse(src));
    let function = &parse_result.units[1];
    let variables = &function.variable_blocks[0].variables;
    assert_eq!(
        &vec![
            Variable {
                name: "x".into(),
                data_type: DataTypeDeclaration::DataTypeReference {
                    referenced_type: "T".into(),
                    location: SourceRange::new(64..65),
                },
                initializer: None,
                location: SourceRange::new(60..61),
            },
            Variable {
                name: "y".into(),
                data_type: DataTypeDeclaration::DataTypeReference {
                    referenced_type: "R".into(),
                    location: SourceRange::new(71..72),
                },
                initializer: None,
                location: SourceRange::new(67..68),
            },
        ],
        variables
    );
}

