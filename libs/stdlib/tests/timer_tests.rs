use std::time::Duration;

use common::{compile_and_load, get_includes};
use iec61131std::timers::TimerParams;

// Import common functionality into the integration tests
mod common;

use plc::codegen::CodegenContext;
/*
 * ┌───────────────────────────────────────────────────────┐   ┌────────────────────────────────────────────────────────────┐     ┌────────────────────────────────────────────────────────────┐
 * │ TP                                                    │   │ TON                                                        │     │ TOF                                                        │
 * │                                                       │   │                                                            │     │                                                            │
 * │                                                       │   │                                                            │     │                                                            │
 * │                                                       │   │                                                            │     │                                                            │
 * │                ┌───────────────┐     ┌┐  ┌┐           │   │                ┌──────────┐    ┌───┐   ┌──────────┐        │     │                ┌──────────┐      ┌───┐     ┌──────┐        │
 * │                │               │     ││  ││           │   │                │          │    │   │   │          │        │     │                │          │      │   │     │      │        │
 * │       IN       │               │     ││  ││           │   │       IN       │          │    │   │   │          │        │     │       IN       │          │      │   │     │      │        │
 * │          ──────┘               └─────┴┴──┴┴───        │   │          ──────┘          └────┘   └───┘          └────    │     │          ──────┘          └──────┘   └─────┘      └────    │
 * │               t0               t1    t2  t3           │   │               t0          t1   t2  t3  t4         t5       │     │               t0          t1     t2  t3    t4     t5       │
 * │                                                       │   │                                                            │     │                                                            │
 * │                                                       │   │                                                            │     │                                                            │
 * │                ┌─────┐               ┌─────┐          │   │                      ┌────┐                  ┌────┐        │     │                ┌─────────────┐   ┌───────────────────┐     │
 * │                │     │               │     │          │   │                      │    │                  │    │        │     │                │             │   │                   │     │
 * │       Q        │     │               │     │          │   │       Q              │    │                  │    │        │     │       Q        │             │   │                   │     │
 * │          ──────┘     └───────────────┘     └─────     │   │          ────────────┘    └──────────────────┘    └─────   │     │          ──────┘             └───┘                   └──   │
 * │               t0     t0+TP          t2     t2+TP      │   │                   t0+TP   t1               t4+TP  t5       │     │                t0         t1+TP   t2              t5+TP    │
 * │                                                       │   │                                                            │     │                                                            │
 * │                                                       │   │                                                            │     │                                                            │
 * │                                                       │   │                                                            │     │                                                            │
 * │                                                       │   │                                                            │     │                                                            │
 * │                                                       │   │                                                            │     │                                                            │
 * │       PT                                              │   │       PT                                                   │     │                                                            │
 * │       │              ──────────┐                      │   │       │              /───┐                                 │     │                                                            │
 * │       │             /          │                      │   │       │             /    │                   /────┐        │     │       PT                                                   │
 * │       │            /           │          /│          │   │       │            /     │        /│        /     │        │     │         │                    /───┐     /             /─────┤
 * │ ET    │           /            │         / │          │   │ ET    │           /      │       / │       /      │        │     │ ET      │                   /    │    /│            /      │
 * │       │          /             │        /  │          │   │       │          /       │      /  │      /       │        │     │         │                  /     │   / │           /       │
 * │       │         /              │       /   │          │   │       │         /        │     /   │     /        │        │     │         └─────────────────       └──/  └──────────/        │
 * │       └────────/               └──────/    └──────────┤   │       └────────/         └────/    └────/         └──      │     │                                                            │
 * │       0       t0     t0+TP           t2    t2+TP      │   │       0       t0        t1   t2    t3             t5       │     │                                                            │
 * │                                                       │   │                                                            │     │                                                            │
 * │                                                       │   │                                                            │     │                                                            │
 * └───────────────────────────────────────────────────────┘   └────────────────────────────────────────────────────────────┘     └────────────────────────────────────────────────────────────┘
 */
#[repr(C)]
#[derive(Default, Debug)]
struct MainType {
    value: bool,
    tp_out: bool,
    tp_et: iec61131std::timers::Time,
    tp_inst: TimerParams,
}

