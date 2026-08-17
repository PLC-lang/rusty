const NANOS_PER_MILLISECOND: i64 = 1_000 * 1_000;
const NANOS_PER_SECOND: i64 = 1_000 * 1_000 * 1_000;
const SECONDS_PER_DAY: u32 = 60 * 60 * 24;
const NANOS_PER_DAY: i64 = NANOS_PER_SECOND * SECONDS_PER_DAY as i64;

/// .
/// Converts DT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_DATE(input: u32) -> u32 {
    (input / SECONDS_PER_DAY) * SECONDS_PER_DAY
}

/// .
/// Converts LDT to LDATE
/// The midnight of the first representable day lies below the LDATE range and
/// saturates to the range minimum
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDATE_AND_TIME_TO_LDATE(input: i64) -> i64 {
    input.div_euclid(NANOS_PER_DAY).checked_mul(NANOS_PER_DAY).unwrap_or(i64::MIN)
}

/// .
/// Converts DT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DATE_AND_TIME_TO_TIME_OF_DAY(input: u32) -> u32 {
    (input % SECONDS_PER_DAY) * 1_000
}

/// .
/// Converts LDT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDATE_AND_TIME_TO_LTIME_OF_DAY(input: i64) -> i64 {
    input.rem_euclid(NANOS_PER_DAY)
}

/// .
/// Converts LTIME to TIME
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LTIME_TO_TIME(input: i64) -> u32 {
    (input / NANOS_PER_MILLISECOND) as u32
}

/// .
/// Converts TIME to LTIME
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TIME_TO_LTIME(input: u32) -> i64 {
    (input as i64) * NANOS_PER_MILLISECOND
}

/// .
/// Converts LDT to DT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDT_TO_DT(input: i64) -> u32 {
    (input / NANOS_PER_SECOND) as u32
}

/// .
/// Converts LDT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDT_TO_DATE(input: i64) -> u32 {
    (LDATE_AND_TIME_TO_LDATE(input) / NANOS_PER_SECOND) as u32
}

/// .
/// Converts LDT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDT_TO_LTOD(input: i64) -> i64 {
    LDATE_AND_TIME_TO_LTIME_OF_DAY(input)
}

/// .
/// Converts LDT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LDT_TO_TOD(input: i64) -> u32 {
    (LDT_TO_LTOD(input) / NANOS_PER_MILLISECOND) as u32
}

/// .
/// Converts DT to LDT
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DT_TO_LDT(input: u32) -> i64 {
    (input as i64) * NANOS_PER_SECOND
}

/// .
/// Converts DT to DATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DT_TO_DATE(input: u32) -> u32 {
    (input / SECONDS_PER_DAY) * SECONDS_PER_DAY
}

/// .
/// Converts DT to LDATE
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DT_TO_LDATE(input: u32) -> i64 {
    (DT_TO_DATE(input) as i64) * NANOS_PER_SECOND
}

/// .
/// Converts DT to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DT_TO_LTOD(input: u32) -> i64 {
    LDT_TO_LTOD(DT_TO_LDT(input))
}

/// .
/// Converts DT to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn DT_TO_TOD(input: u32) -> u32 {
    (DT_TO_LTOD(input) / NANOS_PER_MILLISECOND) as u32
}

/// .
/// Converts LTOD to TOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn LTOD_TO_TOD(input: i64) -> u32 {
    (input / NANOS_PER_MILLISECOND) as u32
}

/// .
/// Converts TOD to LTOD
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn TOD_TO_LTOD(input: u32) -> i64 {
    (input as i64) * NANOS_PER_MILLISECOND
}
