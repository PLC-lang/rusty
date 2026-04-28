use crate::utils::Signal;

#[derive(Debug, Default)]
#[repr(C)]
pub struct Trigger {
    __vtable: usize,
    clk: bool,
    output: bool,
    internal: Signal,
}

impl Trigger {
    fn set_output(&mut self, val: bool) {
        self.output = val
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn R_TRIG(trigger: &mut Trigger) {
    let res = trigger.internal.rising_edge(trigger.clk);
    trigger.set_output(res);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn F_TRIG(trigger: &mut Trigger) {
    let res = trigger.internal.falling_edge(trigger.clk);
    trigger.set_output(res);
}
