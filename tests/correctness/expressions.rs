// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use super::super::*;

#[derive(Default)]
#[repr(C)]
struct MainType {
    a: f32,
    b: f32,
    c: f64,
    d: f64,
}

#[test]
fn real_negation() {
    let function = "
            FUNCTION main : DINT
            VAR
                a,b : REAL;
                c,d : LREAL;
            END_VAR
                a := -2.0;
                b := -a;
                c := -3.0;
                d := -c;
            END_FUNCTION
    ";
    let mut maintype = MainType::default();
    compile_and_run::<_, i32>(function.to_string(), &mut maintype);
    assert_eq!(-2.0, maintype.a);
    assert_eq!(2.0, maintype.b);
    assert_eq!(-3.0, maintype.c);
    assert_eq!(3.0, maintype.d);
}
