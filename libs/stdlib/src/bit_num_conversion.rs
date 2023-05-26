/// .
/// Converts LWORD to LREAL
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LWORD_TO_LREAL(input: u64) -> f64 {
    f64::from_bits(input)
}

/// .
/// Converts DWORD to REAL
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DWORD_TO_REAL(input: u32) -> f32 {
    f32::from_bits(input)
}

/// .
/// Converts LREAL to LWORD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LREAL_TO_LWORD(input: f64) -> u64 {
    f64::to_bits(input)
}

/// .
/// Converts REAL to DWORD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn REAL_TO_DWORD(input: f32) -> u32 {
    f32::to_bits(input)
}
