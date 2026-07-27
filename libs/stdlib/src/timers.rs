use std::time::Duration;
#[cfg(not(feature = "mock_time"))]
use std::time::Instant;

#[cfg(feature = "mock_time")]
use test_time_helpers::Instant;

use crate::utils::Signal;

#[cfg(feature = "mock_time")]
pub mod test_time_helpers;

pub type Time = u32;
pub type LTime = i64;

#[repr(C)]
#[derive(Debug, Default)]
pub struct TimerParams {
    __vtable: usize,
    input: bool,
    preset_time: Time,
    output: bool,
    elapsed_time: Time,
    input_edge: Signal,
    is_running: bool,
    start_time: Option<Instant>,
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct TimerParamsLTime {
    __vtable: usize,
    input: bool,
    preset_time: LTime,
    output: bool,
    elapsed_time: LTime,
    input_edge: Signal,
    is_running: bool,
    start_time: Option<Instant>,
}

impl TimerParams {
    fn is_running(&self) -> bool {
        self.is_running
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.is_running = true;
        self.set_elapsed_time(0);
    }

    fn reset(&mut self) {
        self.start_time = None;
        self.is_running = false;
        self.set_elapsed_time(0);
    }

    fn set_elapsed_time(&mut self, duration: Time) {
        self.elapsed_time = duration;
    }

    fn update_elapsed_time(&mut self) {
        if self.is_running() {
            let elapsed_millis =
                self.get_run_time().expect("Timer should be running").as_millis().min(u32::MAX as u128)
                    as u32;

            self.set_elapsed_time(std::cmp::min(self.preset_time, elapsed_millis));
        }
    }

    fn is_in_preset_range(&self) -> bool {
        let duration = Duration::from_millis(self.preset_time as u64);
        self.get_run_time().is_some_and(|it| it <= duration)
    }

    fn get_run_time(&self) -> Option<Duration> {
        self.start_time.map(|it| it.elapsed())
    }

    fn set_output(&mut self, value: bool) {
        self.output = value;
    }

    fn input_rising_edge(&mut self) -> bool {
        self.input_edge.rising_edge(self.input)
    }

    fn input_falling_edge(&mut self) -> bool {
        self.input_edge.falling_edge(self.input)
    }
}

impl TimerParamsLTime {
    fn is_running(&self) -> bool {
        self.is_running
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.is_running = true;
        self.set_elapsed_time(0);
    }

    fn reset(&mut self) {
        self.start_time = None;
        self.is_running = false;
        self.set_elapsed_time(0);
    }

    fn set_elapsed_time(&mut self, duration: LTime) {
        self.elapsed_time = duration;
    }

    fn update_elapsed_time(&mut self) {
        if self.is_running() {
            self.set_elapsed_time(std::cmp::min(
                self.preset_time,
                self.get_run_time().expect("Timer should be running").as_nanos() as i64,
            ));
        }
    }

    fn is_in_preset_range(&self) -> bool {
        let duration = Duration::from_nanos(self.preset_time as u64);
        self.get_run_time().is_some_and(|it| it <= duration)
    }

    fn get_run_time(&self) -> Option<Duration> {
        self.start_time.map(|it| it.elapsed())
    }

    fn set_output(&mut self, value: bool) {
        self.output = value;
    }

    fn input_rising_edge(&mut self) -> bool {
        self.input_edge.rising_edge(self.input)
    }

    fn input_falling_edge(&mut self) -> bool {
        self.input_edge.falling_edge(self.input)
    }
}

trait TimerLogic {
    fn input(&self) -> bool;
    fn is_running(&self) -> bool;
    fn start(&mut self);
    fn reset(&mut self);
    fn update_elapsed_time(&mut self);
    fn is_in_preset_range(&self) -> bool;
    fn set_output(&mut self, value: bool);
    fn input_rising_edge(&mut self) -> bool;
    fn input_falling_edge(&mut self) -> bool;
    fn update_input_edge(&mut self);
}

impl TimerLogic for TimerParams {
    fn input(&self) -> bool {
        self.input
    }

    fn is_running(&self) -> bool {
        self.is_running()
    }

    fn start(&mut self) {
        self.start()
    }

    fn reset(&mut self) {
        self.reset()
    }

    fn update_elapsed_time(&mut self) {
        self.update_elapsed_time()
    }

    fn is_in_preset_range(&self) -> bool {
        self.is_in_preset_range()
    }

    fn set_output(&mut self, value: bool) {
        self.set_output(value)
    }

    fn input_rising_edge(&mut self) -> bool {
        self.input_rising_edge()
    }

    fn input_falling_edge(&mut self) -> bool {
        self.input_falling_edge()
    }

    fn update_input_edge(&mut self) {
        self.input_edge.set(self.input);
    }
}

impl TimerLogic for TimerParamsLTime {
    fn input(&self) -> bool {
        self.input
    }

    fn is_running(&self) -> bool {
        self.is_running()
    }

    fn start(&mut self) {
        self.start()
    }

    fn reset(&mut self) {
        self.reset()
    }

    fn update_elapsed_time(&mut self) {
        self.update_elapsed_time()
    }

    fn is_in_preset_range(&self) -> bool {
        self.is_in_preset_range()
    }

    fn set_output(&mut self, value: bool) {
        self.set_output(value)
    }

    fn input_rising_edge(&mut self) -> bool {
        self.input_rising_edge()
    }

    fn input_falling_edge(&mut self) -> bool {
        self.input_falling_edge()
    }

    fn update_input_edge(&mut self) {
        self.input_edge.set(self.input);
    }
}

fn run_tp<T: TimerLogic>(timer: &mut T) {
    let output = if timer.is_running() {
        timer.update_elapsed_time();
        if timer.is_in_preset_range() {
            true
        } else {
            if timer.input_falling_edge() {
                timer.reset()
            }
            false
        }
    } else if timer.input_rising_edge() {
        timer.start();
        true
    } else {
        false
    };
    timer.set_output(output);
    timer.update_input_edge();
}

fn run_ton<T: TimerLogic>(timer: &mut T) {
    let output = if timer.input() {
        if timer.is_running() {
            timer.update_elapsed_time();
            !timer.is_in_preset_range()
        } else if timer.input_rising_edge() {
            timer.start();
            false
        } else {
            true
        }
    } else {
        timer.reset();
        false
    };
    timer.set_output(output);
    timer.update_input_edge();
}

fn run_tof<T: TimerLogic>(timer: &mut T) {
    let output = if timer.input() {
        if timer.input_rising_edge() {
            timer.reset();
        }
        true
    } else if timer.input_falling_edge() {
        timer.start();
        true
    } else if timer.is_running() {
        timer.update_elapsed_time();
        timer.is_in_preset_range()
    } else {
        false
    };
    timer.set_output(output);
    timer.update_input_edge();
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TP(timer: &mut TimerParams) {
    TP_TIME(timer)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TON(timer: &mut TimerParams) {
    TON_TIME(timer)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TOF(timer: &mut TimerParams) {
    TOF_TIME(timer)
}

#[no_mangle]
pub extern "C" fn TP_TIME(timer: &mut TimerParams) {
    run_tp(timer)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TP_LTIME(timer: &mut TimerParamsLTime) {
    run_tp(timer)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TON_TIME(timer: &mut TimerParams) {
    run_ton(timer)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TON_LTIME(timer: &mut TimerParamsLTime) {
    run_ton(timer)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TOF_TIME(timer: &mut TimerParams) {
    run_tof(timer)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TOF_LTIME(timer: &mut TimerParamsLTime) {
    run_tof(timer)
}
