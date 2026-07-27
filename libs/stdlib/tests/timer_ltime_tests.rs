use std::time::Duration;

use common::{compile_and_load, get_includes};
use iec61131std::timers::{LTime, TimerParamsLTime};

// Import common functionality into the integration tests
mod common;

use plc::codegen::CodegenContext;

#[repr(C)]
#[derive(Default, Debug)]
struct MainTypeLTime {
    value: bool,
    tp_out: bool,
    tp_et: LTime,
    tp_inst: TimerParamsLTime,
}

#[test]
fn tp_ltime_counts_nanoseconds() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_out  : BOOL;
                tp_et   : LTIME;
                tp_inst : TP_LTIME;
            END_VAR
            tp_inst(IN := value, PT := LT#100ns, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainTypeLTime { value: true, ..MainTypeLTime::default() };

    // On first call, output is high and elapsed time starts at 0.
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);

    // Sub-millisecond elapsed time is preserved for LTIME.
    assert!(module.mock_time_advance_ns(25));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 25);

    // At the preset boundary, ET is clamped to PT and Q is still true.
    assert!(module.mock_time_advance_ns(75));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 100);

    // Once PT is exceeded, Q goes low and ET stays at PT.
    assert!(module.mock_time_advance_ns(1));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 100);
}

#[test]
fn ton_ltime_waits_then_switches_output() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_out  : BOOL;
                tp_et   : LTIME;
                tp_inst : TON_LTIME;
            END_VAR
            tp_inst(IN := value, PT := LT#30ns, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainTypeLTime { value: true, ..MainTypeLTime::default() };

    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);

    assert!(module.mock_time_advance_ns(20));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 20);

    assert!(module.mock_time_advance_ns(10));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 30);

    assert!(module.mock_time_advance_ns(1));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 30);

    main_inst.value = false;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
}

#[test]
fn tof_ltime_restarts_when_input_returns_true() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_out  : BOOL;
                tp_et   : LTIME;
                tp_inst : TOF_LTIME;
            END_VAR
            tp_inst(IN := value, PT := LT#40ns, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainTypeLTime { value: true, ..MainTypeLTime::default() };

    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);

    main_inst.value = false;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);

    assert!(module.mock_time_advance_ns(25));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 25);

    // A rising edge while counting should reset ET and keep Q high.
    main_inst.value = true;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);

    // Falling edge starts counting again from zero.
    main_inst.value = false;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);

    assert!(module.mock_time_advance_ns(41));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 40);
}

#[test]
fn ton_ltime_large_preset_uses_nanoseconds() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_out  : BOOL;
                tp_et   : LTIME;
                tp_inst : TON_LTIME;
            END_VAR
            tp_inst(IN := value, PT := LT#10ms, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainTypeLTime { value: true, ..MainTypeLTime::default() };

    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);

    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 5_000_000);

    assert!(module.mock_time_advance_ns(Duration::from_millis(6).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 10_000_000);
}
