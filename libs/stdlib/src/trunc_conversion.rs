use crate::bit_num_conversion::{whole_to_i64, whole_to_u64};

macro_rules! define_truncating_conversion {
    ($name:ident, $input_ty:ty, $output_ty:ty) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn $name(input: $input_ty) -> $output_ty {
            whole_to_i64(f64::from(input).trunc()) as $output_ty
        }
    };
}

define_truncating_conversion!(REAL_TRUNC_SINT, f32, i8);
define_truncating_conversion!(REAL_TRUNC_USINT, f32, u8);
define_truncating_conversion!(REAL_TRUNC_INT, f32, i16);
define_truncating_conversion!(REAL_TRUNC_UINT, f32, u16);
define_truncating_conversion!(REAL_TRUNC_DINT, f32, i32);
define_truncating_conversion!(REAL_TRUNC_UDINT, f32, u32);
define_truncating_conversion!(REAL_TRUNC_LINT, f32, i64);

define_truncating_conversion!(LREAL_TRUNC_SINT, f64, i8);
define_truncating_conversion!(LREAL_TRUNC_USINT, f64, u8);
define_truncating_conversion!(LREAL_TRUNC_INT, f64, i16);
define_truncating_conversion!(LREAL_TRUNC_UINT, f64, u16);
define_truncating_conversion!(LREAL_TRUNC_DINT, f64, i32);
define_truncating_conversion!(LREAL_TRUNC_UDINT, f64, u32);
define_truncating_conversion!(LREAL_TRUNC_LINT, f64, i64);

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
    fn out_of_range_value_keeps_low_bits_of_target() {
        assert_eq!(REAL_TRUNC_SINT(300.7), 44);
        assert_eq!(LREAL_TRUNC_INT(40000.9), -25536);
        assert_eq!(REAL_TRUNC_INT(1.0E10), -7168);
        assert_eq!(LREAL_TRUNC_DINT(1.0E10), 1_410_065_408);
    }

    #[test]
    fn value_outside_i64_becomes_i64_min() {
        assert_eq!(LREAL_TRUNC_DINT(1.0E20), 0);
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
