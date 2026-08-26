const NANOS_PER_SECOND: i64 = 1_000 * 1_000 * 1_000;
const SECONDS_PER_DAY: i64 = 60 * 60 * 24;
const NANOS_PER_DAY: i64 = NANOS_PER_SECOND * SECONDS_PER_DAY;

/// .
/// Converts DT/LDT to DATE
/// The midnight of the first representable day lies below the DATE range and
/// saturates to the range minimum
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_DATE(input: i64) -> i64 {
    input.div_euclid(NANOS_PER_DAY).checked_mul(NANOS_PER_DAY).unwrap_or(i64::MIN)
}

/// .
/// Converts DT/LDT to TOD/LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_TIME_OF_DAY(input: i64) -> i64 {
    input.rem_euclid(NANOS_PER_DAY)
}
