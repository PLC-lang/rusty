use plc_ast::ast::{AccessModifier, ArgumentProperty, PolymorphismMode, PouType, VariableBlockType};

use crate::test_utils::tests::parse;

#[test]
fn simple_class_with_defaults_can_be_parsed() {
    let src = "CLASS MyClass END_CLASS";
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Class);

    assert_eq!(class.name, "MyClass");
    assert_eq!(class.poly_mode, Some(PolymorphismMode::None));

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 1);
}

#[test]
fn extends_can_be_parsed() {
    let src = "
    CLASS MyClass
    END_CLASS

    CLASS MyClass2 EXTENDS MyClass
    END_CLASS
    ";
    let unit = parse(src).0;

    assert_eq!(&unit.units[1].super_class.clone().unwrap(), "MyClass");
}

#[test]
fn simple_class_can_be_parsed() {
    let src = "CLASS ABSTRACT MyClass END_CLASS";
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Class);

    assert_eq!(class.name, "MyClass");
    assert_eq!(class.poly_mode, Some(PolymorphismMode::Abstract));

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 1);
}

#[test]
fn simple_class2_can_be_parsed() {
    let src = "CLASS FINAL MyClass2 END_CLASS";
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Class);

    assert_eq!(class.name, "MyClass2");
    assert_eq!(class.poly_mode, Some(PolymorphismMode::Final));

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 1);
}

#[test]
fn method_with_defaults_can_be_parsed() {
    let src = "CLASS MyClass METHOD testMethod END_METHOD END_CLASS";
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Class);

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 2);

    let method_pou = &unit.units[1];
    assert_eq!(method_pou.pou_type, PouType::Method { owner_class: "MyClass".into() });
    let method = &unit.implementations[0];

    assert_eq!(method_pou.name, "MyClass.testMethod");
    assert_eq!(method.access, Some(AccessModifier::Protected));
    assert_eq!(method_pou.poly_mode, Some(PolymorphismMode::None));
    assert_eq!(method_pou.return_type, None);
    assert_eq!(method.overriding, false);
}

#[test]
fn method_can_be_parsed() {
    let src = "CLASS MyClass METHOD INTERNAL FINAL OVERRIDE testMethod2 END_METHOD END_CLASS";
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Class);

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 2);

    let method_pou = &unit.units[1];
    assert_eq!(method_pou.pou_type, PouType::Method { owner_class: "MyClass".into() });
    let method = &unit.implementations[0];

    assert_eq!(method_pou.name, "MyClass.testMethod2");
    assert_eq!(method.access, Some(AccessModifier::Internal));
    assert_eq!(method_pou.poly_mode, Some(PolymorphismMode::Final));
    assert_eq!(method_pou.return_type, None);
    assert_eq!(method.overriding, true);
}

#[test]
fn two_methods_can_be_parsed() {
    let src ="CLASS MyClass METHOD INTERNAL testMethod2 END_METHOD METHOD PROTECTED otherMethod VAR_TEMP END_VAR END_METHOD END_CLASS";
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Class);

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 3);

    let method1 = &unit.implementations[0];
    assert_eq!(method1.name, "MyClass.testMethod2");
    assert_eq!(method1.access, Some(AccessModifier::Internal));

    let method2 = &unit.implementations[1];
    assert_eq!(method2.name, "MyClass.otherMethod");
    assert_eq!(method2.access, Some(AccessModifier::Protected));
}

#[test]
fn method_with_return_type_can_be_parsed() {
    let src = "CLASS MyClass METHOD PRIVATE ABSTRACT OVERRIDE testMethod3 : SINT END_METHOD END_CLASS";
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Class);

    let method_pou = &unit.units[1];
    assert_eq!(method_pou.pou_type, PouType::Method { owner_class: "MyClass".into() });
    let method = &unit.implementations[0];

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 2);

    assert_eq!(method_pou.name, "MyClass.testMethod3");
    assert_eq!(method.access, Some(AccessModifier::Private));
    assert_eq!(method_pou.poly_mode, Some(PolymorphismMode::Abstract));
    assert_ne!(method_pou.return_type, None);
    assert_eq!(method.overriding, true);
}

#[test]
fn class_with_var_default_block() {
    let src = "CLASS MyClass VAR END_VAR END_CLASS";
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Class);

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 1);

    let vblock = &class.variable_blocks[0];
    assert_eq!(vblock.variables.len(), 0);

    assert_eq!(vblock.retain, false);
    assert_eq!(vblock.constant, false);
    assert_eq!(vblock.access, AccessModifier::Protected);
    assert_eq!(vblock.variable_block_type, VariableBlockType::Local);
}

#[test]
fn class_with_var_non_retain_block() {
    let src = "CLASS MyClass VAR CONSTANT NON_RETAIN PUBLIC END_VAR END_CLASS";
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Class);

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 1);

    let vblock = &class.variable_blocks[0];
    assert_eq!(vblock.variables.len(), 0);

    assert_eq!(vblock.retain, false);
    assert_eq!(vblock.constant, true);
    assert_eq!(vblock.access, AccessModifier::Public);
    assert_eq!(vblock.variable_block_type, VariableBlockType::Local);
}

