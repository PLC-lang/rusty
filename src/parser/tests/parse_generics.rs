use insta::assert_debug_snapshot;
use plc_ast::ast::{GenericBinding, TypeNature};

use crate::test_utils::tests::parse;

#[test]
fn generic_markers_on_pou_added() {
    let src = "FUNCTION test<
        A: ANY,
        B : ANY_DERIVED,
        C : ANY_ELEMENTARY,
        D: ANY_MAGNITUDE,
        E: ANY_NUM,
        F : ANY_REAL,
        G : ANY_INT,
        H : ANY_SIGNED,
        I : ANY_UNSIGNED,
        J : ANY_DURATION,
        K : ANY_BIT,
        L : ANY_CHARS,
        M : ANY_STRING,
        N : ANY_CHAR,
        O : ANY_DATE,
        P : __ANY_VLA> : INT END_FUNCTION";
    let (parse_result, _) = parse(src);
    let function = &parse_result.pous[0];
    //Make sure the function has the generic parametes T: ANY, R : ANY_NUMBER
    let generics = &function.generics;
    assert!(!generics.is_empty());
    let t = &generics[0];
    assert_eq!(&GenericBinding { name: "A".into(), nature: TypeNature::Any }, t);
    let r = &generics[1];
    assert_eq!(&GenericBinding { name: "B".into(), nature: TypeNature::Derived }, r);
    let t = &generics[2];
    assert_eq!(&GenericBinding { name: "C".into(), nature: TypeNature::Elementary }, t);
    let r = &generics[3];
    assert_eq!(&GenericBinding { name: "D".into(), nature: TypeNature::Magnitude }, r);
    let t = &generics[4];
    assert_eq!(&GenericBinding { name: "E".into(), nature: TypeNature::Num }, t);
    let r = &generics[5];
    assert_eq!(&GenericBinding { name: "F".into(), nature: TypeNature::Real }, r);
    let t = &generics[6];
    assert_eq!(&GenericBinding { name: "G".into(), nature: TypeNature::Int }, t);
    let r = &generics[7];
    assert_eq!(&GenericBinding { name: "H".into(), nature: TypeNature::Signed }, r);
    let t = &generics[8];
    assert_eq!(&GenericBinding { name: "I".into(), nature: TypeNature::Unsigned }, t);
    let r = &generics[9];
    assert_eq!(&GenericBinding { name: "J".into(), nature: TypeNature::Duration }, r);
    let t = &generics[10];
    assert_eq!(&GenericBinding { name: "K".into(), nature: TypeNature::Bit }, t);
    let r = &generics[11];
    assert_eq!(&GenericBinding { name: "L".into(), nature: TypeNature::Chars }, r);
    let t = &generics[12];
    assert_eq!(&GenericBinding { name: "M".into(), nature: TypeNature::String }, t);
    let r = &generics[13];
    assert_eq!(&GenericBinding { name: "N".into(), nature: TypeNature::Char }, r);
    let t = &generics[14];
    assert_eq!(&GenericBinding { name: "O".into(), nature: TypeNature::Date }, t);
    let t = &generics[15];
    assert_eq!(&GenericBinding { name: "P".into(), nature: TypeNature::__VLA }, t);
}

#[test]
fn generic_markers_on_method_added() {
    let src = "CLASS xx METHOD test<T: ANY, R : ANY_NUM> : INT END_METHOD END_CLASS";
    let (parse_result, _) = parse(src);
    let function = &parse_result.pous[1];
    //Make sure the function has the generic parametes T: ANY, R : ANY_NUMBER
    let generics = &function.generics;
    assert!(!generics.is_empty());
    let t = &generics[0];
    assert_eq!(&GenericBinding { name: "T".into(), nature: TypeNature::Any }, t);
    let r = &generics[1];
    assert_eq!(&GenericBinding { name: "R".into(), nature: TypeNature::Num }, r);
}

#[test]
fn generic_parameters_are_datatypes() {
    let src = "FUNCTION test<T: ANY, R : ANY_NUM> : R VAR_INPUT x : T; y : R; END_VAR END_FUNCTION";
    let (parse_result, _) = parse(src);
    let function = &parse_result.pous[0];
    let variables = &function.variable_blocks[0].variables;
    assert_debug_snapshot!(variables);
}

#[test]
fn generic_method_parameters_are_datatypes() {
    let src =
        "CLASS cls METHOD test<T: ANY, R : ANY_NUM> : R VAR_INPUT x : T; y : R; END_VAR END_METHOD END_CLASS";
    let (parse_result, _) = parse(src);
    let function = &parse_result.pous[1];
    let variables = &function.variable_blocks[0].variables;
    assert_debug_snapshot!(variables);
}
