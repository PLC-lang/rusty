/// One past the largest value a signed 64-bit integer can hold, as a float: 2^63.
const I64_RANGE_END: f64 = 9223372036854775808.0;

/// One past the largest value an unsigned 64-bit integer can hold, as a float: 2^64.
const U64_RANGE_END: f64 = 18446744073709551616.0;

/// Rounds half away from zero, then converts as the x86 `cvttsd2si` instruction does: a value
/// outside the signed 64-bit range, NaN and the infinities included, becomes `i64::MIN`. The
/// caller keeps the low bits of its target width, so an out-of-range value wraps and a negative
/// value lands at the top of an unsigned range.
fn round_to_i64(value: f64) -> i64 {
    let rounded = value.round();
    if (-I64_RANGE_END..I64_RANGE_END).contains(&rounded) {
        rounded as i64
    } else {
        i64::MIN
    }
}

/// The 64-bit target keeps a value in [2^63, 2^64) exact, as the x86 unsigned conversion sequence
/// does. Everything else follows `round_to_i64`, so a negative value wraps, and NaN, the infinities
/// and a value outside [-2^63, 2^64) become 2^63.
fn round_to_u64(value: f64) -> u64 {
    let rounded = value.round();
    if (I64_RANGE_END..U64_RANGE_END).contains(&rounded) {
        rounded as u64
    } else {
        round_to_i64(value) as u64
    }
}

macro_rules! define_wrapping_conversion {
    ($name:ident, $input_ty:ty, $output_ty:ty) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn $name(input: $input_ty) -> $output_ty {
            round_to_i64(f64::from(input)) as $output_ty
        }
    };
}

define_wrapping_conversion!(LREAL_TO_DWORD, f64, u32);
define_wrapping_conversion!(LREAL_TO_WORD, f64, u16);
define_wrapping_conversion!(LREAL_TO_BYTE, f64, u8);

define_wrapping_conversion!(REAL_TO_DWORD, f32, u32);
define_wrapping_conversion!(REAL_TO_WORD, f32, u16);
define_wrapping_conversion!(REAL_TO_BYTE, f32, u8);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LREAL_TO_LWORD(input: f64) -> u64 {
    round_to_u64(input)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn REAL_TO_LWORD(input: f32) -> u64 {
    round_to_u64(f64::from(input))
}
