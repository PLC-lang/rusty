const MILLIS_PER_SECOND: u32 = 1_000;
const MILLIS_PER_DAY: u32 = 60 * 60 * 24 * MILLIS_PER_SECOND;

fn millis_to_seconds(input: u32) -> u32 {
    input / MILLIS_PER_SECOND
}

fn wrapping_seconds_to_millis(input: u32) -> u32 {
    input.wrapping_mul(MILLIS_PER_SECOND)
}

/// .
/// This operator returns the value of adding up two TIME operands.
/// Wraps on overflow.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_TIME(in1: u32, in2: u32) -> u32 {
    in1.wrapping_add(in2)
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
/// Wraps on overflow.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_DT_TIME(in1: u32, in2: u32) -> u32 {
    let time_seconds = millis_to_seconds(in2);
    in1.wrapping_add(time_seconds)
}

fn add_datetime_time(in1: i64, in2: i64) -> i64 {
    in1.wrapping_add(in2)
}

/// .
/// This operator produces the subtraction of two TIME operands
/// Wraps on underflow.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_TIME(in1: u32, in2: u32) -> u32 {
    in1.wrapping_sub(in2)
}

/// .
/// This operator produces the subtraction of two DATE operands
/// Wraps on underflow and when the resulting TIME exceeds the TIME range.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_DATE_DATE(in1: u32, in2: u32) -> u32 {
    wrapping_seconds_to_millis(in1.wrapping_sub(in2))
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
/// Wraps on underflow.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_TOD_TOD(in1: u32, in2: u32) -> u32 {
    in1.wrapping_sub(in2)
}

fn sub_datetimes(in1: i64, in2: i64) -> i64 {
    in1.wrapping_sub(in2)
}

/// .
/// This operator produces the subtraction of DT and TIME
/// Wraps on underflow.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_DT_TIME(in1: u32, in2: u32) -> u32 {
    let time_seconds = millis_to_seconds(in2);
    in1.wrapping_sub(time_seconds)
}

fn sub_datetime_duration(in1: i64, in2: i64) -> i64 {
    in1.wrapping_sub(in2)
}

/// .
/// This operator produces the subtraction of two DT operands
/// Wraps on underflow and when the resulting TIME exceeds the TIME range.
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_DT_DT(in1: u32, in2: u32) -> u32 {
    wrapping_seconds_to_millis(in1.wrapping_sub(in2))
}

/// .
/// This operator returns the value of adding up two LTIME operands.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_LTIME(in1: i64, in2: i64) -> i64 {
    ADD__LTIME__LTIME(in1, in2)
}

/// .
/// This operator returns the value of adding up LTOD and LTIME.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_LTOD_LTIME(in1: i64, in2: i64) -> i64 {
    ADD__LTOD__LTIME(in1, in2)
}

/// .
/// This operator returns the value of adding up LDT and LTIME.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_LDT_LTIME(in1: i64, in2: i64) -> i64 {
    ADD__LDT__LTIME(in1, in2)
}

/// .
/// This operator produces the subtraction of two LTIME operands.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_LTIME(in1: i64, in2: i64) -> i64 {
    SUB__LTIME__LTIME(in1, in2)
}

/// .
/// This operator produces the subtraction of two LDATE operands.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_LDATE_LDATE(in1: i64, in2: i64) -> i64 {
    SUB__LDATE__LDATE(in1, in2)
}

/// .
/// This operator produces the subtraction of LTOD and LTIME.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_LTOD_LTIME(in1: i64, in2: i64) -> i64 {
    SUB__LTOD__LTIME(in1, in2)
}

/// .
/// This operator produces the subtraction of two LTOD operands.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_LTOD_LTOD(in1: i64, in2: i64) -> i64 {
    SUB__LTOD__LTOD(in1, in2)
}

/// .
/// This operator produces the subtraction of LDT and LTIME.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_LDT_LTIME(in1: i64, in2: i64) -> i64 {
    SUB__LDT__LTIME(in1, in2)
}

