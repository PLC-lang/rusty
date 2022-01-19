use crate::{
    assert_type_and_hint,
    ast::AstStatement,
    resolver::{AnnotationMap, TypeAnnotator},
    test_utils::tests::{annotate, index},
    typesystem::{DataType, DataTypeInformation, StringEncoding, TypeSize, DINT_TYPE},
};

#[test]
fn bool_literals_are_annotated() {
    let (unit, index) = index(
        "PROGRAM PRG
                TRUE;
                FALSE;
            END_PROGRAM",
    );
    let annotations = TypeAnnotator::visit_unit(&index, &unit);
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
    //GIVEN some string literals
    let (unit, mut index) = index(
        r#"PROGRAM PRG
                'abc';
                "xyzxyz";
            END_PROGRAM"#,
    );

    //WHEN they are annotated
    let mut annotations = TypeAnnotator::visit_unit(&index, &unit);
    index.import(std::mem::take(&mut annotations.new_index));

    // THEN we expect them to be annotated with correctly sized string types
    let statements = &unit.implementations[0].statements;
    assert_type_and_hint!(&annotations, &index, &statements[0], "__STRING_3", None);
    assert_type_and_hint!(&annotations, &index, &statements[1], "__WSTRING_6", None);
    // AND we expect some newly created String-types
    assert_eq!(
        index.get_type_or_panic("__STRING_3"),
        &DataType {
            initial_value: None,
            name: "__STRING_3".into(),
            nature: crate::ast::TypeNature::Chars,
            information: DataTypeInformation::String {
                encoding: crate::typesystem::StringEncoding::Utf8,
                size: crate::typesystem::TypeSize::LiteralInteger(4)
            }
        }
    );
    assert_eq!(
        index.get_type_or_panic("__WSTRING_6"),
        &DataType {
            initial_value: None,
            name: "__WSTRING_6".into(),
            nature: crate::ast::TypeNature::Chars,
            information: DataTypeInformation::String {
                encoding: crate::typesystem::StringEncoding::Utf16,
                size: crate::typesystem::TypeSize::LiteralInteger(7)
            }
        }
    );
}

#[test]
fn int_literals_are_annotated() {
    let (unit, index) = index(
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
    let annotations = TypeAnnotator::visit_unit(&index, &unit);
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
    let (unit, index) = index(
        "PROGRAM PRG
                T#12.4d;
                TIME#-12m;
                TOD#00:00:12;
                TIME_OF_DAY#04:16:22;
				TIME_OF_DAY#04:16;
                DATE_AND_TIME#1984-10-01-16:40:22; 
                DT#2021-04-20-22:33:14; 
				DATE_AND_TIME#2000-01-01-20:15;
                DATE#1984-10-01; 
                D#2021-04-20; 
            END_PROGRAM",
    );
    let annotations = TypeAnnotator::visit_unit(&index, &unit);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec![
        "TIME",
        "TIME",
        "TIME_OF_DAY",
        "TIME_OF_DAY",
        "TIME_OF_DAY",
        "DATE_AND_TIME",
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
    let (unit, index) = index(
        "PROGRAM PRG
                3.1415;
                1.0;
            END_PROGRAM",
    );
    let annotations = TypeAnnotator::visit_unit(&index, &unit);
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
    let (unit, index) = index(
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
    let annotations = TypeAnnotator::visit_unit(&index, &unit);
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
    let (unit, index) = index(
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
    let annotations = TypeAnnotator::visit_unit(&index, &unit);
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
    let (unit, index) = index(
        "
            TYPE Color: (Green, Yellow, Red); END_TYPE

            PROGRAM PRG
                Color#Red;
            END_PROGRAM",
    );
    let annotations = TypeAnnotator::visit_unit(&index, &unit);
    let color_red = &unit.implementations[0].statements[0];

    assert_eq!(
        &DataTypeInformation::Enum {
            name: "Color".into(),
            elements: vec!["Green".into(), "Yellow".into(), "Red".into()],
            referenced_type: DINT_TYPE.into(),
        },
        annotations
            .get_type_or_void(color_red, &index)
            .get_type_information()
    );

    if let AstStatement::CastStatement { target, .. } = color_red {
        assert_eq!(
            &DataTypeInformation::Enum {
                name: "Color".into(),
                elements: vec!["Green".into(), "Yellow".into(), "Red".into()],
                referenced_type: DINT_TYPE.into(),
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
    let (unit, index) = index(
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
    let annotations = TypeAnnotator::visit_unit(&index, &unit);
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
    let (unit, mut index) = index(
        "
            TYPE Color: (red, green, blue); END_TYPE
            PROGRAM PRG
                Color#red;
                Color#green;
                Color#blue;
            END_PROGRAM",
    );
    let annotations = annotate(&unit, &mut index);
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
    let (unit, mut index) = index(
        "PROGRAM PRG
                (1,TRUE,3.1415);
            END_PROGRAM",
    );
    let annotations = annotate(&unit, &mut index);
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
    let (unit, mut index) = index(
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
    let annotations = annotate(&unit, &mut index);
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
    let (unit, mut index) = index(
        "
            VAR_GLOBAL CONSTANT
                a : ARRAY[0..2] OF BYTE := [1,2,3];
            END_VAR
            ",
    );

    let annotations = annotate(&unit, &mut index);

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

#[test]
fn expression_list_as_array_initilization_is_annotated_correctly() {
    // GIVEN two global variables beeing initialized with expression lists
    let (unit, mut index) = index(
        "
			VAR_GLOBAL
				a : ARRAY[0..2] OF INT := 1+1,2;
				b : ARRAY[0..2] OF STRING[3] := 'ABC','D';
			END_VAR
		",
    );

    // WHEN annotation is done
    let annotations = annotate(&unit, &mut index);

    // THEN for the first statement
    let a_init = unit.global_vars[0].variables[0]
        .initializer
        .as_ref()
        .unwrap();
    // all expressions should be annotated with the right type [INT]
    if let AstStatement::ExpressionList { expressions, .. } = a_init {
        for exp in expressions {
            if let Some(data_type) = annotations.get_type_hint(exp, &index) {
                let type_info = data_type.get_type_information();
                assert_eq!(
                    true,
                    matches!(type_info, DataTypeInformation::Integer { .. })
                )
            } else {
                unreachable!();
            }
        }
    } else {
        unreachable!();
    }

    // AND for the second statement
    let b_init = unit.global_vars[0].variables[1]
        .initializer
        .as_ref()
        .unwrap();
    // all expressions should be annotated with the right type [STRING]
    if let AstStatement::ExpressionList { expressions, .. } = b_init {
        for exp in expressions {
            let data_type = annotations.get_type_hint(exp, &index).unwrap();
            let type_info = data_type.get_type_information();
            assert_eq!(
                type_info,
                &DataTypeInformation::String {
                    encoding: StringEncoding::Utf8,
                    size: TypeSize::from_literal(4),
                }
            )
        }
    } else {
        unreachable!();
    }
}
