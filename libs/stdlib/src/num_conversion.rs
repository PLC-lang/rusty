macro_rules! define_rounding_conversion {
    ($name:ident, $input_ty:ty, $output_ty:ty) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn $name(input: $input_ty) -> $output_ty {
            input.round() as $output_ty
        }
    };
}

define_rounding_conversion!(LREAL_TO_LINT, f64, i64);
define_rounding_conversion!(LREAL_TO_DINT, f64, i32);
define_rounding_conversion!(LREAL_TO_INT, f64, i16);
define_rounding_conversion!(LREAL_TO_SINT, f64, i8);
define_rounding_conversion!(LREAL_TO_ULINT, f64, u64);
define_rounding_conversion!(LREAL_TO_UDINT, f64, u32);
define_rounding_conversion!(LREAL_TO_UINT, f64, u16);
define_rounding_conversion!(LREAL_TO_USINT, f64, u8);

define_rounding_conversion!(REAL_TO_LINT, f32, i64);
define_rounding_conversion!(REAL_TO_DINT, f32, i32);
define_rounding_conversion!(REAL_TO_INT, f32, i16);
define_rounding_conversion!(REAL_TO_SINT, f32, i8);
define_rounding_conversion!(REAL_TO_ULINT, f32, u64);
define_rounding_conversion!(REAL_TO_UDINT, f32, u32);
define_rounding_conversion!(REAL_TO_UINT, f32, u16);
define_rounding_conversion!(REAL_TO_USINT, f32, u8);
