use crate::resolver::tests::{annotate, parse};

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
                "xyz";
            END_PROGRAM"#,
    );
    let annotations = annotate(&unit, &index);
    let statements = &unit.implementations[0].statements;

    for s in statements.iter() {
        assert_eq!("STRING", annotations.get_type_or_void(s, &index).get_name());
    }
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
