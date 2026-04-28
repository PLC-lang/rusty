use common::{compile_and_load, get_includes};
use iec61131std::counters::CTDParams;
use iec61131std::counters::CTUDParams;
use iec61131std::counters::CTUParams;

// Import common functionality into the integration tests
mod common;

use plc::codegen::CodegenContext;

#[repr(C)]
#[derive(Default, Debug)]
struct CTUType<T> {
    fb: CTUParams<T>,
    q: bool,
    cv: T,
}

#[test]
fn ctu() {
    let prog = r#"
        PROGRAM main
        VAR
            ctu_inst : CTU;
            Q_res : BOOL;
            CV_res : INT;
        END_VAR
            // count up until Q_res true and then reset CV_res
            IF Q_res THEN
                ctu_inst(CU:= TRUE, R:= TRUE, PV:= INT#3, Q => Q_res, CV => CV_res);
            ELSE
                ctu_inst(CU:= TRUE, R:= FALSE, PV:= INT#3, Q => Q_res, CV => CV_res);
                // input CU evaluated by R_EDGE, this call will enable to count up again
                ctu_inst(CU:= FALSE, R:= FALSE, PV:= INT#3, Q => Q_res, CV => CV_res);
            END_IF
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTUType::<i16> { ..CTUType::default() };
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 3);
    // reset
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctu_int() {
    let prog = r#"
        PROGRAM main
        VAR
            ctu_inst : CTU_INT;
            Q_res : BOOL;
            CV_res : INT;
        END_VAR
            // count up until Q_res true and then reset CV_res
            IF Q_res THEN
                ctu_inst(CU:= TRUE, R:= TRUE, PV:= INT#3, Q => Q_res, CV => CV_res);
            ELSE
                ctu_inst(CU:= TRUE, R:= FALSE, PV:= INT#3, Q => Q_res, CV => CV_res);
                // input CU evaluated by R_EDGE, this call will enable to count up again
                ctu_inst(CU:= FALSE, R:= FALSE, PV:= INT#3, Q => Q_res, CV => CV_res);
            END_IF
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTUType::<i16> { ..CTUType::default() };
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 3);
    // reset
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctu_dint() {
    let prog = r#"
        PROGRAM main
        VAR
            ctu_inst : CTU_DINT;
            Q_res : BOOL;
            CV_res : DINT;
        END_VAR
            // count up until Q_res true and then reset CV_res
            IF Q_res THEN
                ctu_inst(CU:= TRUE, R:= TRUE, PV:= DINT#3, Q => Q_res, CV => CV_res);
            ELSE
                ctu_inst(CU:= TRUE, R:= FALSE, PV:= DINT#3, Q => Q_res, CV => CV_res);
                // input CU evaluated by R_EDGE, this call will enable to count up again
                ctu_inst(CU:= FALSE, R:= FALSE, PV:= DINT#3, Q => Q_res, CV => CV_res);
            END_IF
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTUType::<i32> { ..CTUType::default() };
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 3);
    // reset
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctu_udint() {
    let prog = r#"
        PROGRAM main
        VAR
            ctu_inst : CTU_UDINT;
            Q_res : BOOL;
            CV_res : UDINT;
        END_VAR
            // count up until Q_res true and then reset CV_res
            IF Q_res THEN
                ctu_inst(CU:= TRUE, R:= TRUE, PV:= UDINT#3, Q => Q_res, CV => CV_res);
            ELSE
                ctu_inst(CU:= TRUE, R:= FALSE, PV:= UDINT#3, Q => Q_res, CV => CV_res);
                // input CU evaluated by R_EDGE, this call will enable to count up again
                ctu_inst(CU:= FALSE, R:= FALSE, PV:= UDINT#3, Q => Q_res, CV => CV_res);
            END_IF
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTUType::<u32> { ..CTUType::default() };
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 3);
    // reset
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctu_lint() {
    let prog = r#"
        PROGRAM main
        VAR
            ctu_inst : CTU_LINT;
            Q_res : BOOL;
            CV_res : LINT;
        END_VAR
            // count up until Q_res true and then reset CV_res
            IF Q_res THEN
                ctu_inst(CU:= TRUE, R:= TRUE, PV:= LINT#3, Q => Q_res, CV => CV_res);
            ELSE
                ctu_inst(CU:= TRUE, R:= FALSE, PV:= LINT#3, Q => Q_res, CV => CV_res);
                // input CU evaluated by R_EDGE, this call will enable to count up again
                ctu_inst(CU:= FALSE, R:= FALSE, PV:= LINT#3, Q => Q_res, CV => CV_res);
            END_IF
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTUType::<i64> { ..CTUType::default() };
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 3);
    // reset
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctu_ulint() {
    let prog = r#"
        PROGRAM main
        VAR
            ctu_inst : CTU_ULINT;
            Q_res : BOOL;
            CV_res : ULINT;
        END_VAR
            // count up until Q_res true and then reset CV_res
            IF Q_res THEN
                ctu_inst(CU:= TRUE, R:= TRUE, PV:= ULINT#3, Q => Q_res, CV => CV_res);
            ELSE
                ctu_inst(CU:= TRUE, R:= FALSE, PV:= ULINT#3, Q => Q_res, CV => CV_res);
                // input CU evaluated by R_EDGE, this call will enable to count up again
                ctu_inst(CU:= FALSE, R:= FALSE, PV:= ULINT#3, Q => Q_res, CV => CV_res);
            END_IF
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTUType::<u64> { ..CTUType::default() };
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    // count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 3);
    // reset
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[repr(C)]
#[derive(Default, Debug)]
struct CTDType<T> {
    fb: CTDParams<T>,
    load: bool,
    q: bool,
    cv: T,
}

#[test]
fn ctd() {
    let prog = r#"
        PROGRAM main
        VAR
            ctd_inst : CTD;
            load : BOOL := TRUE;
            Q_res : BOOL;
            CV_res : INT;
        END_VAR
            // load PV value
            IF load THEN
                ctd_inst(CD:= TRUE, LD:= load, PV:= INT#3, Q => Q_res, CV => CV_res);
                load := FALSE;
            END_IF
            ctd_inst(CD:= TRUE, LD:= load, PV:= INT#3, Q => Q_res, CV => CV_res);
            // input CD evaluated by R_EDGE, this call will enable to count down again
            ctd_inst(CD:= FALSE, LD:= load, PV:= INT#3, Q => Q_res, CV => CV_res);
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTDType::<i16> { load: true, ..CTDType::default() };
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, -1);
}

#[test]
fn ctd_int() {
    let prog = r#"
        PROGRAM main
        VAR
            ctd_inst : CTD_INT;
            load : BOOL := TRUE;
            Q_res : BOOL;
            CV_res : INT;
        END_VAR
            // load PV value
            IF load THEN
                ctd_inst(CD:= TRUE, LD:= load, PV:= INT#3, Q => Q_res, CV => CV_res);
                load := FALSE;
            END_IF
            ctd_inst(CD:= TRUE, LD:= load, PV:= INT#3, Q => Q_res, CV => CV_res);
            // input CD evaluated by R_EDGE, this call will enable to count down again
            ctd_inst(CD:= FALSE, LD:= load, PV:= INT#3, Q => Q_res, CV => CV_res);
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTDType::<i16> { load: true, ..CTDType::default() };
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, -1);
}

#[test]
fn ctd_dint() {
    let prog = r#"
        PROGRAM main
        VAR
            ctd_inst : CTD_DINT;
            load : BOOL := TRUE;
            Q_res : BOOL;
            CV_res : DINT;
        END_VAR
            // load PV value
            IF load THEN
                ctd_inst(CD:= TRUE, LD:= load, PV:= DINT#3, Q => Q_res, CV => CV_res);
                load := FALSE;
            END_IF
            ctd_inst(CD:= TRUE, LD:= load, PV:= DINT#3, Q => Q_res, CV => CV_res);
            // input CD evaluated by R_EDGE, this call will enable to count down again
            ctd_inst(CD:= FALSE, LD:= load, PV:= DINT#3, Q => Q_res, CV => CV_res);
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTDType::<i32> { load: true, ..CTDType::default() };
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, -1);
}

#[test]
fn ctd_udint() {
    let prog = r#"
        PROGRAM main
        VAR
            ctd_inst : CTD_UDINT;
            load : BOOL := TRUE;
            Q_res : BOOL;
            CV_res : UDINT;
        END_VAR
            // load PV value
            IF load THEN
                ctd_inst(CD:= TRUE, LD:= load, PV:= UDINT#3, Q => Q_res, CV => CV_res);
                load := FALSE;
            END_IF
            ctd_inst(CD:= TRUE, LD:= load, PV:= UDINT#3, Q => Q_res, CV => CV_res);
            // input CD evaluated by R_EDGE, this call will enable to count down again
            ctd_inst(CD:= FALSE, LD:= load, PV:= UDINT#3, Q => Q_res, CV => CV_res);
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTDType::<u32> { load: true, ..CTDType::default() };
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctd_lint() {
    let prog = r#"
        PROGRAM main
        VAR
            ctd_inst : CTD_LINT;
            load : BOOL := TRUE;
            Q_res : BOOL;
            CV_res : LINT;
        END_VAR
            // load PV value
            IF load THEN
                ctd_inst(CD:= TRUE, LD:= load, PV:= LINT#3, Q => Q_res, CV => CV_res);
                load := FALSE;
            END_IF
            ctd_inst(CD:= TRUE, LD:= load, PV:= LINT#3, Q => Q_res, CV => CV_res);
            // input CD evaluated by R_EDGE, this call will enable to count down again
            ctd_inst(CD:= FALSE, LD:= load, PV:= LINT#3, Q => Q_res, CV => CV_res);
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTDType::<i64> { load: true, ..CTDType::default() };
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, -1);
}

#[test]
fn ctd_ulint() {
    let prog = r#"
        PROGRAM main
        VAR
            ctd_inst : CTD_ULINT;
            load : BOOL := TRUE;
            Q_res : BOOL;
            CV_res : ULINT;
        END_VAR
            // load PV value
            IF load THEN
                ctd_inst(CD:= TRUE, LD:= load, PV:= ULINT#3, Q => Q_res, CV => CV_res);
                load := FALSE;
            END_IF
            ctd_inst(CD:= TRUE, LD:= load, PV:= ULINT#3, Q => Q_res, CV => CV_res);
            // input CD evaluated by R_EDGE, this call will enable to count down again
            ctd_inst(CD:= FALSE, LD:= load, PV:= ULINT#3, Q => Q_res, CV => CV_res);
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTDType::<u64> { load: true, ..CTDType::default() };
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 2);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.q);
    assert_eq!(main_inst.cv, 1);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
    // count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.q);
    assert_eq!(main_inst.cv, 0);
}

#[repr(C)]
#[derive(Default, Debug)]
struct CTUDType<T> {
    fb: CTUDParams<T>,
    load: bool,
    qu: bool,
    qd: bool,
    cv: T,
    i: u8,
}

#[test]
fn ctud() {
    let prog = r#"
        PROGRAM main
        VAR
            ctud_inst : CTUD;
            load : BOOL := TRUE;
            QU_res : BOOL;
            QD_res : BOOL;
            CV_res : INT;
            i : SINT;
        END_VAR
            // 1st call, load PV value
            IF load THEN
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= FALSE, LD:= TRUE, PV:= INT#1, QU => QU_res, QD => QD_res, CV => CV_res);
                load := FALSE;
            END_IF

            // 2nd call, CU/CD both true, nothing should happen
            IF i = 1 THEN
                ctud_inst(CU:= TRUE, CD:= TRUE, R:= FALSE, LD:= FALSE, PV:= INT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 3rd call, count down
            IF i = 2 THEN
                // input CD evaluated by R_EDGE, this call will enable count down again
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= FALSE, LD:= FALSE, PV:= INT#1, QU => QU_res, QD => QD_res, CV => CV_res);
                ctud_inst(CU:= FALSE, CD:= TRUE, R:= FALSE, LD:= FALSE, PV:= INT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 4th call, count up
            IF i = 3 THEN
                // input CU evaluated by R_EDGE, third call enabled count up again
                ctud_inst(CU:= TRUE, CD:= FALSE, R:= FALSE, LD:= FALSE, PV:= INT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 5th call, reset
            IF i = 4 THEN
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= TRUE, LD:= FALSE, PV:= INT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF
            i := i + 1;
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTUDType::<i16> { load: true, ..CTUDType::default() };
    // 1st call, load PV value
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 2nd call, CU/CD both true, nothing should happen
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 3rd call, count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.qu);
    assert!(main_inst.qd);
    assert_eq!(main_inst.cv, 0);
    // 4th call, count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 5th call, reset
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.qu);
    assert!(main_inst.qd);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctud_int() {
    let prog = r#"
        PROGRAM main
        VAR
            ctud_inst : CTUD_INT;
            load : BOOL := TRUE;
            QU_res : BOOL;
            QD_res : BOOL;
            CV_res : INT;
            i : SINT;
        END_VAR
            // 1st call, load PV value
            IF load THEN
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= FALSE, LD:= TRUE, PV:= INT#1, QU => QU_res, QD => QD_res, CV => CV_res);
                load := FALSE;
            END_IF

            // 2nd call, CU/CD both true, nothing should happen
            IF i = 1 THEN
                ctud_inst(CU:= TRUE, CD:= TRUE, R:= FALSE, LD:= FALSE, PV:= INT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 3rd call, count down
            IF i = 2 THEN
                // input CD evaluated by R_EDGE, this call will enable count down again
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= FALSE, LD:= FALSE, PV:= INT#1, QU => QU_res, QD => QD_res, CV => CV_res);
                ctud_inst(CU:= FALSE, CD:= TRUE, R:= FALSE, LD:= FALSE, PV:= INT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 4th call, count up
            IF i = 3 THEN
                // input CU evaluated by R_EDGE, third call enabled count up again
                ctud_inst(CU:= TRUE, CD:= FALSE, R:= FALSE, LD:= FALSE, PV:= INT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 5th call, reset
            IF i = 4 THEN
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= TRUE, LD:= FALSE, PV:= INT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF
            i := i + 1;
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTUDType::<i16> { load: true, ..CTUDType::default() };
    // 1st call, load PV value
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 2nd call, CU/CD both true, nothing should happen
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 3rd call, count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.qu);
    assert!(main_inst.qd);
    assert_eq!(main_inst.cv, 0);
    // 4th call, count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 5th call, reset
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.qu);
    assert!(main_inst.qd);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctud_dint() {
    let prog = r#"
        PROGRAM main
        VAR
            ctud_inst : CTUD_DINT;
            load : BOOL := TRUE;
            QU_res : BOOL;
            QD_res : BOOL;
            CV_res : DINT;
            i : SINT;
        END_VAR
            // 1st call, load PV value
            IF load THEN
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= FALSE, LD:= TRUE, PV:= DINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
                load := FALSE;
            END_IF

            // 2nd call, CU/CD both true, nothing should happen
            IF i = 1 THEN
                ctud_inst(CU:= TRUE, CD:= TRUE, R:= FALSE, LD:= FALSE, PV:= DINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 3rd call, count down
            IF i = 2 THEN
                // input CD evaluated by R_EDGE, this call will enable count down again
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= FALSE, LD:= FALSE, PV:= DINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
                ctud_inst(CU:= FALSE, CD:= TRUE, R:= FALSE, LD:= FALSE, PV:= DINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 4th call, count up
            IF i = 3 THEN
                // input CU evaluated by R_EDGE, third call enabled count up again
                ctud_inst(CU:= TRUE, CD:= FALSE, R:= FALSE, LD:= FALSE, PV:= DINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 5th call, reset
            IF i = 4 THEN
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= TRUE, LD:= FALSE, PV:= DINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF
            i := i + 1;
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTUDType::<i32> { load: true, ..CTUDType::default() };
    // 1st call, load PV value
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 2nd call, CU/CD both true, nothing should happen
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 3rd call, count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.qu);
    assert!(main_inst.qd);
    assert_eq!(main_inst.cv, 0);
    // 4th call, count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 5th call, reset
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.qu);
    assert!(main_inst.qd);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctud_udint() {
    let prog = r#"
        PROGRAM main
        VAR
            ctud_inst : CTUD_UDINT;
            load : BOOL := TRUE;
            QU_res : BOOL;
            QD_res : BOOL;
            CV_res : UDINT;
            i : SINT;
        END_VAR
            // 1st call, load PV value
            IF load THEN
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= FALSE, LD:= TRUE, PV:= UDINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
                load := FALSE;
            END_IF

            // 2nd call, CU/CD both true, nothing should happen
            IF i = 1 THEN
                ctud_inst(CU:= TRUE, CD:= TRUE, R:= FALSE, LD:= FALSE, PV:= UDINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 3rd call, count down
            IF i = 2 THEN
                // input CD evaluated by R_EDGE, this call will enable count down again
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= FALSE, LD:= FALSE, PV:= UDINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
                ctud_inst(CU:= FALSE, CD:= TRUE, R:= FALSE, LD:= FALSE, PV:= UDINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 4th call, count up
            IF i = 3 THEN
                // input CU evaluated by R_EDGE, third call enabled count up again
                ctud_inst(CU:= TRUE, CD:= FALSE, R:= FALSE, LD:= FALSE, PV:= UDINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 5th call, reset
            IF i = 4 THEN
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= TRUE, LD:= FALSE, PV:= UDINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF
            i := i + 1;
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTUDType::<u32> { load: true, ..CTUDType::default() };
    // 1st call, load PV value
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 2nd call, CU/CD both true, nothing should happen
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 3rd call, count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.qu);
    assert!(main_inst.qd);
    assert_eq!(main_inst.cv, 0);
    // 4th call, count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 5th call, reset
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.qu);
    assert!(main_inst.qd);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctud_lint() {
    let prog = r#"
        PROGRAM main
        VAR
            ctud_inst : CTUD_LINT;
            load : BOOL := TRUE;
            QU_res : BOOL;
            QD_res : BOOL;
            CV_res : LINT;
            i : SINT;
        END_VAR
            // 1st call, load PV value
            IF load THEN
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= FALSE, LD:= TRUE, PV:= LINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
                load := FALSE;
            END_IF

            // 2nd call, CU/CD both true, nothing should happen
            IF i = 1 THEN
                ctud_inst(CU:= TRUE, CD:= TRUE, R:= FALSE, LD:= FALSE, PV:= LINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 3rd call, count down
            IF i = 2 THEN
                // input CD evaluated by R_EDGE, this call will enable count down again
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= FALSE, LD:= FALSE, PV:= LINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
                ctud_inst(CU:= FALSE, CD:= TRUE, R:= FALSE, LD:= FALSE, PV:= LINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 4th call, count up
            IF i = 3 THEN
                // input CU evaluated by R_EDGE, third call enabled count up again
                ctud_inst(CU:= TRUE, CD:= FALSE, R:= FALSE, LD:= FALSE, PV:= LINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 5th call, reset
            IF i = 4 THEN
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= TRUE, LD:= FALSE, PV:= LINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF
            i := i + 1;
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTUDType::<i64> { load: true, ..CTUDType::default() };
    // 1st call, load PV value
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 2nd call, CU/CD both true, nothing should happen
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 3rd call, count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.qu);
    assert!(main_inst.qd);
    assert_eq!(main_inst.cv, 0);
    // 4th call, count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 5th call, reset
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.qu);
    assert!(main_inst.qd);
    assert_eq!(main_inst.cv, 0);
}

#[test]
fn ctud_ulint() {
    let prog = r#"
        PROGRAM main
        VAR
            ctud_inst : CTUD_ULINT;
            load : BOOL := TRUE;
            QU_res : BOOL;
            QD_res : BOOL;
            CV_res : ULINT;
            i : SINT;
        END_VAR
            // 1st call, load PV value
            IF load THEN
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= FALSE, LD:= TRUE, PV:= ULINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
                load := FALSE;
            END_IF

            // 2nd call, CU/CD both true, nothing should happen
            IF i = 1 THEN
                ctud_inst(CU:= TRUE, CD:= TRUE, R:= FALSE, LD:= FALSE, PV:= ULINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 3rd call, count down
            IF i = 2 THEN
                // input CD evaluated by R_EDGE, this call will enable count down again
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= FALSE, LD:= FALSE, PV:= ULINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
                ctud_inst(CU:= FALSE, CD:= TRUE, R:= FALSE, LD:= FALSE, PV:= ULINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 4th call, count up
            IF i = 3 THEN
                // input CU evaluated by R_EDGE, third call enabled count up again
                ctud_inst(CU:= TRUE, CD:= FALSE, R:= FALSE, LD:= FALSE, PV:= ULINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF

            // 5th call, reset
            IF i = 4 THEN
                ctud_inst(CU:= FALSE, CD:= FALSE, R:= TRUE, LD:= FALSE, PV:= ULINT#1, QU => QU_res, QD => QD_res, CV => CV_res);
            END_IF
            i := i + 1;
        END_PROGRAM
    "#;

    let source = vec![prog.into()];
    let includes = get_includes(&["counters.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, source, includes);
    let mut main_inst = CTUDType::<u64> { load: true, ..CTUDType::default() };
    // 1st call, load PV value
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 2nd call, CU/CD both true, nothing should happen
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 3rd call, count down
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.qu);
    assert!(main_inst.qd);
    assert_eq!(main_inst.cv, 0);
    // 4th call, count up
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.qu);
    assert!(!main_inst.qd);
    assert_eq!(main_inst.cv, 1);
    // 5th call, reset
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.qu);
    assert!(main_inst.qd);
    assert_eq!(main_inst.cv, 0);
}
