#[no_mangle]
pub static PI_LREAL: f64 = std::f64::consts::PI;
#[no_mangle]
pub static PI_REAL: f32 = std::f32::consts::PI;
#[no_mangle]
pub static FRAC_PI_2_LREAL: f64 = std::f64::consts::FRAC_PI_2;
#[no_mangle]
pub static FRAC_PI_2_REAL: f32 = std::f32::consts::FRAC_PI_2;
#[no_mangle]
pub static FRAC_PI_4_LREAL: f64 = std::f64::consts::FRAC_PI_4;
#[no_mangle]
pub static FRAC_PI_4_REAL: f32 = std::f32::consts::FRAC_PI_4;
#[no_mangle]
pub static E_REAL: f32 = std::f32::consts::E;
#[no_mangle]
pub static E_LREAL: f64 = std::f64::consts::E;
#[no_mangle]
pub static INF_REAL: f32 = f32::INFINITY;
#[no_mangle]
pub static INF_LREAL: f64 = f64::INFINITY;
#[no_mangle]
pub static NAN_REAL: f32 = f32::NAN;
#[no_mangle]
pub static NAN_LREAL: f64 = f64::NAN;

/// .
/// Calculates the square root of the given (f32) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQRT__REAL(input: f32) -> f32 {
    f32::sqrt(input)
}

/// .
/// Calculates the square root of the given (f64) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SQRT__LREAL(input: f64) -> f64 {
    f64::sqrt(input)
}

/// .
/// Calculates the natural logarithm of the given (f32) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LN__REAL(input: f32) -> f32 {
    f32::ln(input)
}

/// .
/// Calculates the natural logarithm of the given (f64) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LN__LREAL(input: f64) -> f64 {
    f64::ln(input)
}

/// .
/// Calculates the base 10 logarithm of the given (f32) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LOG__REAL(input: f32) -> f32 {
    f32::log10(input)
}

/// .
/// Calculates the natural logarithm of the given (f64) value
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LOG__LREAL(input: f64) -> f64 {
    f64::log10(input)
}

/// .
/// The natural exponential function (e)
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXP__REAL(input: f32) -> f32 {
    f32::exp(input)
}

/// .
/// The natural exponential function (e)
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXP__LREAL(input: f64) -> f64 {
    f64::exp(input)
}

///
/// .
/// Calculates the sine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SIN__REAL(input: f32) -> f32 {
    f32::sin(input)
}

/// .
/// Calculates the sine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn SIN__LREAL(input: f64) -> f64 {
    f64::sin(input)
}

/// .
/// Calculates the cosine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn COS__REAL(input: f32) -> f32 {
    f32::cos(input)
}

/// .
/// Calculates the cosine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn COS__LREAL(input: f64) -> f64 {
    f64::cos(input)
}

/// .
/// Calculates the tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TAN__REAL(input: f32) -> f32 {
    f32::tan(input)
}

/// .
/// Calculates the tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TAN__LREAL(input: f64) -> f64 {
    f64::tan(input)
}

/// .
/// Calculates the arc sine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ASIN__REAL(input: f32) -> f32 {
    f32::asin(input)
}

/// .
/// Calculates the arc sine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ASIN__LREAL(input: f64) -> f64 {
    f64::asin(input)
}

/// .
/// Calculates the arc cosine of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ACOS__REAL(input: f32) -> f32 {
    f32::acos(input)
}

/// .
/// Calculates the arc cosine of the given value in radiants
///
/// # Examples
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ACOS__LREAL(input: f64) -> f64 {
    f64::acos(input)
}

/// .
/// Calculates the arc tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN__REAL(input: f32) -> f32 {
    f32::atan(input)
}

/// .
/// Calculates the arc tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN__LREAL(input: f64) -> f64 {
    f64::atan(input)
}

/// .
/// Calculates the four quadrant arc tangent of the value with another value
///
/// # Examples
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN2__REAL(in1: f32, in2: f32) -> f32 {
    in1.atan2(in2)
}

/// .
/// Calculates the arc tangent of the given value in radiants
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn ATAN2__LREAL(in1: f64, in2: f64) -> f64 {
    in1.atan2(in2)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXPT__REAL__DINT(in1: f32, in2: i32) -> f32 {
    in1.powi(in2)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXPT__LREAL__DINT(in1: f64, in2: i32) -> f64 {
    in1.powi(in2)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXPT__REAL__REAL(in1: f32, in2: f32) -> f32 {
    in1.powf(in2)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXPT__REAL__LREAL(in1: f32, in2: f64) -> f32 {
    // casting from an f32 to an f64 will produce the closest possible f32
    // on overflow, infinity of the same sign as the input
    in1.powf(in2 as f32)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXPT__LREAL__REAL(in1: f64, in2: f32) -> f64 {
    // casting from an f32 to an f64 is perfect and lossless
    in1.powf(in2 as f64)
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn EXPT__LREAL__LREAL(in1: f64, in2: f64) -> f64 {
    in1.powf(in2)
}