#[test]
fn tp_true_for_time() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_out  : BOOL;
                tp_et   : TIME;
                tp_inst : TP;
            END_VAR
            tp_inst(IN := value, PT := T#10ms, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainType { value: true, ..MainType::default() };
    //On first call, out is true, et is 0
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    //After 5ms, out is true, et is 5ms
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 5_000_000);
    //At 10ms, out is true, et is 10ms
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 10_000_000);
    //After 15ms, out is false, et is 10/
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 10_000_000);
    //After 20ms, input is off, out remains off, et set to 0
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    main_inst.value = false;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
}

#[test]
fn tp_does_not_retrigger_on_consecutive_input() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_out  : BOOL;
                tp_et   : TIME;
                tp_inst : TP;
            END_VAR
            tp_inst(IN := value, PT := T#10ms, Q => tp_out, ET => tp_et);

        END_PROGRAM
    "#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);

    let mut main_inst = MainType { value: true, ..MainType::default() };
    //On first call, out is true, et is 0
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    //At 10ms, out is true, et is 10ms
    assert!(module.mock_time_advance_ns(Duration::from_millis(10).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 10_000_000);
    //After 15ms, out is false, et is 10/
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 10_000_000);
    //After 20ms, out is false, et is 10/
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 10_000_000);
}

#[test]
fn tp_not_interrupted_by_signal_change() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_out  : BOOL;
                tp_et   : TIME;
                tp_inst : TP;
            END_VAR
            tp_inst(IN := value, PT := T#10ms, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainType { value: true, ..MainType::default() };

    //On first call with true, out is true, et is 0
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);

    //advance 1 ms
    assert!(module.mock_time_advance_ns(Duration::from_millis(1).as_nanos() as u64));
    //call timer with false
    main_inst.value = false;
    module.run::<_, ()>("main", &mut main_inst);
    //Verify that the timer is still running
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 1_000_000);
    // advance by 1 ms
    assert!(module.mock_time_advance_ns(Duration::from_millis(1).as_nanos() as u64));
    //call timer with true
    main_inst.value = true;
    module.run::<_, ()>("main", &mut main_inst);
    //assert that the signal was not interrupted
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 2_000_000);
}

#[test]
fn ton_returns_true_after_time_preset() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_out  : BOOL;
                tp_et   : TIME;
                tp_inst : TON;
            END_VAR
            tp_inst(IN := value, PT := T#10ms, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainType { value: true, ..MainType::default() };
    // Value true First call -> false
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    // Value true After 5ms -> false
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    main_inst.value = true;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 5_000_000);
    // Value true After 10ms -> false
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    main_inst.value = true;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 10_000_000);
    // Value true After 15ms -> true
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    main_inst.value = true;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 10_000_000);
    // Value false after 20ms -> false
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    main_inst.value = false;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
}

#[test]
fn ton_q_defaults_to_false() {
    let prog = r#"
        VAR_GLOBAL
            ton_test: TON;
        END_VAR

        PROGRAM main
            VAR
                tp_out  : BOOL;
                tp_et   : TIME;
                tp_inst : TON;
            END_VAR
                ton_test(IN:=TRUE, PT:=T#2s);
                tp_out = ton_test.Q;
        END_PROGRAM
    "#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainType { value: true, ..MainType::default() };
    // Value true First call -> false
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
}

#[test]
fn ton_counts_elapsed_time_while_waiting() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_out  : BOOL;
                tp_et   : TIME;
                tp_inst : TON;
            END_VAR
            tp_inst(IN := value, PT := T#10ms, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainType { value: true, ..MainType::default() };
    // Value true, counter starts at 0
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    // Value true after 5ms counter at 5ms
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    main_inst.value = true;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 5_000_000);
    // Value false after 6ms counter at 0ms (stopped)
    assert!(module.mock_time_advance_ns(Duration::from_millis(1).as_nanos() as u64));
    main_inst.value = false;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
}

