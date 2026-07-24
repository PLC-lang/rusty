use chrono::TimeZone;

const MILLIS_PER_SECOND: u32 = 1_000;
const MILLIS_PER_DAY: u32 = 60 * 60 * 24 * MILLIS_PER_SECOND;

fn checked_millis_to_seconds(input: u32) -> u32 {
    input / MILLIS_PER_SECOND
}

fn checked_seconds_to_millis(input: u32) -> u32 {
    input.checked_mul(MILLIS_PER_SECOND).unwrap()
}

/// .
/// This operator returns the value of adding up two TIME operands.
/// Panics on overflow.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_TIME(in1: u32, in2: u32) -> u32 {
    in1.checked_add(in2).unwrap()
}

/// .
/// This operator returns the value of adding up TOD and TIME.
/// Wraps around day boundaries.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_TOD_TIME(in1: u32, in2: u32) -> u32 {
    ((in1 as u64 + in2 as u64) % MILLIS_PER_DAY as u64) as u32
}

/// .
/// This operator returns the value of adding up DT and TIME.
/// Panics on overflow.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_DT_TIME(in1: u32, in2: u32) -> u32 {
    let time_seconds = checked_millis_to_seconds(in2);
    in1.checked_add(time_seconds).unwrap()
}

fn add_datetime_time(in1: i64, in2: i64) -> i64 {
    chrono::Utc
        .timestamp_nanos(in1)
        .checked_add_signed(chrono::Duration::nanoseconds(in2))
        .unwrap()
        .timestamp_nanos_opt()
        .unwrap()
}

/// .
/// This operator produces the subtraction of two TIME operands
/// Panics on underflow.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_TIME(in1: u32, in2: u32) -> u32 {
    in1.checked_sub(in2).unwrap()
}

/// .
/// This operator produces the subtraction of two DATE operands
/// Panics on underflow and when the resulting TIME would overflow.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_DATE_DATE(in1: u32, in2: u32) -> u32 {
    checked_seconds_to_millis(in1.checked_sub(in2).unwrap())
}

/// .
/// This operator produces the subtraction of TOD and TIME
/// Wraps around day boundaries.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_TOD_TIME(in1: u32, in2: u32) -> u32 {
    ((in1 as u64 + MILLIS_PER_DAY as u64 - (in2 % MILLIS_PER_DAY) as u64) % MILLIS_PER_DAY as u64) as u32
}

/// .
/// This operator produces the subtraction of two TOD operands
/// Panics on underflow.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_TOD_TOD(in1: u32, in2: u32) -> u32 {
    in1.checked_sub(in2).unwrap()
}

fn sub_datetimes(in1: i64, in2: i64) -> i64 {
    chrono::Utc
        .timestamp_nanos(in1)
        .signed_duration_since(chrono::Utc.timestamp_nanos(in2))
        .num_nanoseconds()
        .unwrap()
}

/// .
/// This operator produces the subtraction of DT and TIME
/// Panics on underflow.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_DT_TIME(in1: u32, in2: u32) -> u32 {
    let time_seconds = checked_millis_to_seconds(in2);
    in1.checked_sub(time_seconds).unwrap()
}

fn sub_datetime_duration(in1: i64, in2: i64) -> i64 {
    chrono::Utc
        .timestamp_nanos(in1)
        .checked_sub_signed(chrono::Duration::nanoseconds(in2))
        .unwrap()
        .timestamp_nanos_opt()
        .unwrap()
}

/// .
/// This operator produces the subtraction of two DT operands
/// Panics on underflow and when the resulting TIME would overflow.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_DT_DT(in1: u32, in2: u32) -> u32 {
    checked_seconds_to_millis(in1.checked_sub(in2).unwrap())
}

/// .
/// This operator returns the value of adding up two LTIME operands.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_LTIME(in1: i64, in2: i64) -> i64 {
    ADD__LTIME__LTIME(in1, in2)
}

/// .
/// This operator returns the value of adding up LTOD and LTIME.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_LTOD_LTIME(in1: i64, in2: i64) -> i64 {
    ADD__LTOD__LTIME(in1, in2)
}

/// .
/// This operator returns the value of adding up LDT and LTIME.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_LDT_LTIME(in1: i64, in2: i64) -> i64 {
    ADD__LDT__LTIME(in1, in2)
}

