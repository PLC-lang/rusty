use chrono::NaiveDate;

/// calculates the seconds in the given days, hours minutes and seconds
pub fn calculate_dhm_time_seconds(day: f64, hour: f64, min: f64, sec: f64) -> f64 {
    let hours = day * 24_f64 + hour;
    let mins = hours * 60_f64 + min;
    mins * 60_f64 + sec
}

/// calculates the nanos in the given seconds, millis, micros and nano/**
pub fn calculate_time_nano(negative: bool, sec: f64, milli: f64, micro: f64, nano: u32) -> i64 {
    let millis = sec * 1000_f64 + milli;
    let micro = millis * 1000_f64 + micro;
    let nano = micro * 1000_f64 + nano as f64;
    //go to full micro
    let nanos = nano.round() as i64;

    if negative {
        -nanos
    } else {
        nanos
    }
}

/// calculates the nanoseconds since 1970-01-01-00:00:00 for the given
/// point in time
pub fn calculate_date_time(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
    nano: u32,
) -> Result<i64, String> {
    if let Some(date_time) = NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|date| date.and_hms_nano_opt(hour, min, sec, nano))
    {
        return Ok(date_time.timestamp_nanos());
    }
    Err(format!(
        "Invalid Date {}-{}-{}-{}:{}:{}.{}",
        year, month, day, hour, min, sec, nano
    ))
}
