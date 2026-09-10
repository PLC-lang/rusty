/// .
/// This operator returns the value of adding up two TIME operands.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_TIME(in1: i64, in2: i64) -> i64 {
    in1.wrapping_add(in2)
}

/// .
/// This operator returns the value of adding up TOD and TIME.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_TOD_TIME(in1: i64, in2: i64) -> i64 {
    in1.wrapping_add(in2)
}

/// .
/// This operator returns the value of adding up DT and TIME.
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn ADD_DT_TIME(in1: i64, in2: i64) -> i64 {
    in1.wrapping_add(in2)
}

/// .
/// This operator produces the subtraction of two TIME operands
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_TIME(in1: i64, in2: i64) -> i64 {
    in1.wrapping_sub(in2)
}

/// .
/// This operator produces the subtraction of two DATE operands
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_DATE_DATE(in1: i64, in2: i64) -> i64 {
    in1.wrapping_sub(in2)
}

/// .
/// This operator produces the subtraction of TOD and TIME
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_TOD_TIME(in1: i64, in2: i64) -> i64 {
    in1.wrapping_sub(in2)
}

/// .
/// This operator produces the subtraction of two TOD operands
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_TOD_TOD(in1: i64, in2: i64) -> i64 {
    in1.wrapping_sub(in2)
}

/// .
/// This operator produces the subtraction of DT and TIME
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_DT_TIME(in1: i64, in2: i64) -> i64 {
    in1.wrapping_sub(in2)
}

/// .
/// This operator produces the subtraction of two DT operands
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn SUB_DT_DT(in1: i64, in2: i64) -> i64 {
    in1.wrapping_sub(in2)
}