/// .
/// This operator produces the subtraction of two LTIME operands.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_LTIME(in1: i64, in2: i64) -> i64 {
    SUB__LTIME__LTIME(in1, in2)
}

/// .
/// This operator produces the subtraction of two LDATE operands.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_LDATE_LDATE(in1: i64, in2: i64) -> i64 {
    SUB__LDATE__LDATE(in1, in2)
}

/// .
/// This operator produces the subtraction of LTOD and LTIME.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_LTOD_LTIME(in1: i64, in2: i64) -> i64 {
    SUB__LTOD__LTIME(in1, in2)
}

/// .
/// This operator produces the subtraction of two LTOD operands.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_LTOD_LTOD(in1: i64, in2: i64) -> i64 {
    SUB__LTOD__LTOD(in1, in2)
}

/// .
/// This operator produces the subtraction of LDT and LTIME.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_LDT_LTIME(in1: i64, in2: i64) -> i64 {
    SUB__LDT__LTIME(in1, in2)
}

/// .
/// This operator produces the subtraction of two LDT operands.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_LDT_LDT(in1: i64, in2: i64) -> i64 {
    SUB__LDT__LDT(in1, in2)
}

/// .
/// Multiply TIME with SINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__SINT(in1: i64, in2: i8) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with INT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__INT(in1: i64, in2: i16) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with DINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__DINT(in1: i64, in2: i32) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with LINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__LINT(in1: i64, in2: i64) -> i64 {
    checked_mul_time_with_signed_int(in1, in2)
}

/// .
/// Multiply TIME with SINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__SINT(in1: i64, in2: i8) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with INT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__INT(in1: i64, in2: i16) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with DINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__DINT(in1: i64, in2: i32) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with LINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__LINT(in1: i64, in2: i64) -> i64 {
    checked_mul_time_with_signed_int(in1, in2)
}

/// .
/// Multiply LTIME with SINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__SINT(in1: i64, in2: i8) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply LTIME with INT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__INT(in1: i64, in2: i16) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply LTIME with DINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__DINT(in1: i64, in2: i32) -> i64 {
    checked_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply LTIME with LINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__LINT(in1: i64, in2: i64) -> i64 {
    checked_mul_time_with_signed_int(in1, in2)
}

/// .
/// Multiply TIME/LTIME with ANY_SIGNED_INT
/// Panic on overflow
///
fn checked_mul_time_with_signed_int(in1: i64, in2: i64) -> i64 {
    in1.checked_mul(in2).unwrap()
}

/// .
/// Multiply TIME with USINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__USINT(in1: i64, in2: u8) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__UINT(in1: i64, in2: u16) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UDINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__UDINT(in1: i64, in2: u32) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with ULINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__ULINT(in1: i64, in2: u64) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2)
}

/// .
/// Multiply TIME with USINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__USINT(in1: i64, in2: u8) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__UINT(in1: i64, in2: u16) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UDINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__UDINT(in1: i64, in2: u32) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with ULINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__ULINT(in1: i64, in2: u64) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2)
}

/// .
/// Multiply LTIME with USINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__USINT(in1: i64, in2: u8) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply LTIME with UINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__UINT(in1: i64, in2: u16) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply LTIME with UDINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__UDINT(in1: i64, in2: u32) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply LTIME with ULINT
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__ULINT(in1: i64, in2: u64) -> i64 {
    checked_mul_time_with_unsigned_int(in1, in2)
}

/// .
/// Multiply TIME/LTIME with ANY_UNSIGNED_INT
/// Panic on overflow
///
fn checked_mul_time_with_unsigned_int(in1: i64, in2: u64) -> i64 {
    // convert in2 [u64] to [i64]
    // if in2 is to large for [i64] the multiplication will allways overflow -> panic on try_into()
    in1.checked_mul(in2.try_into().unwrap()).unwrap()
}

/// .
/// Divide TIME by SINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__SINT(in1: i64, in2: i8) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by INT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__INT(in1: i64, in2: i16) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by DINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__DINT(in1: i64, in2: i32) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by LINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__LINT(in1: i64, in2: i64) -> i64 {
    checked_div_time_by_signed_int(in1, in2)
}

