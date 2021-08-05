use crate::{
    ast::*,
    parser::{parse, tests::lex},
};

#[test]
fn simple_class_with_defaults_can_be_parsed() {
    let lexer = lex("CLASS MyClass END_CLASS");
    let unit = parse(lexer).0;

    let class = unit.classes.first().unwrap();
    assert_eq!(class.name, "MyClass");
    assert_eq!(class.poly_mode, PolymorphisMode::None);
    assert_eq!(class.methods.len(), 0);
}

#[test]
fn simple_class_can_be_parsed() {
    let lexer = lex("CLASS ABSTRACT MyClass END_CLASS");
    let unit = parse(lexer).0;

    let class = unit.classes.first().unwrap();
    assert_eq!(class.name, "MyClass");
    assert_eq!(class.poly_mode, PolymorphisMode::Abstract);
    assert_eq!(class.methods.len(), 0);
}

#[test]
fn simple_class2_can_be_parsed() {
    let lexer = lex("CLASS FINAL MyClass2 END_CLASS");
    let unit = parse(lexer).0;

    let class = unit.classes.first().unwrap();
    assert_eq!(class.name, "MyClass2");
    assert_eq!(class.poly_mode, PolymorphisMode::Final);
    assert_eq!(class.methods.len(), 0);
}

#[test]
fn method_with_defaults_can_be_parsed() {
    let lexer = lex("CLASS MyClass METHOD testMethod END_METHOD END_CLASS");
    let unit = parse(lexer).0;

    let class = unit.classes.first().unwrap();
    assert_eq!(class.methods.len(), 1);

    let method = class.methods.first().unwrap();

    assert_eq!(method.name, "testMethod");
    assert_eq!(method.access, AccessModifier::Protected);
    assert_eq!(method.poly_mode, PolymorphisMode::None);
    assert_eq!(method.return_type, None);
    assert_eq!(method.overriding, false);
}

#[test]
fn method_can_be_parsed() {
    let lexer =
        lex("CLASS MyClass METHOD INTERNAL FINAL OVERRIDE testMethod2 END_METHOD END_CLASS");
    let unit = parse(lexer).0;

    let class = unit.classes.first().unwrap();
    assert_eq!(class.methods.len(), 1);

    let method = class.methods.first().unwrap();

    assert_eq!(method.name, "testMethod2");
    assert_eq!(method.access, AccessModifier::Internal);
    assert_eq!(method.poly_mode, PolymorphisMode::Final);
    assert_eq!(method.return_type, None);
    assert_eq!(method.overriding, true);
}

#[test]
fn method_with_return_type_can_be_parsed() {
    let lexer = lex(
        "CLASS MyClass METHOD PRIVATE ABSTRACT OVERRIDE testMethod3 : SINT END_METHOD END_CLASS",
    );
    let unit = parse(lexer).0;

    let class = unit.classes.first().unwrap();
    assert_eq!(class.methods.len(), 1);

    let method = class.methods.first().unwrap();

    assert_eq!(method.name, "testMethod3");
    assert_eq!(method.access, AccessModifier::Private);
    assert_eq!(method.poly_mode, PolymorphisMode::Abstract);
    assert_ne!(method.return_type, None);
    assert_eq!(method.overriding, true);
}

#[test]
fn class_with_var_default_block() {
    let lexer = lex(
        "CLASS MyClass VAR END_VAR END_CLASS",
    );
    let unit = parse(lexer).0;

    let class = unit.classes.first().unwrap();
    assert_eq!(class.methods.len(), 0);

    let vblock = class.variable_blocks.first().unwrap();
    assert_eq!(vblock.variables.len(), 0);

    assert_eq!(vblock.retain, false);
    assert_eq!(vblock.constant, false);
    assert_eq!(vblock.access, AccessModifier::Protected);
    assert_eq!(vblock.variable_block_type, VariableBlockType::Local);
}

#[test]
fn class_with_var_non_retain_block() {
    let lexer = lex(
        "CLASS MyClass VAR CONSTANT NON_RETAIN PUBLIC END_VAR END_CLASS",
    );
    let unit = parse(lexer).0;

    let class = unit.classes.first().unwrap();
    assert_eq!(class.methods.len(), 0);

    let vblock = class.variable_blocks.first().unwrap();
    assert_eq!(vblock.variables.len(), 0);

    assert_eq!(vblock.retain, false);
    assert_eq!(vblock.constant, true);
    assert_eq!(vblock.access, AccessModifier::Public);
    assert_eq!(vblock.variable_block_type, VariableBlockType::Local);
}

#[test]
fn class_with_var_retain_block() {
    let lexer = lex(
        "CLASS MyClass VAR RETAIN INTERNAL END_VAR END_CLASS",
    );
    let unit = parse(lexer).0;

    let class = unit.classes.first().unwrap();
    assert_eq!(class.methods.len(), 0);

    let vblock = class.variable_blocks.first().unwrap();
    assert_eq!(vblock.variables.len(), 0);

    assert_eq!(vblock.retain, true);
    assert_eq!(vblock.constant, false);
    assert_eq!(vblock.access, AccessModifier::Internal);
    assert_eq!(vblock.variable_block_type, VariableBlockType::Local);
}
