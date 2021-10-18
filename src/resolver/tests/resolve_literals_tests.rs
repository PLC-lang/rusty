use crate::{
    ast::AstStatement,
    resolver::tests::{annotate, parse},
    typesystem::{DataTypeInformation, CONST_STRING_TYPE, CONST_WSTRING_TYPE},
};

#[test]
fn bool_literals_are_annotated() {
    let (unit, index) = parse(
        "PROGRAM PRG
                TRUE;
                FALSE;
            END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    assert_eq!(
        "BOOL",
        annotations
            .get_type_or_void(&statements[0], &index)
            .get_name()
    );
    assert_eq!(
        "BOOL",
        annotations
            .get_type_or_void(&statements[1], &index)
            .get_name()
    );
}

#[test]
fn string_literals_are_annotated() {
    let (unit, index) = parse(
        r#"PROGRAM PRG
                "abc";
                'xyz';
            END_PROGRAM"#,
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![CONST_WSTRING_TYPE, CONST_STRING_TYPE];

    let types: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(expected_types, types);
}

#[test]
fn int_literals_are_annotated() {
    let (unit, index) = parse(
        "PROGRAM PRG
                0;
                127;
                128;
                32767;
                32768;
                2147483647;
                2147483648;
            END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["DINT", "DINT", "DINT", "DINT", "DINT", "DINT", "LINT"];

    let types: Vec<&str> = statements
        .iter()
        .map(|s| annotations.get_type_or_void(s, &index).get_name())
        .collect();

    assert_eq!(expected_types, types);
}

#[test]
fn date_literals_are_annotated() {
    let (unit, index) = parse(
        "PROGRAM PRG
                T#12.4d;
                TIME#-12m;
                TOD#00:00:12;
                TIME_OF_DAY#04:16:22;
                DATE_AND_TIME#1984-10-01-16:40:22; 
                DT#2021-04-20-22:33:14; 
                DATE#1984-10-01; 
                D#2021-04-20; 
            END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![
        "TIME",
        "TIME",
        "TIME_OF_DAY",
        "TIME_OF_DAY",
        "DATE_AND_TIME",
        "DATE_AND_TIME",
        "DATE",
        "DATE",
    ];
    for (i, s) in statements.iter().enumerate() {
        assert_eq!(
            expected_types[i],
            annotations.get_type_or_void(s, &index).get_name(),
            "{:#?}",
            s
        );
    }
}

#[test]
fn real_literals_are_annotated() {
    let (unit, index) = parse(
        "PROGRAM PRG
                3.1415;
                1.0;
            END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["REAL", "REAL"];
    for (i, s) in statements.iter().enumerate() {
        assert_eq!(
            expected_types[i].to_string(),
            annotations.get_type_or_void(s, &index).get_name(),
            "{:#?}",
            s
        );
    }
}

#[test]
fn casted_literals_are_annotated() {
    let (unit, index) = parse(
        "PROGRAM PRG
                SINT#7;
                INT#7;
                DINT#7;
                LINT#7;
                REAL#7.7;
                LREAL#7.7;
                BOOL#1;
                BOOL#FALSE;
            END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![
        "SINT", "INT", "DINT", "LINT", "REAL", "LREAL", "BOOL", "BOOL",
    ];
    let actual_types: Vec<&str> = statements
        .iter()
        .map(|it| annotations.get_type_or_void(it, &index).get_name())
        .collect();

    assert_eq!(
        format!("{:#?}", expected_types),
        format!("{:#?}", actual_types),
    )
}

#[test]
fn enum_literals_are_annotated() {
    let (unit, index) = parse(
        "
            TYPE Color: (Green, Yellow, Red); END_TYPE
            TYPE Animal: (Dog, Cat, Horse); END_TYPE

            VAR_GLOBAL 
                Cat : BOOL;
            END_VAR
        
            PROGRAM PRG
                VAR Yellow: BYTE; END_VAR

                Green;  //Color
                Dog;    //Animal

                Yellow;     // local variable
                Color#Yellow;  //Animal

                Cat;   //global variable
                Animal#Cat;  //Animal

                // make sure these dont accidentally resolve to wrong enum
                Animal#Green;   //invalid (VOID)
                Color#Dog;      //invalid (VOID)
                invalid#Dog;    //invalid (VOID)
                Animal.Dog;     //invalid (VOID)
                PRG.Cat;        //invalid (VOID)

            END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let actual_resolves: Vec<&str> = statements
        .iter()
        .map(|it| annotations.get_type_or_void(it, &index).get_name())
        .collect();
    assert_eq!(
        vec![
            "Color", "Animal", "BYTE", "Color", "BOOL", "Animal", "VOID", "VOID", "VOID", "VOID",
            "VOID"
        ],
        actual_resolves
    )
}

#[test]
fn enum_literals_target_are_annotated() {
    let (unit, index) = parse(
        "
            TYPE Color: (Green, Yellow, Red); END_TYPE

            PROGRAM PRG
                Color#Red;
            END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let color_red = &unit.implementations[0].statements[0];

    assert_eq!(
        &DataTypeInformation::Enum {
            name: "Color".into(),
            elements: vec!["Green".into(), "Yellow".into(), "Red".into()]
        },
        annotations
            .get_type_or_void(color_red, &index)
            .get_type_information()
    );

    if let AstStatement::CastStatement { target, .. } = color_red {
        assert_eq!(
            &DataTypeInformation::Enum {
                name: "Color".into(),
                elements: vec!["Green".into(), "Yellow".into(), "Red".into()]
            },
            annotations
                .get_type_or_void(target, &index)
                .get_type_information()
        );
    } else {
        panic!("no cast statement")
    }
}

#[test]
fn casted_inner_literals_are_annotated() {
    let (unit, index) = parse(
        "PROGRAM PRG
                SINT#7;
                INT#7;
                DINT#7;
                LINT#7;
                REAL#7.7;
                LREAL#7.7;
                BOOL#1;
                BOOL#FALSE;
            END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![
        "SINT", "INT", "DINT", "LINT", "REAL", "LREAL", "BOOL", "BOOL",
    ];
    let actual_types: Vec<&str> = statements
        .iter()
        .map(|it| {
            if let AstStatement::CastStatement { target, .. } = it {
                target
            } else {
                panic!("no cast")
            }
        })
        .map(|it| annotations.get_type_or_void(it, &index).get_name())
        .collect();

    assert_eq!(
        format!("{:#?}", expected_types),
        format!("{:#?}", actual_types),
    )
}

#[test]
fn casted_literals_enums_are_annotated_correctly() {
    let (unit, index) = parse(
        "
            TYPE Color: (red, green, blue); END_TYPE
            PROGRAM PRG
                Color#red;
                Color#green;
                Color#blue;
            END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["Color", "Color", "Color"];
    let actual_types: Vec<&str> = statements
        .iter()
        .map(|it| {
            if let AstStatement::CastStatement { target, .. } = it {
                target
            } else {
                unreachable!();
            }
        })
        .map(|it| annotations.get_type_or_void(it, &index).get_name())
        .collect();

    assert_eq!(
        format!("{:#?}", expected_types),
        format!("{:#?}", actual_types),
    )
}

#[test]
fn expression_list_members_are_annotated() {
    let (unit, index) = parse(
        "PROGRAM PRG
                (1,TRUE,3.1415);
            END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let exp_list = &unit.implementations[0].statements[0];

    let expected_types = vec!["DINT", "BOOL", "REAL"];

    if let AstStatement::ExpressionList { expressions, .. } = exp_list {
        let actual_types: Vec<&str> = expressions
            .iter()
            .map(|it| annotations.get_type_or_void(it, &index).get_name())
            .collect();

        assert_eq!(
            format!("{:#?}", expected_types),
            format!("{:#?}", actual_types),
        )
    } else {
        unreachable!()
    }
}

#[test]
fn expression_lists_with_expressions_are_annotated() {
    let (unit, index) = parse(
        "
            VAR_GLOBAL CONSTANT
                a : INT : = 2;
                b : BOOL : = FALSE;
                c : LREAL : = 3.14;
            END_VAR

            PROGRAM PRG
                (a + a, b OR b , 2 * c, a + c);
            END_PROGRAM",
    );
    let annotations = annotate(&unit, &index);
    let exp_list = &unit.implementations[0].statements[0];

    let expected_types = vec!["DINT", "BOOL", "LREAL", "LREAL"];

    if let AstStatement::ExpressionList { expressions, .. } = exp_list {
        let actual_types: Vec<&str> = expressions
            .iter()
            .map(|it| annotations.get_type_or_void(it, &index).get_name())
            .collect();

        assert_eq!(
            format!("{:#?}", expected_types),
            format!("{:#?}", actual_types),
        )
    } else {
        unreachable!()
    }
}

#[test]
fn array_initialization_is_annotated_correctly() {
    let (unit, index) = parse(
        "
            VAR_GLOBAL CONSTANT
                a : ARRAY[0..2] OF BYTE := [1,2,3];
            END_VAR
            ",
    );

    let annotations = annotate(&unit, &index);

    let a_init = unit.global_vars[0].variables[0]
        .initializer
        .as_ref()
        .unwrap();
    let t = annotations.get_type_hint(a_init, &index).unwrap();
    assert_eq!(
        index.find_global_variable("a").unwrap().get_type_name(),
        t.get_name()
    )
}