/// .
/// Divide TIME by SINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__SINT(in1: i64, in2: i8) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by INT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__INT(in1: i64, in2: i16) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by DINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__DINT(in1: i64, in2: i32) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by LINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__LINT(in1: i64, in2: i64) -> i64 {
    checked_div_time_by_signed_int(in1, in2)
}

/// .
/// Divide LTIME by SINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__SINT(in1: i64, in2: i8) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide LTIME by INT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__INT(in1: i64, in2: i16) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide LTIME by DINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__DINT(in1: i64, in2: i32) -> i64 {
    checked_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide LTIME by LINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__LINT(in1: i64, in2: i64) -> i64 {
    checked_div_time_by_signed_int(in1, in2)
}

/// .
/// Divide TIME/LTIME with ANY_SIGNED_INT
/// Panic on overflow or division by zero
///
fn checked_div_time_by_signed_int(in1: i64, in2: i64) -> i64 {
    in1.checked_div(in2).unwrap()
}

/// .
/// Divide TIME by USINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__USINT(in1: i64, in2: u8) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__UINT(in1: i64, in2: u16) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UDINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__UDINT(in1: i64, in2: u32) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by ULINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__ULINT(in1: i64, in2: u64) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2)
}

/// .
/// Divide TIME by USINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__USINT(in1: i64, in2: u8) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__UINT(in1: i64, in2: u16) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UDINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__UDINT(in1: i64, in2: u32) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by ULINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__ULINT(in1: i64, in2: u64) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2)
}

/// .
/// Divide LTIME by USINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__USINT(in1: i64, in2: u8) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide LTIME by UINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__UINT(in1: i64, in2: u16) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide LTIME by UDINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__UDINT(in1: i64, in2: u32) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide LTIME by ULINT
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__ULINT(in1: i64, in2: u64) -> i64 {
    checked_div_time_by_unsigned_int(in1, in2)
}

/// .
/// Divide TIME/LTIME with ANY_UNSIGNED_INT
/// Panic on overflow or division by zero
///
fn checked_div_time_by_unsigned_int(in1: i64, in2: u64) -> i64 {
    // convert in2 [u64] to [i64]
    // if in2 is to large for [i64] the division will allways fail -> panic on try_into()
    in1.checked_div(in2.try_into().unwrap()).unwrap()
}

/// .
/// Multiply TIME with REAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__REAL(in1: i64, in2: f32) -> i64 {
    checked_mul_time_with_f32(in1, in2)
}

/// .
/// Multiply TIME with REAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__REAL(in1: i64, in2: f32) -> i64 {
    checked_mul_time_with_f32(in1, in2)
}

/// .
/// Multiply LTIME with REAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__REAL(in1: i64, in2: f32) -> i64 {
    checked_mul_time_with_f32(in1, in2)
}

fn checked_mul_time_with_f32(in1: i64, in2: f32) -> i64 {
    // std::time::Duration can't handle negatives
    // we need to check for negative numbers and convert them to positives if necessary
    let is_in1_negative = in1.is_negative();
    let duration = std::time::Duration::from_nanos(in1.unsigned_abs());

    // if overflows i64 return panic
    let is_in2_negative = in2.is_sign_negative();
    let res: i64 = duration.mul_f32(in2.abs()).as_nanos().try_into().unwrap();

    // convert to negative if necessary
    let should_res_be_negative = is_in1_negative ^ is_in2_negative;
    match should_res_be_negative {
        true => -res,
        false => res,
    }
}

/// .
/// Multiply TIME with LREAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__LREAL(in1: i64, in2: f64) -> i64 {
    checked_mul_time_with_f64(in1, in2)
}

/// .
/// Multiply TIME with LREAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__LREAL(in1: i64, in2: f64) -> i64 {
    checked_mul_time_with_f64(in1, in2)
}

/// .
/// Multiply LTIME with LREAL
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__LREAL(in1: i64, in2: f64) -> i64 {
    checked_mul_time_with_f64(in1, in2)
}

