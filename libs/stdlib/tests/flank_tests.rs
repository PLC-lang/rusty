use common::compile_with_native;
use inkwell::context::Context;
use rusty::runner::run;

// Import common functionality into the integration tests
mod common;

use common::add_std;

// Rising and falling edge implementation is tested in the utils class, these are only wiring tests

#[repr(C)]
#[derive(Debug, Default)]
struct MainType {
    val: bool,
    edge: iec61131std::flanks::Trigger,
    out: bool,
}

#[test]
fn rising_edge_smoke_test() {
    let prg = r#"
        PROGRAM main
            VAR_INPUT
                val : BOOL;
            END_VAR
            VAR
                re : R_TRIG;
                out : BOOL;
            END_VAR
            re(CLK := val, Q => out);
        END_PROGRAM
    "#;
    let source = add_std!(prg, "flanks.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = MainType { val: true, ..Default::default() };
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.out);
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.out);
}

#[test]
fn falling_edge_smoke_test() {
    let prg = r#"
    PROGRAM main
        VAR_INPUT
            val : BOOL;
        END_VAR
        VAR
            re : F_TRIG;
            out : BOOL;
        END_VAR
        re(CLK := val, Q => out);
    END_PROGRAM
"#;
    let source = add_std!(prg, "flanks.st");
    let context: Context = Context::create();
    let exec_engine = compile_with_native(&context, source);
    let mut main_inst = MainType { val: true, ..Default::default() };
    main_inst.val = true;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(!main_inst.out);
    main_inst.val = false;
    run::<_, ()>(&exec_engine, "main", &mut main_inst);
    assert!(main_inst.out);
}
