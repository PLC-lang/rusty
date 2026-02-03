use std::time::Duration;
#[cfg(not(feature = "mock_time"))]
use std::time::Instant;

#[cfg(feature = "mock_time")]
use test_time_helpers::Instant;

use crate::utils::Signal;

#[cfg(feature = "mock_time")]
pub mod test_time_helpers;

pub type Time = i64;

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

impl TimerParams {
    /// This method returns true if the timer has already started
    /// It does not take into consideration the preset/range for the timer
    /// Only if a start time has been set.
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

    fn set_elapsed_time(&mut self, duration: i64) {
        self.elapsed_time = duration;
    }

    /// Sets the elapsed time to either the preset time or the real elapsed time, whatever is smaller
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

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TP(timer: &mut TimerParams) {
    //If timer is active (start time set)
    let output = if timer.is_running() {
        timer.update_elapsed_time();
        // If time elapsed within range
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
        unreachable!("We should not get here, if we do write the failing test for it.")
    };
    timer.set_output(output);
    timer.input_edge.set(timer.input);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TON(timer: &mut TimerParams) {
    let output = if timer.input {
        //Timer was strarted at some point
        if timer.is_running() {
            //Timer is still running
            timer.update_elapsed_time();
            !timer.is_in_preset_range()
            //Timer stopped, but the input is new
        } else if timer.input_rising_edge() {
            timer.start();
            false
            //Timer stopped, input didn't change (still true from last time)
        } else {
            true
        }
    } else {
        //Input is false, stop timer regardless
        timer.reset();
        false
    };
    timer.set_output(output);
    timer.input_edge.set(timer.input);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TOF(timer: &mut TimerParams) {
    let output = if timer.input {
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
    timer.input_edge.set(timer.input);
}

// Aliases
#[no_mangle]
pub extern "C" fn TP_TIME(timer: &mut TimerParams) {
    TP(timer)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TP_LTIME(timer: &mut TimerParams) {
    TP(timer)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TON_TIME(timer: &mut TimerParams) {
    TON(timer)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TON_LTIME(timer: &mut TimerParams) {
    TON(timer)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TOF_TIME(timer: &mut TimerParams) {
    TOF(timer)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TOF_LTIME(timer: &mut TimerParams) {
    TOF(timer)
}
