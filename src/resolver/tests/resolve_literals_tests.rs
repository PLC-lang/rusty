use crate::{resolver::{tests::{annotate, parse}}};


#[test]
fn bool_literals_are_annotated() {
    let (unit, index) = parse(
        "PROGRAM PRG
                TRUE;
                FALSE;
            END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;

    assert_eq!(
        Some(&"BOOL".to_string()),
        annotations.type_map.get(&statements[0].get_id())
    );
    assert_eq!(
        Some(&"BOOL".to_string()),
        annotations.type_map.get(&statements[1].get_id())
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
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;


    for s in statements.iter() {
        assert_eq!(
            Some(&"STRING".to_string()),
            annotations.type_map.get(&s.get_id())
        );
    }
}

#[test]
fn int_literals_are_annotated() {
    let (unit, index) = parse(
        "PROGRAM PRG
                1;
                1000;
                1000000;
                10000000000;
            END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;
    
    let expected_types = vec!["BYTE", "UINT", "UDINT", "ULINT", "DATE_AND_TIME", "DATE_AND_TIME", "DATE", "DATE"];
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
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["TIME", "TIME", "TIME_OF_DAY", "TIME_OF_DAY", "DATE_AND_TIME", "DATE_AND_TIME", "DATE", "DATE"];
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
fn real_literals_are_annotated() {
    let (unit, index) = parse(
        "PROGRAM PRG
                3.1415;
                1.0;
            END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["REAL", "REAL"];
    for (i, s) in statements.iter().enumerate() {
        assert_eq!(
            Some(&expected_types[i].to_string()),
            annotations.type_map.get(&s.get_id()),
            "{:#?}",
            s
        );
    }
}