fn checked_mul_time_with_f64(in1: i64, in2: f64) -> i64 {
    // std::time::Duration can't handle negatives
    // we need to check for negative numbers and convert them to positives if necessary
    let is_in1_negative = in1.is_negative();
    let duration = std::time::Duration::from_nanos(in1.unsigned_abs());

    // if overflows i64 return panic
    let is_in2_negative = in2.is_sign_negative();
    let res: i64 = duration.mul_f64(in2.abs()).as_nanos().try_into().unwrap();

    // convert to negative if necessary
    let should_res_be_negative = is_in1_negative ^ is_in2_negative;
    match should_res_be_negative {
        true => -res,
        false => res,
    }
}

/// .
/// Divide TIME by REAL
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__REAL(in1: i64, in2: f32) -> i64 {
    checked_div_time_by_f32(in1, in2)
}

/// .
/// Divide TIME by REAL
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__REAL(in1: i64, in2: f32) -> i64 {
    checked_div_time_by_f32(in1, in2)
}

/// .
/// Divide LTIME by REAL
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__REAL(in1: i64, in2: f32) -> i64 {
    checked_div_time_by_f32(in1, in2)
}

fn checked_div_time_by_f32(in1: i64, in2: f32) -> i64 {
    // std::time::Duration can't handle negatives
    // we need to check for negative numbers and convert them to positives if necessary
    let is_in1_negative = in1.is_negative();
    let duration = std::time::Duration::from_nanos(in1.unsigned_abs());

    // if overflows i64 return panic
    let is_in2_negative = in2.is_sign_negative();
    let res: i64 = duration.div_f32(in2.abs()).as_nanos().try_into().unwrap();

    // convert to negative if necessary
    let should_res_be_negative = is_in1_negative ^ is_in2_negative;
    match should_res_be_negative {
        true => -res,
        false => res,
    }
}

