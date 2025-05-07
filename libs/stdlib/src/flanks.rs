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

#[repr(C)]
pub struct VTableTrigger {
    body: extern "C" fn(&mut Trigger),
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_R_TRIG: VTableTrigger = VTableTrigger { body: R_TRIG };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __R_TRIG__init: Trigger =
    Trigger { __vtable: 0, clk: false, output: false, internal: Signal { current_value: false } };

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn R_TRIG(trigger: &mut Trigger) {
    let res = trigger.internal.rising_edge(trigger.clk);
    trigger.set_output(res);
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_F_TRIG: VTableTrigger = VTableTrigger { body: F_TRIG };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __F_TRIG__init: Trigger =
    Trigger { __vtable: 0, clk: false, output: false, internal: Signal { current_value: false } };

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn F_TRIG(trigger: &mut Trigger) {
    let res = trigger.internal.falling_edge(trigger.clk);
    trigger.set_output(res);
}
