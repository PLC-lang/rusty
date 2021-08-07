use crate::{resolver::{tests::{annotate, parse}}};


#[test]
fn binary_expressions_resolves_types() {
    let (unit, index) = parse(
        "PROGRAM PRG
            1 + 2;
            1 + 2000;
            2000 + 1;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["BYTE", "UINT", "UINT"];
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
fn binary_expressions_resolves_types_with_floats() {
    let (unit, index) = parse(
        "PROGRAM PRG
            1 + 2.2;
            1.1 + 2000;
            2000.0 + 1.0;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["REAL", "REAL", "REAL"];
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
fn local_variables_resolves_types() {
    let (unit, index) = parse(
        "PROGRAM PRG
            VAR
                b : BYTE;
                w : WORD;
                dw : DWORD;
                lw : LWORD;
                si : SINT;
                usi : USINT;
                i : INT;
                ui : UINT;
                di : DINT;
                udi : UDINT;
                li : LINT;
                uli : ULINT;
            END_VAR

            b;
            w;
            dw;
            lw;
            si;
            usi;
            i;
            ui;
            di;
            udi;
            li;
            uli;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["BYTE", "WORD", "DWORD", "LWORD", "SINT", "USINT", "INT", "UINT", "DINT", "UDINT", "LINT", "ULINT"];
    let nothing = "-".to_string();
    let type_names : Vec<&String> = statements.iter().map(|s| 
            annotations.type_map.get(&s.get_id()).unwrap_or(&nothing)).collect();

    assert_eq!(format!("{:?}", expected_types),
                format!("{:?}", type_names));
}

#[test]
fn global_resolves_types() {
    let (unit, index) = parse(
        "
        VAR_GLOBAL
            b : BYTE;
            w : WORD;
            dw : DWORD;
            lw : LWORD;
            si : SINT;
            usi : USINT;
            i : INT;
            ui : UINT;
            di : DINT;
            udi : UDINT;
            li : LINT;
            uli : ULINT;
        END_VAR
        
        PROGRAM PRG
            b;
            w;
            dw;
            lw;
            si;
            usi;
            i;
            ui;
            di;
            udi;
            li;
            uli;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["BYTE", "WORD", "DWORD", "LWORD", "SINT", "USINT", "INT", "UINT", "DINT", "UDINT", "LINT", "ULINT"];
    let nothing = "-".to_string();
    let type_names : Vec<&String> = statements.iter().map(|s| 
            annotations.type_map.get(&s.get_id()).unwrap_or(&nothing)).collect();

    assert_eq!(format!("{:?}", expected_types),
                format!("{:?}", type_names));
}


#[test]
fn resolve_binary_expressions() {
    let (unit, index) = parse(
        "
        VAR_GLOBAL
            b : BYTE;
            w : WORD;
            dw : DWORD;
            lw : LWORD;
            si : SINT;
            usi : USINT;
            i : INT;
            ui : UINT;
            di : DINT;
            udi : UDINT;
            li : LINT;
            uli : ULINT;
        END_VAR
        
        PROGRAM PRG
            b + b;
            b + w;
            b + dw;
            b + lw;
            b + si;
            b + usi;
            b + i;
            b + ui;
            b + di;
            b + udi;
            b + li;
            b + uli;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["BYTE", "WORD", "DWORD", "LWORD", "SINT", "BYTE", "INT", "UINT", "DINT", "UDINT", "LINT", "ULINT"];
    let nothing = "-".to_string();
    let type_names : Vec<&String> = statements.iter().map(|s| 
            annotations.type_map.get(&s.get_id()).unwrap_or(&nothing)).collect();

    assert_eq!(format!("{:?}", expected_types),
                format!("{:?}", type_names));
}


#[test]
fn complex_expressions_resolve_types() {
    let (unit, index) = parse(
        "PROGRAM PRG
            VAR
                b : BYTE;
                w : WORD;
                dw : DWORD;
                lw : LWORD;
                si : SINT;
                usi : USINT;
                i : INT;
                ui : UINT;
                di : DINT;
                udi : UDINT;
                li : LINT;
                uli : ULINT;
                r : REAL;
            END_VAR

            b + w * di + li;
            b + w + di;
            b + w * di + r;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[0].statements;

    let expected_types = vec!["LINT", "DINT", "REAL"];
    let nothing = "-".to_string();
    let type_names : Vec<&String> = statements.iter().map(|s| 
            annotations.type_map.get(&s.get_id()).unwrap_or(&nothing)).collect();

    assert_eq!(format!("{:?}", expected_types),
                format!("{:?}", type_names));
}


#[test]
fn qualified_expressions_resolve_types() {
    let (unit, index) = parse(
        "
         PROGRAM Other
            VAR_INPUT
                b : BYTE;
                w : WORD;
                dw : DWORD;
                lw : LWORD;
            END_VAR
        END_PROGRAM   

        PROGRAM PRG
            Other.b;
            Other.w;
            Other.dw;
            Other.lw;
            Other.b + Other.w;
            Other.b + Other.w + Other.dw;
            Other.b + Other.w + Other.dw + Other.lw;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[1].statements;

    let expected_types = vec!["BYTE", "WORD", "DWORD", "LWORD", "WORD", "DWORD", "LWORD"];
    let nothing = "-".to_string();
    let type_names : Vec<&String> = statements.iter().map(|s| 
            annotations.type_map.get(&s.get_id()).unwrap_or(&nothing)).collect();

    assert_eq!(format!("{:?}", expected_types),
                format!("{:?}", type_names));
}


#[test]
fn pou_expressions_resolve_types() {
    let (unit, index) = parse(
        "
        PROGRAM OtherPrg
        END_PROGRAM   

        FUNCTION OtherFunc
        END_FUNCTION

        FUNCTION_BLOCK OtherFuncBlock
        END_FUNCTION_BLOCK

        PROGRAM PRG
            OtherPrg;
            OtherFunc;
            OtherFuncBlock;
        END_PROGRAM",
    );
    let annotations = annotate(&unit, index);
    let statements = &unit.implementations[3].statements;

    let expected_types = vec!["OtherPrg", "OtherFunc", "OtherFuncBlock"];
    let nothing = "-".to_string();
    let type_names : Vec<&String> = statements.iter().map(|s| 
            annotations.type_map.get(&s.get_id()).unwrap_or(&nothing)).collect();

    assert_eq!(format!("{:?}", expected_types),
                format!("{:?}", type_names));
}


