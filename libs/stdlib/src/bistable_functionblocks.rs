#[repr(C)]
#[derive(Debug, Default)]
pub struct SetResetParams {
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
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn SR(params: &mut SetResetParams) {
    params.set_output(params.set | (!params.reset & params.output));
}

///.
/// Bistable function, reset dominant
///
/// # Safety
/// Working with raw pointers
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn RS(params: &mut SetResetParams) {
    params.set_output(!params.reset & (params.set | params.output));
}
