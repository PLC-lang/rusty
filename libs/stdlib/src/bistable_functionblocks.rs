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

///.
/// Bistable function, set dominant
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SR(params: &mut SetResetParams) {
    params.set_output(params.set | (!params.reset & params.output));
}

///.
/// Bistable function, reset dominant
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn RS(params: &mut SetResetParams) {
    params.set_output(!params.reset & (params.set | params.output));
}
