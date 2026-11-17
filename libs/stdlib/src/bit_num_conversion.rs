/// One past the largest value a signed 64-bit integer can hold, as a float: 2^63.
const I64_RANGE_END: f64 = (i64::MAX as u128 + 1) as f64;

/// One past the largest value an unsigned 64-bit integer can hold, as a float: 2^64.
const U64_RANGE_END: f64 = (u64::MAX as u128 + 1) as f64;

/// Converts a whole value as the x86 `cvttsd2si` instruction does: a value outside the signed
/// 64-bit range, NaN and the infinities included, becomes `i64::MIN`. The caller keeps the low bits
/// of its target width, so an out-of-range value wraps and a negative value lands at the top of an
/// unsigned range.
pub(crate) fn whole_to_i64(whole: f64) -> i64 {
    if (-I64_RANGE_END..I64_RANGE_END).contains(&whole) {
        whole as i64
    } else {
        i64::MIN
    }
}

/// The 64-bit target keeps a whole value in [2^63, 2^64) exact, as the x86 unsigned conversion
/// sequence does. Everything else follows `whole_to_i64`, so a negative value wraps, and NaN, the
/// infinities and a value outside [-2^63, 2^64) become 2^63.
pub(crate) fn whole_to_u64(whole: f64) -> u64 {
    if (I64_RANGE_END..U64_RANGE_END).contains(&whole) {
        whole as u64
    } else {
        whole_to_i64(whole) as u64
    }
}

macro_rules! define_wrapping_conversion {
    ($name:ident, $input_ty:ty, $output_ty:ty) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn $name(input: $input_ty) -> $output_ty {
            whole_to_i64(f64::from(input).round()) as $output_ty
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
    whole_to_u64(input.round())
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn REAL_TO_LWORD(input: f32) -> u64 {
    whole_to_u64(f64::from(input).round())
}