#[test]
fn class_with_var_retain_block() {
    let src = "CLASS MyClass VAR RETAIN INTERNAL END_VAR END_CLASS";
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Class);

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 1);

    let vblock = &class.variable_blocks[0];
    assert_eq!(vblock.variables.len(), 0);

    assert_eq!(vblock.retain, true);
    assert_eq!(vblock.constant, false);
    assert_eq!(vblock.access, AccessModifier::Internal);
    assert_eq!(vblock.variable_block_type, VariableBlockType::Local);
}

#[test]
fn method_with_var_block() {
    let src = "CLASS MyClass METHOD testMethod3 VAR_TEMP END_VAR END_METHOD END_CLASS";
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Class);

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 2);

    let method_pou = &unit.units[1];
    let vblock = &method_pou.variable_blocks[0];

    assert_eq!(vblock.retain, false);
    assert_eq!(vblock.constant, false);
    assert_eq!(vblock.access, AccessModifier::Protected);
    assert_eq!(vblock.variable_block_type, VariableBlockType::Temp);
}

#[test]
fn method_with_var_inout_blocks() {
    let src = r#"
            CLASS MyClass
                METHOD testMethod3
                    VAR_INPUT CONSTANT
                        x:SINT := 3;
                    END_VAR
                    VAR_IN_OUT END_VAR
                    VAR_OUTPUT END_VAR
                END_METHOD
            END_CLASS"#;
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Class);

    let method_pou = &unit.units[1];

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 2);

    assert_eq!(method_pou.variable_blocks.len(), 3);
    let vblock1 = &method_pou.variable_blocks[0];
    let vblock2 = &method_pou.variable_blocks[1];
    let vblock3 = &method_pou.variable_blocks[2];

    assert_eq!(vblock1.constant, true);
    assert_eq!(vblock1.variable_block_type, VariableBlockType::Input(ArgumentProperty::ByVal));

    assert_eq!(vblock2.constant, false);
    assert_eq!(vblock2.variable_block_type, VariableBlockType::InOut);

    assert_eq!(vblock3.constant, false);
    assert_eq!(vblock3.variable_block_type, VariableBlockType::Output);
}

#[test]
fn fb_method_can_be_parsed() {
    let src = r#"
            FUNCTION_BLOCK MyFb
                METHOD INTERNAL FINAL OVERRIDE testMethod2 END_METHOD
            END_FUNCTION_BLOCK
        "#;
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::FunctionBlock);

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 2);

    let method_pou = &unit.units[1];
    assert_eq!(method_pou.pou_type, PouType::Method { owner_class: "MyFb".into() });
    let method = &unit.implementations[0];

    assert_eq!(method_pou.name, "MyFb.testMethod2");
    assert_eq!(method.access, Some(AccessModifier::Internal));
    assert_eq!(method_pou.poly_mode, Some(PolymorphismMode::Final));
    assert_eq!(method_pou.return_type, None);
    assert_eq!(method.overriding, true);
}

#[test]
fn fb_two_methods_can_be_parsed() {
    let src = r#"
            FUNCTION_BLOCK MyNewFb
                METHOD INTERNAL testMethod2 END_METHOD
                METHOD otherMethod VAR_TEMP END_VAR END_METHOD
            END_FUNCTION_BLOCK
        "#;
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::FunctionBlock);

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 3);

    let method1 = &unit.implementations[0];
    assert_eq!(method1.name, "MyNewFb.testMethod2");
    assert_eq!(method1.access, Some(AccessModifier::Internal));

    let method2 = &unit.implementations[1];
    assert_eq!(method2.name, "MyNewFb.otherMethod");
    assert_eq!(method2.access, Some(AccessModifier::Protected));
}

#[test]
fn fb_method_with_return_type_can_be_parsed() {
    let src = r#"
        FUNCTION_BLOCK MyShinyFb
            METHOD PRIVATE ABSTRACT OVERRIDE testMethod3 : SINT END_METHOD
        END_FUNCTION_BLOCK
    "#;
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::FunctionBlock);

    let method_pou = &unit.units[1];
    assert_eq!(method_pou.pou_type, PouType::Method { owner_class: "MyShinyFb".into() });
    let method = &unit.implementations[0];

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 2);

    assert_eq!(method_pou.name, "MyShinyFb.testMethod3");
    assert_eq!(method.access, Some(AccessModifier::Private));
    assert_eq!(method_pou.poly_mode, Some(PolymorphismMode::Abstract));
    assert_ne!(method_pou.return_type, None);
    assert_eq!(method.overriding, true);
}

#[test]
fn program_methods_can_be_parsed() {
    let src = r#"
        PROGRAM prog
            METHOD INTERNAL FINAL OVERRIDE testMethod2 END_METHOD
        END_PROG
    "#;
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Program);

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 2);

    let method_pou = &unit.units[1];
    assert_eq!(method_pou.pou_type, PouType::Method { owner_class: "prog".into() });
    let method = &unit.implementations[0];

    assert_eq!(method_pou.name, "prog.testMethod2");
    assert_eq!(method.access, Some(AccessModifier::Internal));
    assert_eq!(method_pou.poly_mode, Some(PolymorphismMode::Final));
    assert_eq!(method_pou.return_type, None);
    assert_eq!(method.overriding, true);
}

