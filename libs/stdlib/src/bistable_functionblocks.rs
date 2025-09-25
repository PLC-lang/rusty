#[repr(C)]
#[derive(Debug, Default)]
pub struct SetResetParams {
    __vtable: usize,
    set: bool,
    reset: bool,
    output: bool,
}

impl SetResetParams {
    fn set_output(&mut self, value: bool) {
        self.output = value;
    }
}

#[repr(C)]
pub struct VTable {
    pub body: extern "C" fn(&mut SetResetParams),
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_SR: VTable = VTable { body: SR };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __SR__init: SetResetParams =
    SetResetParams { __vtable: 0, set: false, reset: false, output: false };
///.
/// Bistable function, set dominant
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SR(params: &mut SetResetParams) {
    params.set_output(params.set | (!params.reset & params.output));
}

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __vtable_RS: VTable = VTable { body: RS };

#[allow(non_upper_case_globals)]
#[no_mangle]
#[used]
pub static __RS__init: SetResetParams =
    SetResetParams { __vtable: 0, set: false, reset: false, output: false };

///.
/// Bistable function, reset dominant
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn RS(params: &mut SetResetParams) {
    params.set_output(!params.reset & (params.set | params.output));
}