#[test]
fn ton_waits_again_after_turining_off() {
    let prog = r#"
        PROGRAM main
            VAR_INPUT
                value : BOOL;
            END_VAR
            VAR
                tp_out  : BOOL;
                tp_et   : TIME;
                tp_inst : TON;
            END_VAR
            tp_inst(IN := value, PT := T#9ms, Q => tp_out, ET => tp_et);
        END_PROGRAM
    "#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainType { value: true, ..MainType::default() };
    // Value true First call -> false
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    // Value true After 5ms -> false
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    main_inst.value = true;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 5_000_000);
    // Value true After 10ms -> true
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    main_inst.value = true;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 9_000_000);
    // Value false After 15ms -> false
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    main_inst.value = false;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    // Value true after 20ms -> false
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    main_inst.value = true;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    // Value true after 30ms -> true
    assert!(module.mock_time_advance_ns(Duration::from_millis(10).as_nanos() as u64));
    main_inst.value = true;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 9_000_000);
}

#[test]
fn toff_starts_timer_after_input_is_off() {
    let prog = r#"
    PROGRAM main
        VAR_INPUT
            value : BOOL;
        END_VAR
        VAR
            tp_out  : BOOL;
            tp_et   : TIME;
            tp_inst : TOF;
        END_VAR
        tp_inst(IN := value, PT := T#9ms, Q => tp_out, ET => tp_et);
    END_PROGRAM
"#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainType { value: true, ..MainType::default() };
    // Value true First call -> true
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    assert!(module.mock_time_advance_ns(Duration::from_millis(10).as_nanos() as u64));
    //Turn off after 10ms -> Timer kicks in, output remains true
    main_inst.value = false;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    //After 15 ms, output still true, time elapsed is 5ms
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 5_000_000);
}

#[test]
fn toff_runs_for_preset_time() {
    let prog = r#"
    PROGRAM main
        VAR_INPUT
            value : BOOL;
        END_VAR
        VAR
            tp_out  : BOOL;
            tp_et   : TIME;
            tp_inst : TOF;
        END_VAR
        tp_inst(IN := value, PT := T#9ms, Q => tp_out, ET => tp_et);
    END_PROGRAM
"#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainType { value: true, ..MainType::default() };
    // Value true First call -> true
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    assert!(module.mock_time_advance_ns(Duration::from_millis(10).as_nanos() as u64));
    //Turn off after 10ms -> Timer kicks in, output remains true
    main_inst.value = false;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    //After 20ms, output is turned off, time elapsed is equal to tp (9ms)
    assert!(module.mock_time_advance_ns(Duration::from_millis(10).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(!main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 9_000_000);

    //On the next true signal, the timer's elapsed time is set to 0 again
    // Value true First call -> true
    main_inst.value = true;
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
}

#[test]
fn toff_keeps_returning_true_if_input_returns_to_true() {
    let prog = r#"
    PROGRAM main
        VAR_INPUT
            value : BOOL;
        END_VAR
        VAR
            tp_out  : BOOL;
            tp_et   : TIME;
            tp_inst : TOF;
        END_VAR
        tp_inst(IN := value, PT := T#9ms, Q => tp_out, ET => tp_et);
    END_PROGRAM
"#;

    let sources = vec![prog.into()];
    let includes = get_includes(&["timers.st"]);
    let context = CodegenContext::create();
    let module = compile_and_load(&context, sources, includes);
    let mut main_inst = MainType { value: true, ..MainType::default() };
    // Value true First call -> false
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    //Turn off after 10ms -> Timer kicks in, output remains true
    assert!(module.mock_time_advance_ns(Duration::from_millis(10).as_nanos() as u64));
    main_inst.value = false;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    //After 15 ms, output still true, time elapsed is 5ms
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 5_000_000);
    //After 16ms, the input becomes true again, the timer stops, et is set to 0 but the signal remains true
    assert!(module.mock_time_advance_ns(Duration::from_millis(1).as_nanos() as u64));
    main_inst.value = true;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    //After 20ms, the input turns off, the timer starts again
    assert!(module.mock_time_advance_ns(Duration::from_millis(4).as_nanos() as u64));
    main_inst.value = false;
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 0);
    //After 25ms, the input is still off, the timer's elapsed time is 5ms, the output is true
    assert!(module.mock_time_advance_ns(Duration::from_millis(5).as_nanos() as u64));
    module.run::<_, ()>("main", &mut main_inst);
    assert!(main_inst.tp_out);
    assert_eq!(main_inst.tp_et, 5_000_000);
}
