use paste::paste;
macro_rules! define_endianness_for_int_types {
    ( $st_type:tt, $t:ty ) => {
        paste! {
            /// .
            /// Converts given integer type from little endian data format to big endian data format
            ///
            #[allow(non_snake_case)]
            #[no_mangle]
            pub fn [<TO_BIG_ENDIAN__ $st_type>](input: $t) -> $t {
                return input.to_be();
            }

            /// .
            /// Converts given integer type from big endian data format to little endian data format
            ///
            #[allow(non_snake_case)]
            #[no_mangle]
            pub fn [<TO_LITTLE_ENDIAN__ $st_type>](input: $t) -> $t {
                return input.to_le();
            }

            /// .
            /// Converts given integer type from big endian data format to little endian data format
            ///
            #[allow(non_snake_case)]
            #[no_mangle]
            pub fn [<FROM_BIG_ENDIAN__ $st_type>](input: $t) -> $t {
                return input.to_le();
            }

            /// .
            /// Converts given integer type from little endian data format to big endian data format
            ///
            #[allow(non_snake_case)]
            #[no_mangle]
            pub fn [<FROM_LITTLE_ENDIAN__ $st_type>](input: $t) -> $t {
                return input.to_be();
            }
        }
    };
}

// Define endianness for types specified in DIN-EN 61131-3 (with the exception of structs)
define_endianness_for_int_types!(INT, i16);
define_endianness_for_int_types!(DINT, i32);
define_endianness_for_int_types!(LINT, i64);
define_endianness_for_int_types!(UINT, u16);
define_endianness_for_int_types!(UDINT, u32);
define_endianness_for_int_types!(ULINT, u64);
define_endianness_for_int_types!(WORD, u16);
define_endianness_for_int_types!(DWORD, u32);
define_endianness_for_int_types!(LWORD, u64);
define_endianness_for_int_types!(WCHAR, u16);
define_endianness_for_int_types!(DATE, i64);
define_endianness_for_int_types!(TIME_OF_DAY, i64);
define_endianness_for_int_types!(DATE_AND_TIME, i64);

/// .
/// Converts given f32 from little endian data format to big endian data format
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__REAL(input: f32) -> f32 {
    f32::from_be_bytes(input.to_be_bytes())
}

/// .
/// Converts given f32 from big endian data format to little endian data format
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__REAL(input: f32) -> f32 {
    f32::from_le_bytes(input.to_le_bytes())
}

/// .
/// Converts given f32 from big endian data format to little endian data format
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__REAL(input: f32) -> f32 {
    f32::from_le_bytes(input.to_le_bytes())
}

/// .
/// Converts given f32 from little endian data format to big endian data format
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__REAL(input: f32) -> f32 {
    f32::from_be_bytes(input.to_be_bytes())
}

/// .
/// Converts given f64 from little endian data format to big endian data format
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_BIG_ENDIAN__LREAL(input: f64) -> f64 {
    f64::from_be_bytes(input.to_be_bytes())
}

/// .
/// Converts given f64 from big endian data format to little endian data format
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn TO_LITTLE_ENDIAN__LREAL(input: f64) -> f64 {
    f64::from_le_bytes(input.to_le_bytes())
}

/// .
/// Converts given f64 from big endian data format to little endian data format
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_BIG_ENDIAN__LREAL(input: f64) -> f64 {
    f64::from_le_bytes(input.to_le_bytes())
}

/// .
/// Converts given f64 from little endian data format to big endian data format
///
#[allow(non_snake_case)]
#[no_mangle]
pub fn FROM_LITTLE_ENDIAN__LREAL(input: f64) -> f64 {
    f64::from_be_bytes(input.to_be_bytes())
}
