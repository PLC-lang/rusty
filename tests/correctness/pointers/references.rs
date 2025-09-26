// Copyright (c) 2021 Daniel Schwenniger

use crate::compile_and_run;

#[test]
fn multiple_pointer_dereference() {
    struct MainType {
        a: u8,
        b: u8,
    }

    let src = r#"
        PROGRAM main
        VAR
            a: BYTE;
            b: BYTE;
        END_VAR
        VAR_TEMP
            c: REF_TO BYTE;
            d: REF_TO REF_TO BYTE;
            e: BYTE;
        END_VAR
            c := REF(a);
            d := REF(c);
            b := d^^;
            e := (d^)^;
            a := e + 16#01;
        END_PROGRAM
    "#;

    let mut maintype = MainType { a: 0xFF, b: 0 };
    let _: i32 = compile_and_run(src, &mut maintype);

    assert_eq!(0x00_u8, maintype.a);
    assert_eq!(0xFF_u8, maintype.b);
}
