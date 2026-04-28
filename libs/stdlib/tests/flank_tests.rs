use common::compile_and_load;

// Import common functionality into the integration tests
mod common;

use plc::codegen::CodegenContext;

use crate::common::get_includes;

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
    let source = vec![prg.into()];
    let includes = get_includes(&["flanks.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = MainType { val: true, ..Default::default() };
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.out);
    module.run::<_, ()>("main", &mut main_inst);
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
    let source = vec![prg.into()];
    let includes = get_includes(&["flanks.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = MainType { val: true, ..Default::default() };
    main_inst.val = true;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.out);
    main_inst.val = false;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.out);
}
