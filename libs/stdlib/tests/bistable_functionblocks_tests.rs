use common::compile_with_native;
use iec61131std::bistable_functionblocks::SetResetParams;

// Import common functionality into the integration tests
mod common;

use common::add_std;
use plc::codegen::CodegenContext;

#[repr(C)]
#[derive(Default, Debug)]
struct MainType {
    fb: SetResetParams,
    t_t_t: bool,
    t_t_f: bool,
    t_f_t: bool,
    t_f_f: bool,
    f_t_t: bool,
    f_t_f: bool,
    f_f_t: bool,
    f_f_f: bool,
}

#[test]
fn sr() {
    let prog = r#"
        PROGRAM main
        VAR
            sr_inst : SR;
            t_t_t  : BOOL;
            t_t_f  : BOOL;
            t_f_t  : BOOL;
            t_f_f  : BOOL;
            f_t_t  : BOOL;
            f_t_f  : BOOL;
            f_f_t  : BOOL;
            f_f_f  : BOOL;
        END_VAR
            sr_inst(SET1 := TRUE, RESET := TRUE, Q1 => t_t_f); (* Q is in default state, S and R are asserted -> Q goes high *)
            sr_inst(SET1 := FALSE, RESET := TRUE, Q1 => f_t_t); (* Q is high, R is asserted -> Q goes low *)
            sr_inst(SET1 := FALSE, RESET := FALSE, Q1 => f_f_f); (* Q is low, neither S nor R are asserted -> Q stays low*)
            sr_inst(SET1 := TRUE, RESET := FALSE, Q1 => t_f_f); (* Q is low, S is asserted -> Q goes high *)
            sr_inst(SET1 := TRUE, RESET := TRUE, Q1 => t_t_t); (* Q is high, S and R are asserted -> Q stays high *)
            sr_inst(SET1 := TRUE, RESET := FALSE, Q1 => t_f_t); (* Q is high, S is asserted -> Q stays high *)
            sr_inst(SET1 := FALSE, RESET := FALSE, Q1 => f_f_t); (* Q is high, neither S nor R are asserted -> Q stays high *)
            sr_inst(SET1 := FALSE, RESET := TRUE, Q1 => f_t_f); (* reset *)
            sr_inst(SET1 := FALSE, RESET := TRUE, Q1 => f_t_f); (* Q is low, R is asserted -> Q stays low *)
        END_PROGRAM
    "#;

    let source = add_std!(prog, "bistable_functionblocks.st");
    let context = CodegenContext::create();
    let module = compile_with_native(&context, source);
    let mut main_inst = MainType { ..MainType::default() };
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.t_t_f);
    assert!(!main_inst.f_t_t);
    assert!(!main_inst.f_f_f);
    assert!(main_inst.t_f_f);
    assert!(main_inst.t_t_t);
    assert!(main_inst.t_f_t);
    assert!(main_inst.f_f_t);
    assert!(!main_inst.f_t_f);
}

#[test]
fn rs() {
    let prog = r#"
        PROGRAM main
        VAR
            rs_inst : RS;
            t_t_t  : BOOL;
            t_t_f  : BOOL;
            t_f_t  : BOOL;
            t_f_f  : BOOL;
            f_t_t  : BOOL;
            f_t_f  : BOOL;
            f_f_t  : BOOL;
            f_f_f  : BOOL;
        END_VAR
            rs_inst(SET0 := TRUE, RESET1 := TRUE, Q1 => t_t_f); (* Q is in default state, S and R are asserted -> Q stays low *)
            rs_inst(SET0 := FALSE, RESET1 := FALSE, Q1 => f_f_f); (* Q is low, neither S nor R are asserted -> Q stays low*)
            rs_inst(SET0 := TRUE, RESET1 := FALSE, Q1 => t_f_f); (* Q is low, S is asserted -> Q goes high *)
            rs_inst(SET0 := FALSE, RESET1 := TRUE, Q1 => f_t_t); (* Q is high, R is asserted -> Q goes low *)
            rs_inst(SET0 := FALSE, RESET1 := TRUE, Q1 => f_t_f); (* Q is low, R is asserted -> Q stays low *)
            rs_inst(SET0 := TRUE, RESET1 := FALSE, Q1 => t_f_t); (* set *)
            rs_inst(SET0 := TRUE, RESET1 := FALSE, Q1 => t_f_t); (* Q is high, S is asserted -> Q stays high *)
            rs_inst(SET0 := FALSE, RESET1 := FALSE, Q1 => f_f_t); (* Q is high, neither S nor R are asserted -> Q stays high *)
            rs_inst(SET0 := TRUE, RESET1 := TRUE, Q1 => t_t_t); (* Q is high, S and R are asserted -> Q goes low *)
        END_PROGRAM
    "#;

    let source = add_std!(prog, "bistable_functionblocks.st");
    let context = CodegenContext::create();
    let module = compile_with_native(&context, source);
    let mut main_inst = MainType { ..MainType::default() };
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.t_t_f);
    assert!(!main_inst.f_f_f);
    assert!(main_inst.t_f_f);
    assert!(!main_inst.f_t_t);
    assert!(!main_inst.f_t_f);
    assert!(main_inst.t_f_t);
    assert!(main_inst.f_f_t);
    assert!(!main_inst.t_t_t);
}