#[test]
fn program_two_methods_can_be_parsed() {
    let src = r#"
            PROGRAM prog
                METHOD INTERNAL testMethod2 END_METHOD
                METHOD otherMethod VAR_TEMP END_VAR END_METHOD
            END_PROGRAM
        "#;
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Program);

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 3);

    let method1 = &unit.implementations[0];
    assert_eq!(method1.name, "prog.testMethod2");
    assert_eq!(method1.access, Some(AccessModifier::Internal));

    let method2 = &unit.implementations[1];
    assert_eq!(method2.name, "prog.otherMethod");
    assert_eq!(method2.access, Some(AccessModifier::Protected));
}

#[test]
fn program_method_with_return_type_can_be_parsed() {
    let src = r#"
        PROGRAM prog
            METHOD PRIVATE ABSTRACT OVERRIDE testMethod3 : SINT END_METHOD
        END_PROGRAM
    "#;
    let unit = parse(src).0;

    let class = &unit.units[0];
    assert_eq!(class.pou_type, PouType::Program);

    let method_pou = &unit.units[1];
    assert_eq!(method_pou.pou_type, PouType::Method { owner_class: "prog".into() });
    let method = &unit.implementations[0];

    // classes have implementation because they are treated as other POUs
    assert_eq!(unit.implementations.len(), 2);

    assert_eq!(method_pou.name, "prog.testMethod3");
    assert_eq!(method.access, Some(AccessModifier::Private)); // TODO: default public?
    assert_eq!(method_pou.poly_mode, Some(PolymorphismMode::Abstract));
    assert_ne!(method_pou.return_type, None);
    assert_eq!(method.overriding, true);
}

#[test]
fn method_variable_blocks_can_be_parsed() {
    let src = r"
    FUNCTION_BLOCK fb
        METHOD mthd
            VAR
                x : DINT;
                y : DINT := 3;
            END_VAR
            VAR_TEMP
                xTmp : DINT;
                yTmp : DINT := 3;
            END_VAR
            VAR_INPUT
                xIn : DINT;
                yIn : DINT := 3;
            END_VAR
            VAR_OUTPUT
                xOut : DINT;
                yOut : DINT := 3;
            END_VAR
            VAR_IN_OUT
                xIO : DINT;
                yIO : DINT;
            END_VAR
        END_METHOD
    END_FUNCTION_BLOCK

    PROGRAM prg
        METHOD mthd
            VAR
                x : DINT;
                y : DINT := 3;
            END_VAR
            VAR_TEMP
                xTmp : DINT;
                yTmp : DINT := 3;
            END_VAR
            VAR_INPUT
                xIn : DINT;
                yIn : DINT := 3;
            END_VAR
            VAR_OUTPUT
                xOut : DINT;
                yOut : DINT := 3;
            END_VAR
            VAR_IN_OUT
                xIO : DINT;
                yIO : DINT;
            END_VAR
        END_METHOD
    END_PROGRAM
    ";

    let (unit, _) = parse(src);
    let fb_mthd = &unit.units[1];
    assert_eq!(fb_mthd.name, "fb.mthd".to_string());
    assert_eq!(fb_mthd.pou_type, PouType::Method { owner_class: "fb".into() });

    let prg_mthd = &unit.units[3];
    assert_eq!(prg_mthd.name, "prg.mthd".to_string());
    assert_eq!(prg_mthd.pou_type, PouType::Method { owner_class: "prg".into() });

    // we expect one of each of these `VariableBlockType` to be parsed
    let expected_var_blocks = vec![
        VariableBlockType::Local,
        VariableBlockType::Temp,
        VariableBlockType::Input(ArgumentProperty::ByVal),
        VariableBlockType::Output,
        VariableBlockType::InOut,
    ];
    let actual =
        &fb_mthd.variable_blocks.iter().map(|it| it.variable_block_type).collect::<Vec<VariableBlockType>>();
    assert_eq!(&expected_var_blocks, actual);
    let actual =
        &prg_mthd.variable_blocks.iter().map(|it| it.variable_block_type).collect::<Vec<VariableBlockType>>();
    assert_eq!(&expected_var_blocks, actual);

    // we expect to have parsed 10 variables and 4 of them (all `y`s apart from the in-out) to have initializer
    let variables = &fb_mthd.variable_blocks.iter().fold(vec![], |mut acc, block| {
        acc.extend(block.variables.iter());
        acc
    });
    assert_eq!(variables.len(), 10);
    let with_initializer = variables.iter().filter(|it| it.initializer.is_some());
    assert_eq!(with_initializer.count(), 4);

    let variables = &prg_mthd.variable_blocks.iter().fold(vec![], |mut acc, block| {
        acc.extend(block.variables.iter());
        acc
    });
    assert_eq!(variables.len(), 10);
    let with_initializer = variables.iter().filter(|it| it.initializer.is_some());
    assert_eq!(with_initializer.count(), 4);
}