/// .
/// This operator produces the subtraction of two LDT operands.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_LDT_LDT(in1: i64, in2: i64) -> i64 {
    SUB__LDT__LDT(in1, in2)
}

/// .
/// Multiply TIME with SINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__SINT(in1: i64, in2: i8) -> i64 {
    wrapping_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with INT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__INT(in1: i64, in2: i16) -> i64 {
    wrapping_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with DINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__DINT(in1: i64, in2: i32) -> i64 {
    wrapping_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with LINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__LINT(in1: i64, in2: i64) -> i64 {
    wrapping_mul_time_with_signed_int(in1, in2)
}

/// .
/// Multiply TIME with SINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__SINT(in1: i64, in2: i8) -> i64 {
    wrapping_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with INT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__INT(in1: i64, in2: i16) -> i64 {
    wrapping_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with DINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__DINT(in1: i64, in2: i32) -> i64 {
    wrapping_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with LINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__LINT(in1: i64, in2: i64) -> i64 {
    wrapping_mul_time_with_signed_int(in1, in2)
}

/// .
/// Multiply LTIME with SINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__SINT(in1: i64, in2: i8) -> i64 {
    wrapping_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply LTIME with INT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__INT(in1: i64, in2: i16) -> i64 {
    wrapping_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply LTIME with DINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__DINT(in1: i64, in2: i32) -> i64 {
    wrapping_mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply LTIME with LINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__LINT(in1: i64, in2: i64) -> i64 {
    wrapping_mul_time_with_signed_int(in1, in2)
}

/// .
/// Multiply TIME/LTIME with ANY_SIGNED_INT
/// Wraps on overflow
///
fn wrapping_mul_time_with_signed_int(in1: i64, in2: i64) -> i64 {
    in1.wrapping_mul(in2)
}

/// .
/// Multiply TIME with USINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__USINT(in1: i64, in2: u8) -> i64 {
    wrapping_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__UINT(in1: i64, in2: u16) -> i64 {
    wrapping_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UDINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__UDINT(in1: i64, in2: u32) -> i64 {
    wrapping_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with ULINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__ULINT(in1: i64, in2: u64) -> i64 {
    wrapping_mul_time_with_unsigned_int(in1, in2)
}

/// .
/// Multiply TIME with USINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__USINT(in1: i64, in2: u8) -> i64 {
    wrapping_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__UINT(in1: i64, in2: u16) -> i64 {
    wrapping_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UDINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__UDINT(in1: i64, in2: u32) -> i64 {
    wrapping_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with ULINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__ULINT(in1: i64, in2: u64) -> i64 {
    wrapping_mul_time_with_unsigned_int(in1, in2)
}

/// .
/// Multiply LTIME with USINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__USINT(in1: i64, in2: u8) -> i64 {
    wrapping_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply LTIME with UINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__UINT(in1: i64, in2: u16) -> i64 {
    wrapping_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply LTIME with UDINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__UDINT(in1: i64, in2: u32) -> i64 {
    wrapping_mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply LTIME with ULINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__ULINT(in1: i64, in2: u64) -> i64 {
    wrapping_mul_time_with_unsigned_int(in1, in2)
}

/// .
/// Multiply TIME/LTIME with ANY_UNSIGNED_INT
/// Wraps on overflow
///
fn wrapping_mul_time_with_unsigned_int(in1: i64, in2: u64) -> i64 {
    // modular 64-bit arithmetic: the cast wraps factors beyond the i64 range,
    // matching two's-complement multiplication
    in1.wrapping_mul(in2 as i64)
}

/// .
/// Divide TIME by SINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__SINT(in1: i64, in2: i8) -> i64 {
    wrapping_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by INT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__INT(in1: i64, in2: i16) -> i64 {
    wrapping_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by DINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__DINT(in1: i64, in2: i32) -> i64 {
    wrapping_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by LINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__LINT(in1: i64, in2: i64) -> i64 {
    wrapping_div_time_by_signed_int(in1, in2)
}

/// .
/// Divide TIME by SINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__SINT(in1: i64, in2: i8) -> i64 {
    wrapping_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by INT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__INT(in1: i64, in2: i16) -> i64 {
    wrapping_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by DINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__DINT(in1: i64, in2: i32) -> i64 {
    wrapping_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by LINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__LINT(in1: i64, in2: i64) -> i64 {
    wrapping_div_time_by_signed_int(in1, in2)
}

/// .
/// Divide LTIME by SINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__SINT(in1: i64, in2: i8) -> i64 {
    wrapping_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide LTIME by INT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__INT(in1: i64, in2: i16) -> i64 {
    wrapping_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide LTIME by DINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__DINT(in1: i64, in2: i32) -> i64 {
    wrapping_div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide LTIME by LINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__LINT(in1: i64, in2: i64) -> i64 {
    wrapping_div_time_by_signed_int(in1, in2)
}

/// .
/// Divide TIME/LTIME with ANY_SIGNED_INT
/// Division by zero yields zero; wraps on overflow
///
fn wrapping_div_time_by_signed_int(in1: i64, in2: i64) -> i64 {
    if in2 == 0 {
        return 0;
    }
    in1.wrapping_div(in2)
}

/// .
/// Divide TIME by USINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__USINT(in1: i64, in2: u8) -> i64 {
    wrapping_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__UINT(in1: i64, in2: u16) -> i64 {
    wrapping_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UDINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__UDINT(in1: i64, in2: u32) -> i64 {
    wrapping_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by ULINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__ULINT(in1: i64, in2: u64) -> i64 {
    wrapping_div_time_by_unsigned_int(in1, in2)
}

/// .
/// Divide TIME by USINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__USINT(in1: i64, in2: u8) -> i64 {
    wrapping_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__UINT(in1: i64, in2: u16) -> i64 {
    wrapping_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UDINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__UDINT(in1: i64, in2: u32) -> i64 {
    wrapping_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by ULINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__ULINT(in1: i64, in2: u64) -> i64 {
    wrapping_div_time_by_unsigned_int(in1, in2)
}

/// .
/// Divide LTIME by USINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__USINT(in1: i64, in2: u8) -> i64 {
    wrapping_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide LTIME by UINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__UINT(in1: i64, in2: u16) -> i64 {
    wrapping_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide LTIME by UDINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__UDINT(in1: i64, in2: u32) -> i64 {
    wrapping_div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide LTIME by ULINT
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__ULINT(in1: i64, in2: u64) -> i64 {
    wrapping_div_time_by_unsigned_int(in1, in2)
}

/// .
/// Divide TIME/LTIME with ANY_UNSIGNED_INT
/// Division by zero yields zero
///
fn wrapping_div_time_by_unsigned_int(in1: i64, in2: u64) -> i64 {
    if in2 == 0 {
        return 0;
    }
    // a divisor beyond the i64 range exceeds any possible dividend magnitude,
    // so the quotient is always zero
    let Ok(divisor) = i64::try_from(in2) else {
        return 0;
    };
    in1.wrapping_div(divisor)
}

/// .
/// Multiply TIME with REAL
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__REAL(in1: i64, in2: f32) -> i64 {
    mul_time_with_f32(in1, in2)
}

/// .
/// Multiply TIME with REAL
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__REAL(in1: i64, in2: f32) -> i64 {
    mul_time_with_f32(in1, in2)
}

/// .
/// Multiply LTIME with REAL
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__REAL(in1: i64, in2: f32) -> i64 {
    mul_time_with_f32(in1, in2)
}

fn mul_time_with_f32(in1: i64, in2: f32) -> i64 {
    mul_time_with_f64(in1, in2 as f64)
}

/// .
/// Multiply TIME with LREAL
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__LREAL(in1: i64, in2: f64) -> i64 {
    mul_time_with_f64(in1, in2)
}

/// .
/// Multiply TIME with LREAL
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__LREAL(in1: i64, in2: f64) -> i64 {
    mul_time_with_f64(in1, in2)
}

/// .
/// Multiply LTIME with LREAL
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__LREAL(in1: i64, in2: f64) -> i64 {
    mul_time_with_f64(in1, in2)
}

fn mul_time_with_f64(in1: i64, in2: f64) -> i64 {
    let res = in1 as f64 * in2;
    if res.is_nan() {
        return 0;
    }
    // the float-to-int cast saturates at the i64 range
    res as i64
}

/// .
/// Divide TIME by REAL
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__REAL(in1: i64, in2: f32) -> i64 {
    div_time_by_f32(in1, in2)
}

/// .
/// Divide TIME by REAL
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__REAL(in1: i64, in2: f32) -> i64 {
    div_time_by_f32(in1, in2)
}

/// .
/// Divide LTIME by REAL
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__REAL(in1: i64, in2: f32) -> i64 {
    div_time_by_f32(in1, in2)
}

fn div_time_by_f32(in1: i64, in2: f32) -> i64 {
    div_time_by_f64(in1, in2 as f64)
}

/// .
/// Divide TIME by LREAL
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__LREAL(in1: i64, in2: f64) -> i64 {
    div_time_by_f64(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by SINT.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__SINT(in1: i64, in2: i8) -> i64 {
    MUL_LTIME__SINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by INT.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__INT(in1: i64, in2: i16) -> i64 {
    MUL_LTIME__INT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by DINT.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__DINT(in1: i64, in2: i32) -> i64 {
    MUL_LTIME__DINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by LINT.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__LINT(in1: i64, in2: i64) -> i64 {
    MUL_LTIME__LINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by USINT.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__USINT(in1: i64, in2: u8) -> i64 {
    MUL_LTIME__USINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by UINT.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__UINT(in1: i64, in2: u16) -> i64 {
    MUL_LTIME__UINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by UDINT.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__UDINT(in1: i64, in2: u32) -> i64 {
    MUL_LTIME__UDINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by ULINT.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__ULINT(in1: i64, in2: u64) -> i64 {
    MUL_LTIME__ULINT(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by REAL.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__REAL(in1: i64, in2: f32) -> i64 {
    MUL_LTIME__REAL(in1, in2)
}

/// .
/// Compatibility alias for multiplying LTIME by LREAL.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__LTIME__LREAL(in1: i64, in2: f64) -> i64 {
    MUL_LTIME__LREAL(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by SINT.
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__SINT(in1: i64, in2: i8) -> i64 {
    DIV_LTIME__SINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by INT.
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__INT(in1: i64, in2: i16) -> i64 {
    DIV_LTIME__INT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by DINT.
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__DINT(in1: i64, in2: i32) -> i64 {
    DIV_LTIME__DINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by LINT.
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__LINT(in1: i64, in2: i64) -> i64 {
    DIV_LTIME__LINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by USINT.
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__USINT(in1: i64, in2: u8) -> i64 {
    DIV_LTIME__USINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by UINT.
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__UINT(in1: i64, in2: u16) -> i64 {
    DIV_LTIME__UINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by UDINT.
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__UDINT(in1: i64, in2: u32) -> i64 {
    DIV_LTIME__UDINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by ULINT.
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__ULINT(in1: i64, in2: u64) -> i64 {
    DIV_LTIME__ULINT(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by REAL.
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__REAL(in1: i64, in2: f32) -> i64 {
    DIV_LTIME__REAL(in1, in2)
}

/// .
/// Compatibility alias for dividing LTIME by LREAL.
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__LTIME__LREAL(in1: i64, in2: f64) -> i64 {
    DIV_LTIME__LREAL(in1, in2)
}

/// .
/// Compatibility symbol for LTIME + LTIME overload resolution.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD__LTIME__LTIME(in1: i64, in2: i64) -> i64 {
    in1.wrapping_add(in2)
}

/// .
/// Compatibility symbol for LTOD + LTIME overload resolution.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD__LTOD__LTIME(in1: i64, in2: i64) -> i64 {
    add_datetime_time(in1, in2)
}

/// .
/// Compatibility symbol for LDT + LTIME overload resolution.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD__LDT__LTIME(in1: i64, in2: i64) -> i64 {
    add_datetime_time(in1, in2)
}

/// .
/// Compatibility symbol for LTIME - LTIME overload resolution.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LTIME__LTIME(in1: i64, in2: i64) -> i64 {
    in1.wrapping_sub(in2)
}

/// .
/// Compatibility symbol for LDATE - LDATE overload resolution.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LDATE__LDATE(in1: i64, in2: i64) -> i64 {
    sub_datetimes(in1, in2)
}

/// .
/// Compatibility symbol for LTOD - LTIME overload resolution.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LTOD__LTIME(in1: i64, in2: i64) -> i64 {
    sub_datetime_duration(in1, in2)
}

/// .
/// Compatibility symbol for LTOD - LTOD overload resolution.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LTOD__LTOD(in1: i64, in2: i64) -> i64 {
    sub_datetimes(in1, in2)
}

/// .
/// Compatibility symbol for LDT - LTIME overload resolution.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LDT__LTIME(in1: i64, in2: i64) -> i64 {
    sub_datetime_duration(in1, in2)
}

/// .
/// Compatibility symbol for LDT - LDT overload resolution.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LDT__LDT(in1: i64, in2: i64) -> i64 {
    sub_datetimes(in1, in2)
}

/// .
/// Compatibility alias for LDATE_AND_TIME + LTIME.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD__LDATE_AND_TIME__LTIME(in1: i64, in2: i64) -> i64 {
    ADD__LDT__LTIME(in1, in2)
}

/// .
/// Compatibility alias for LTIME_OF_DAY + LTIME.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD__LTIME_OF_DAY__LTIME(in1: i64, in2: i64) -> i64 {
    ADD__LTOD__LTIME(in1, in2)
}

/// .
/// Compatibility alias for LDATE_AND_TIME - LTIME.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LDATE_AND_TIME__LTIME(in1: i64, in2: i64) -> i64 {
    SUB__LDT__LTIME(in1, in2)
}

/// .
/// Compatibility alias for LDATE_AND_TIME - LDATE_AND_TIME.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LDATE_AND_TIME__LDATE_AND_TIME(in1: i64, in2: i64) -> i64 {
    SUB__LDT__LDT(in1, in2)
}

/// .
/// Compatibility alias for LTIME_OF_DAY - LTIME.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LTIME_OF_DAY__LTIME(in1: i64, in2: i64) -> i64 {
    SUB__LTOD__LTIME(in1, in2)
}

/// .
/// Compatibility alias for LTIME_OF_DAY - LTIME_OF_DAY.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB__LTIME_OF_DAY__LTIME_OF_DAY(in1: i64, in2: i64) -> i64 {
    SUB__LTOD__LTOD(in1, in2)
}

/// .
/// Divide TIME by LREAL
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__LREAL(in1: i64, in2: f64) -> i64 {
    div_time_by_f64(in1, in2)
}

/// .
/// Divide LTIME by LREAL
/// Wraps on overflow; division by zero yields zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__LREAL(in1: i64, in2: f64) -> i64 {
    div_time_by_f64(in1, in2)
}

fn div_time_by_f64(in1: i64, in2: f64) -> i64 {
    if in2 == 0.0 || in2.is_nan() {
        return 0;
    }
    // the float-to-int cast saturates at the i64 range
    (in1 as f64 / in2) as i64
}
