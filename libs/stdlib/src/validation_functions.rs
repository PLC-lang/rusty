use std::ops::{BitAnd, Shr};

/// .
/// Check if input is a valid REAL
/// NaN or infinite will return FALSE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IS_VALID__REAL(input: f32) -> bool {
    !(input.is_nan() || input.is_infinite())
}

/// .
/// Check if input is a valid LREAL
/// NaN or infinite will return FALSE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IS_VALID__LREAL(input: f64) -> bool {
    !(input.is_nan() || input.is_infinite())
}

const BITS_PER_BCD_DIGIT: usize = 4;

fn is_valid_bcd<T>(input: T) -> bool
where
    T: Shr<usize, Output = T> + BitAnd<Output = T> + Copy + From<u8> + PartialOrd,
{
    let iterations = std::mem::size_of::<T>() * u8::BITS as usize / BITS_PER_BCD_DIGIT;
    for i in 0..iterations {
        if ((input >> (BITS_PER_BCD_DIGIT * i)) & 0b1111.into()) > 9.into() {
            return false;
        }
    }
    true
}

/// .
/// Check if input as a valid BCD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IS_VALID_BCD__BYTE(input: u8) -> bool {
    is_valid_bcd(input)
}

/// .
/// Check if input as a valid BCD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IS_VALID_BCD__WORD(input: u16) -> bool {
    is_valid_bcd(input)
}

/// .
/// Check if input as a valid BCD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IS_VALID_BCD__DWORD(input: u32) -> bool {
    is_valid_bcd(input)
}

/// .
/// Check if input as a valid BCD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IS_VALID_BCD__LWORD(input: u64) -> bool {
    is_valid_bcd(input)
}
