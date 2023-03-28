use rusty::runner::compile_and_run;

#[test]
fn vla() {
    #[derive(Default)]
    struct MainType {
        i: i32,
    }
    let mut main_type = MainType::default();

    let src = r#"
    FUNCTION main : DINT
    VAR
        i : DINT;
    END_VAR
    VAR_TEMP
        arr : ARRAY[0..1] OF DINT := (5, 6);
    END_VAR

        main := foo(arr);
    END_PROGRAM

    FUNCTION foo : DINT
    VAR_INPUT
        vla : ARRAY[ * ] OF DINT;
    END_VAR       

        foo := vla[0];
    END_FUNCTION
    "#;

    let _: i32 = compile_and_run(src.to_string(), &mut main_type);
    assert_eq!(6, main_type.i)
}
