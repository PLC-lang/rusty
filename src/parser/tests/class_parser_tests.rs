use crate::{
    ast::*,
    parser::{parse, tests::lex},
};

#[test]
fn simple_class_can_be_parsed() {
    let lexer = lex("CLASS MyClass END_CLASS");
    let unit = parse(lexer).0;

    assert_eq!(
        unit.classes.first().unwrap().name,
        "MyClass"
    );
}

#[test]
fn method_with_defaults_can_be_parsed() {
    let lexer = lex("CLASS MyClass METHOD testMethod END_METHOD END_CLASS");
    let unit = parse(lexer).0;

    let method = unit.classes.first().unwrap().methods.first().unwrap();

    assert_eq!(method.name, "testMethod");
    assert_eq!(method.access, AccessModifier::Protected);
    assert_eq!(method.poly_mode, PolymorphisMode::None);
    assert_eq!(method.return_type, None);
    assert_eq!(method.overriding, false);
}

#[test]
fn method_can_be_parsed() {
    let lexer = lex("CLASS MyClass METHOD INTERNAL FINAL OVERRIDE testMethod2 END_METHOD END_CLASS");
    let unit = parse(lexer).0;

    let method = unit.classes.first().unwrap().methods.first().unwrap();

    assert_eq!(method.name, "testMethod2");
    assert_eq!(method.access, AccessModifier::Internal);
    assert_eq!(method.poly_mode, PolymorphisMode::Final);
    assert_eq!(method.return_type, None);
    assert_eq!(method.overriding, true);
}

#[test]
fn method_with_return_type_can_be_parsed() {
    let lexer = lex("CLASS MyClass METHOD PRIVATE ABSTRACT OVERRIDE testMethod3 : SINT END_METHOD END_CLASS");
    let unit = parse(lexer).0;

    let method = unit.classes.first().unwrap().methods.first().unwrap();

    assert_eq!(method.name, "testMethod3");
    assert_eq!(method.access, AccessModifier::Private);
    assert_eq!(method.poly_mode, PolymorphisMode::Abstract);
    assert_ne!(method.return_type, None);
    assert_eq!(method.overriding, true);
}