/// .
/// Divide TIME by LREAL
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__LREAL(in1: i64, in2: f64) -> i64 {
    checked_div_time_by_f64(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by SINT.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__SINT(in1: i64, in2: i8) -> i64 {
    MUL_LTIME__SINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by INT.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__INT(in1: i64, in2: i16) -> i64 {
    MUL_LTIME__INT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by DINT.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__DINT(in1: i64, in2: i32) -> i64 {
    MUL_LTIME__DINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by LINT.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__LINT(in1: i64, in2: i64) -> i64 {
    MUL_LTIME__LINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by USINT.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__USINT(in1: i64, in2: u8) -> i64 {
    MUL_LTIME__USINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by UINT.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__UINT(in1: i64, in2: u16) -> i64 {
    MUL_LTIME__UINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by UDINT.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__UDINT(in1: i64, in2: u32) -> i64 {
    MUL_LTIME__UDINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by ULINT.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__ULINT(in1: i64, in2: u64) -> i64 {
    MUL_LTIME__ULINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by REAL.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__REAL(in1: i64, in2: f32) -> i64 {
    MUL_LTIME__REAL(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by LREAL.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__LREAL(in1: i64, in2: f64) -> i64 {
    MUL_LTIME__LREAL(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by SINT.
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__SINT(in1: i64, in2: i8) -> i64 {
    DIV_LTIME__SINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by INT.
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__INT(in1: i64, in2: i16) -> i64 {
    DIV_LTIME__INT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by DINT.
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__DINT(in1: i64, in2: i32) -> i64 {
    DIV_LTIME__DINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by LINT.
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__LINT(in1: i64, in2: i64) -> i64 {
    DIV_LTIME__LINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by USINT.
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__USINT(in1: i64, in2: u8) -> i64 {
    DIV_LTIME__USINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by UINT.
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__UINT(in1: i64, in2: u16) -> i64 {
    DIV_LTIME__UINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by UDINT.
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__UDINT(in1: i64, in2: u32) -> i64 {
    DIV_LTIME__UDINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by ULINT.
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__ULINT(in1: i64, in2: u64) -> i64 {
    DIV_LTIME__ULINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by REAL.
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__REAL(in1: i64, in2: f32) -> i64 {
    DIV_LTIME__REAL(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by LREAL.
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__LREAL(in1: i64, in2: f64) -> i64 {
    DIV_LTIME__LREAL(in1, in2)
}

/// .
/// Compatibility symbol for LTIME + LTIME overload resolution.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD__LTIME__LTIME(in1: i64, in2: i64) -> i64 {
    in1.checked_add(in2).unwrap()
}

/// .
/// Compatibility symbol for LTOD + LTIME overload resolution.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD__LTOD__LTIME(in1: i64, in2: i64) -> i64 {
    add_datetime_time(in1, in2)
}

/// .
/// Compatibility symbol for LDT + LTIME overload resolution.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD__LDT__LTIME(in1: i64, in2: i64) -> i64 {
    add_datetime_time(in1, in2)
}

/// .
/// Compatibility symbol for LTIME - LTIME overload resolution.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LTIME__LTIME(in1: i64, in2: i64) -> i64 {
    in1.checked_sub(in2).unwrap()
}

/// .
/// Compatibility symbol for LDATE - LDATE overload resolution.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LDATE__LDATE(in1: i64, in2: i64) -> i64 {
    sub_datetimes(in1, in2)
}

/// .
/// Compatibility symbol for LTOD - LTIME overload resolution.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LTOD__LTIME(in1: i64, in2: i64) -> i64 {
    sub_datetime_duration(in1, in2)
}

/// .
/// Compatibility symbol for LTOD - LTOD overload resolution.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LTOD__LTOD(in1: i64, in2: i64) -> i64 {
    sub_datetimes(in1, in2)
}

/// .
/// Compatibility symbol for LDT - LTIME overload resolution.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LDT__LTIME(in1: i64, in2: i64) -> i64 {
    sub_datetime_duration(in1, in2)
}

/// .
/// Compatibility symbol for LDT - LDT overload resolution.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LDT__LDT(in1: i64, in2: i64) -> i64 {
    sub_datetimes(in1, in2)
}

/// .
/// Compatibility alias for LDATE_AND_TIME + LTIME.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD__LDATE_AND_TIME__LTIME(in1: i64, in2: i64) -> i64 {
    ADD__LDT__LTIME(in1, in2)
}

/// .
/// Compatibility alias for LTIME_OF_DAY + LTIME.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD__LTIME_OF_DAY__LTIME(in1: i64, in2: i64) -> i64 {
    ADD__LTOD__LTIME(in1, in2)
}

/// .
/// Compatibility alias for LDATE_AND_TIME - LTIME.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LDATE_AND_TIME__LTIME(in1: i64, in2: i64) -> i64 {
    SUB__LDT__LTIME(in1, in2)
}

/// .
/// Compatibility alias for LDATE_AND_TIME - LDATE_AND_TIME.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LDATE_AND_TIME__LDATE_AND_TIME(in1: i64, in2: i64) -> i64 {
    SUB__LDT__LDT(in1, in2)
}

/// .
/// Compatibility alias for LTIME_OF_DAY - LTIME.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LTIME_OF_DAY__LTIME(in1: i64, in2: i64) -> i64 {
    SUB__LTOD__LTIME(in1, in2)
}

/// .
/// Compatibility alias for LTIME_OF_DAY - LTIME_OF_DAY.
/// Panic on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LTIME_OF_DAY__LTIME_OF_DAY(in1: i64, in2: i64) -> i64 {
    SUB__LTOD__LTOD(in1, in2)
}

/// .
/// Divide TIME by LREAL
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__LREAL(in1: i64, in2: f64) -> i64 {
    checked_div_time_by_f64(in1, in2)
}

/// .
/// Divide LTIME by LREAL
/// Panic on overflow or division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__LREAL(in1: i64, in2: f64) -> i64 {
    checked_div_time_by_f64(in1, in2)
}

fn checked_div_time_by_f64(in1: i64, in2: f64) -> i64 {
    // std::time::Duration can't handle negatives
    // we need to check for negative numbers and convert them to positives if necessary
    let is_in1_negative = in1.is_negative();
    let duration = std::time::Duration::from_nanos(in1.unsigned_abs());

    // if overflows i64 return panic
    let is_in2_negative = in2.is_sign_negative();
    let res: i64 = duration.div_f64(in2.abs()).as_nanos().try_into().unwrap();

    // convert to negative if necessary
    let should_res_be_negative = is_in1_negative ^ is_in2_negative;
    match should_res_be_negative {
        true => -res,
        false => res,
    }
}