/// .
/// Multiply TIME with SINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__SINT(in1: i64, in2: i8) -> i64 {
    mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with INT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__INT(in1: i64, in2: i16) -> i64 {
    mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with DINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__DINT(in1: i64, in2: i32) -> i64 {
    mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with LINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__LINT(in1: i64, in2: i64) -> i64 {
    mul_time_with_signed_int(in1, in2)
}

/// .
/// Multiply TIME with SINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__SINT(in1: i64, in2: i8) -> i64 {
    mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with INT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__INT(in1: i64, in2: i16) -> i64 {
    mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with DINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__DINT(in1: i64, in2: i32) -> i64 {
    mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply TIME with LINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__LINT(in1: i64, in2: i64) -> i64 {
    mul_time_with_signed_int(in1, in2)
}

/// .
/// Multiply LTIME with SINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__SINT(in1: i64, in2: i8) -> i64 {
    mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply LTIME with INT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__INT(in1: i64, in2: i16) -> i64 {
    mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply LTIME with DINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__DINT(in1: i64, in2: i32) -> i64 {
    mul_time_with_signed_int(in1, in2.into())
}

/// .
/// Multiply LTIME with LINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__LINT(in1: i64, in2: i64) -> i64 {
    mul_time_with_signed_int(in1, in2)
}

/// .
/// Multiply TIME/LTIME with ANY_SIGNED_INT
/// Wraps on overflow
///
fn mul_time_with_signed_int(in1: i64, in2: i64) -> i64 {
    in1.wrapping_mul(in2)
}

/// .
/// Multiply TIME with USINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__USINT(in1: i64, in2: u8) -> i64 {
    mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__UINT(in1: i64, in2: u16) -> i64 {
    mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UDINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__UDINT(in1: i64, in2: u32) -> i64 {
    mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with ULINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__ULINT(in1: i64, in2: u64) -> i64 {
    mul_time_with_unsigned_int(in1, in2)
}

/// .
/// Multiply TIME with USINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__USINT(in1: i64, in2: u8) -> i64 {
    mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__UINT(in1: i64, in2: u16) -> i64 {
    mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with UDINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__UDINT(in1: i64, in2: u32) -> i64 {
    mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply TIME with ULINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__ULINT(in1: i64, in2: u64) -> i64 {
    mul_time_with_unsigned_int(in1, in2)
}

/// .
/// Multiply LTIME with USINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__USINT(in1: i64, in2: u8) -> i64 {
    mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply LTIME with UINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__UINT(in1: i64, in2: u16) -> i64 {
    mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply LTIME with UDINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__UDINT(in1: i64, in2: u32) -> i64 {
    mul_time_with_unsigned_int(in1, in2.into())
}

/// .
/// Multiply LTIME with ULINT
/// Wraps on overflow
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__ULINT(in1: i64, in2: u64) -> i64 {
    mul_time_with_unsigned_int(in1, in2)
}

/// .
/// Multiply TIME/LTIME with ANY_UNSIGNED_INT
/// Wraps on overflow
///
fn mul_time_with_unsigned_int(in1: i64, in2: u64) -> i64 {
    // the u64 to i64 cast keeps the result correct modulo 2^64
    in1.wrapping_mul(in2 as i64)
}

/// .
/// Divide TIME by SINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__SINT(in1: i64, in2: i8) -> i64 {
    div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by INT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__INT(in1: i64, in2: i16) -> i64 {
    div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by DINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__DINT(in1: i64, in2: i32) -> i64 {
    div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by LINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__LINT(in1: i64, in2: i64) -> i64 {
    div_time_by_signed_int(in1, in2)
}

/// .
/// Divide TIME by SINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__SINT(in1: i64, in2: i8) -> i64 {
    div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by INT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__INT(in1: i64, in2: i16) -> i64 {
    div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by DINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__DINT(in1: i64, in2: i32) -> i64 {
    div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide TIME by LINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__LINT(in1: i64, in2: i64) -> i64 {
    div_time_by_signed_int(in1, in2)
}

/// .
/// Divide LTIME by SINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__SINT(in1: i64, in2: i8) -> i64 {
    div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide LTIME by INT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__INT(in1: i64, in2: i16) -> i64 {
    div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide LTIME by DINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__DINT(in1: i64, in2: i32) -> i64 {
    div_time_by_signed_int(in1, in2.into())
}

/// .
/// Divide LTIME by LINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__LINT(in1: i64, in2: i64) -> i64 {
    div_time_by_signed_int(in1, in2)
}

/// .
/// Divide TIME/LTIME with ANY_SIGNED_INT
/// Panics on division by zero
///
fn div_time_by_signed_int(in1: i64, in2: i64) -> i64 {
    in1.wrapping_div(in2)
}

/// .
/// Divide TIME by USINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__USINT(in1: i64, in2: u8) -> i64 {
    div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__UINT(in1: i64, in2: u16) -> i64 {
    div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UDINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__UDINT(in1: i64, in2: u32) -> i64 {
    div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by ULINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__ULINT(in1: i64, in2: u64) -> i64 {
    div_time_by_unsigned_int(in1, in2)
}

/// .
/// Divide TIME by USINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__USINT(in1: i64, in2: u8) -> i64 {
    div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__UINT(in1: i64, in2: u16) -> i64 {
    div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by UDINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__UDINT(in1: i64, in2: u32) -> i64 {
    div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide TIME by ULINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__ULINT(in1: i64, in2: u64) -> i64 {
    div_time_by_unsigned_int(in1, in2)
}

/// .
/// Divide LTIME by USINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__USINT(in1: i64, in2: u8) -> i64 {
    div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide LTIME by UINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__UINT(in1: i64, in2: u16) -> i64 {
    div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide LTIME by UDINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__UDINT(in1: i64, in2: u32) -> i64 {
    div_time_by_unsigned_int(in1, in2.into())
}

/// .
/// Divide LTIME by ULINT
/// Panics on division by zero
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__ULINT(in1: i64, in2: u64) -> i64 {
    div_time_by_unsigned_int(in1, in2)
}

/// .
/// Divide TIME/LTIME with ANY_UNSIGNED_INT
/// Panics on division by zero
///
fn div_time_by_unsigned_int(in1: i64, in2: u64) -> i64 {
    // a divisor above the signed range always exceeds the dividend magnitude
    match i64::try_from(in2) {
        Ok(divisor) => in1.wrapping_div(divisor),
        Err(_) => 0,
    }
}

/// .
/// Multiply TIME with REAL
/// A NaN factor yields zero, overflow saturates
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__REAL(in1: i64, in2: f32) -> i64 {
    mul_time_with_f32(in1, in2)
}

/// .
/// Multiply TIME with REAL
/// A NaN factor yields zero, overflow saturates
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__REAL(in1: i64, in2: f32) -> i64 {
    mul_time_with_f32(in1, in2)
}

/// .
/// Multiply LTIME with REAL
/// A NaN factor yields zero, overflow saturates
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__REAL(in1: i64, in2: f32) -> i64 {
    mul_time_with_f32(in1, in2)
}

fn mul_time_with_f32(in1: i64, in2: f32) -> i64 {
    if in2.is_nan() {
        return 0;
    }
    let negative = in1.is_negative() ^ in2.is_sign_negative();
    let magnitude = std::time::Duration::from_nanos(in1.unsigned_abs());
    // pre-check in f64 so the Duration math below can never panic on overflow;
    // NaN only occurs for zero times an infinite factor
    let approx_nanos = magnitude.as_secs_f64() * f64::from(in2.abs()) * 1e9;
    let res: i64 = if approx_nanos.is_nan() {
        0
    } else if approx_nanos >= i64::MAX as f64 {
        i64::MAX
    } else {
        magnitude.mul_f32(in2.abs()).as_nanos().try_into().unwrap_or(i64::MAX)
    };
    if negative {
        -res
    } else {
        res
    }
}

/// .
/// Multiply TIME with LREAL
/// A NaN factor yields zero, overflow saturates
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL__TIME__LREAL(in1: i64, in2: f64) -> i64 {
    mul_time_with_f64(in1, in2)
}

/// .
/// Multiply TIME with LREAL
/// A NaN factor yields zero, overflow saturates
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_TIME__LREAL(in1: i64, in2: f64) -> i64 {
    mul_time_with_f64(in1, in2)
}

/// .
/// Multiply LTIME with LREAL
/// A NaN factor yields zero, overflow saturates
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn MUL_LTIME__LREAL(in1: i64, in2: f64) -> i64 {
    mul_time_with_f64(in1, in2)
}

fn mul_time_with_f64(in1: i64, in2: f64) -> i64 {
    if in2.is_nan() {
        return 0;
    }
    let negative = in1.is_negative() ^ in2.is_sign_negative();
    let magnitude = std::time::Duration::from_nanos(in1.unsigned_abs());
    // pre-check in f64 so the Duration math below can never panic on overflow;
    // NaN only occurs for zero times an infinite factor
    let approx_nanos = magnitude.as_secs_f64() * in2.abs() * 1e9;
    let res: i64 = if approx_nanos.is_nan() {
        0
    } else if approx_nanos >= i64::MAX as f64 {
        i64::MAX
    } else {
        magnitude.mul_f64(in2.abs()).as_nanos().try_into().unwrap_or(i64::MAX)
    };
    if negative {
        -res
    } else {
        res
    }
}

/// .
/// Divide TIME by REAL
/// A zero divisor saturates at the TIME range, a NaN divisor or result yields zero, overflow saturates
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__REAL(in1: i64, in2: f32) -> i64 {
    div_time_by_f32(in1, in2)
}

/// .
/// Divide TIME by REAL
/// A zero divisor saturates at the TIME range, a NaN divisor or result yields zero, overflow saturates
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__REAL(in1: i64, in2: f32) -> i64 {
    div_time_by_f32(in1, in2)
}

/// .
/// Divide LTIME by REAL
/// A zero divisor saturates at the TIME range, a NaN divisor or result yields zero, overflow saturates
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__REAL(in1: i64, in2: f32) -> i64 {
    div_time_by_f32(in1, in2)
}

fn div_time_by_f32(in1: i64, in2: f32) -> i64 {
    let negative = in1.is_negative() ^ in2.is_sign_negative();
    let magnitude = std::time::Duration::from_nanos(in1.unsigned_abs());
    // pre-check in f64 so the Duration math below can never panic on overflow;
    // a NaN result here covers both a NaN divisor and zero divided by zero
    let approx_nanos = magnitude.as_secs_f64() / f64::from(in2.abs()) * 1e9;
    if approx_nanos.is_nan() {
        return 0;
    }
    let res: i64 = if approx_nanos >= i64::MAX as f64 {
        i64::MAX
    } else {
        magnitude.div_f32(in2.abs()).as_nanos().try_into().unwrap_or(i64::MAX)
    };
    if negative {
        -res
    } else {
        res
    }
}

/// .
/// Divide TIME by LREAL
/// A zero divisor saturates at the TIME range, a NaN divisor or result yields zero, overflow saturates
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV__TIME__LREAL(in1: i64, in2: f64) -> i64 {
    div_time_by_f64(in1, in2)
}

/// .
/// Divide TIME by LREAL
/// A zero divisor saturates at the TIME range, a NaN divisor or result yields zero, overflow saturates
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_TIME__LREAL(in1: i64, in2: f64) -> i64 {
    div_time_by_f64(in1, in2)
}

/// .
/// Divide LTIME by LREAL
/// A zero divisor saturates at the TIME range, a NaN divisor or result yields zero, overflow saturates
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C-unwind" fn DIV_LTIME__LREAL(in1: i64, in2: f64) -> i64 {
    div_time_by_f64(in1, in2)
}

fn div_time_by_f64(in1: i64, in2: f64) -> i64 {
    let negative = in1.is_negative() ^ in2.is_sign_negative();
    let magnitude = std::time::Duration::from_nanos(in1.unsigned_abs());
    // pre-check in f64 so the Duration math below can never panic on overflow;
    // a NaN result here covers both a NaN divisor and zero divided by zero
    let approx_nanos = magnitude.as_secs_f64() / in2.abs() * 1e9;
    if approx_nanos.is_nan() {
        return 0;
    }
    let res: i64 = if approx_nanos >= i64::MAX as f64 {
        i64::MAX
    } else {
        magnitude.div_f64(in2.abs()).as_nanos().try_into().unwrap_or(i64::MAX)
    };
    if negative {
        -res
    } else {
        res
    }
}
