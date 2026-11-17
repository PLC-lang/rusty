use crate::bit_num_conversion::{whole_to_i64, whole_to_u64};

/// One past the largest value a signed 32-bit integer can hold, as a float: 2^31.
const I32_RANGE_END: f64 = (i32::MAX as i64 + 1) as f64;

/// Converts a whole value as the x86 `cvttsd2si` instruction does with a 32-bit destination: a
/// value outside the signed 32-bit range, NaN and the infinities included, becomes `i32::MIN`. The
/// caller keeps the low bits of its target width, as CODESYS does for `TRUNC` and `TRUNC_INT`.
fn whole_to_i32(whole: f64) -> i32 {
    if (-I32_RANGE_END..I32_RANGE_END).contains(&whole) {
        whole as i32
    } else {
        i32::MIN
    }
}

macro_rules! define_truncating_conversion_32 {
    ($name:ident, $input_ty:ty, $output_ty:ty) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn $name(input: $input_ty) -> $output_ty {
            whole_to_i32(f64::from(input).trunc()) as $output_ty
        }
    };
}

macro_rules! define_truncating_conversion_64 {
    ($name:ident, $input_ty:ty, $output_ty:ty) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn $name(input: $input_ty) -> $output_ty {
            whole_to_i64(f64::from(input).trunc()) as $output_ty
        }
    };
}

define_truncating_conversion_32!(REAL_TRUNC_SINT, f32, i8);
define_truncating_conversion_32!(REAL_TRUNC_USINT, f32, u8);
define_truncating_conversion_32!(REAL_TRUNC_INT, f32, i16);
define_truncating_conversion_32!(REAL_TRUNC_UINT, f32, u16);
define_truncating_conversion_32!(REAL_TRUNC_DINT, f32, i32);

define_truncating_conversion_32!(LREAL_TRUNC_SINT, f64, i8);
define_truncating_conversion_32!(LREAL_TRUNC_USINT, f64, u8);
define_truncating_conversion_32!(LREAL_TRUNC_INT, f64, i16);
define_truncating_conversion_32!(LREAL_TRUNC_UINT, f64, u16);
define_truncating_conversion_32!(LREAL_TRUNC_DINT, f64, i32);

define_truncating_conversion_64!(REAL_TRUNC_UDINT, f32, u32);
define_truncating_conversion_64!(REAL_TRUNC_LINT, f32, i64);
define_truncating_conversion_64!(LREAL_TRUNC_UDINT, f64, u32);
define_truncating_conversion_64!(LREAL_TRUNC_LINT, f64, i64);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn REAL_TRUNC_ULINT(input: f32) -> u64 {
    whole_to_u64(f64::from(input).trunc())
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LREAL_TRUNC_ULINT(input: f64) -> u64 {
    whole_to_u64(input.trunc())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncates_toward_zero() {
        assert_eq!(REAL_TRUNC_INT(5.9), 5);
        assert_eq!(REAL_TRUNC_INT(-5.9), -5);
        assert_eq!(LREAL_TRUNC_DINT(-0.9), 0);
        assert_eq!(LREAL_TRUNC_USINT(2.5), 2);
    }

    #[test]
    fn negative_value_wraps_into_unsigned_target() {
        assert_eq!(REAL_TRUNC_USINT(-5.9), 251);
        assert_eq!(LREAL_TRUNC_UDINT(-1.9), u32::MAX);
        assert_eq!(LREAL_TRUNC_ULINT(-1.9), u64::MAX);
    }

    #[test]
    fn value_inside_i32_keeps_low_bits_of_target() {
        assert_eq!(REAL_TRUNC_SINT(300.7), 44);
        assert_eq!(LREAL_TRUNC_INT(40000.9), -25536);
        assert_eq!(LREAL_TRUNC_UINT(2_147_483_647.9), u16::MAX);
        assert_eq!(LREAL_TRUNC_DINT(-2_147_483_648.9), i32::MIN);
    }

    #[test]
    fn value_outside_i32_becomes_i32_min_for_sint_to_dint() {
        assert_eq!(LREAL_TRUNC_DINT(2_147_483_648.0), i32::MIN);
        assert_eq!(REAL_TRUNC_INT(1.0E10), 0);
        assert_eq!(LREAL_TRUNC_DINT(f64::NAN), i32::MIN);
        assert_eq!(REAL_TRUNC_SINT(f32::INFINITY), 0);
    }

    #[test]
    fn udint_is_exact_up_to_2_pow_32() {
        assert_eq!(REAL_TRUNC_UDINT(3.0E9), 3_000_000_000);
        assert_eq!(LREAL_TRUNC_UDINT(4_294_967_295.9), u32::MAX);
        assert_eq!(LREAL_TRUNC_UDINT(4_294_967_296.0), 0);
        assert_eq!(LREAL_TRUNC_UDINT(1.0E10), 1_410_065_408);
        assert_eq!(REAL_TRUNC_UDINT(f32::NAN), 0);
    }

    #[test]
    fn value_outside_i64_becomes_i64_min_for_udint_lint_ulint() {
        assert_eq!(LREAL_TRUNC_LINT(1.0E10), 10_000_000_000);
        assert_eq!(REAL_TRUNC_LINT(f32::NAN), i64::MIN);
        assert_eq!(LREAL_TRUNC_LINT(f64::NEG_INFINITY), i64::MIN);
        assert_eq!(REAL_TRUNC_ULINT(f32::INFINITY), 1 << 63);
    }

    #[test]
    fn ulint_keeps_upper_half_exact() {
        assert_eq!(LREAL_TRUNC_ULINT(1.0E19), 10_000_000_000_000_000_000);
        assert_eq!(LREAL_TRUNC_LINT(1.0E19), i64::MIN);
        assert_eq!(LREAL_TRUNC_ULINT(-1.0E19), 1 << 63);
    }
}
