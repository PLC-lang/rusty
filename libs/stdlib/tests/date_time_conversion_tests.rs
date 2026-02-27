use common::{compile_and_run, get_includes};

// Import common functionality into the integration tests
mod common;

#[allow(dead_code)]
#[repr(C)]
struct MainType {
    a: [usize; 1000],
}

impl Default for MainType {
    fn default() -> Self {
        MainType { a: [0; 1000] }
    }
}

#[test]
fn ltime_to_time_conversion() {
    let src = "
    FUNCTION main : TIME
        main := LTIME_TO_TIME(LTIME#10s);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(res, 10000000000);
}

#[test]
fn time_to_ltime_conversion() {
    let src = "
    FUNCTION main : LTIME
        main := TIME_TO_LTIME(TIME#10s);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(res, 10000000000);
}

#[test]
fn ldt_to_dt_conversion() {
    let src = "
    FUNCTION main : DT
        main := LDT_TO_DT(LDT#2021-04-20-22:33:14);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(2021, 4, 20)
            .unwrap()
            .and_hms_opt(22, 33, 14)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn ldt_to_date_conversion() {
    let src = "
    FUNCTION main : DATE
        main := LDT_TO_DATE(LDT#2000-01-01-20:15:11);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn ldt_to_ltod_conversion() {
    let src = "
    FUNCTION main : LTOD
        main := LDT_TO_LTOD(LDT#2000-01-01-15:36:30.123456);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_nano_opt(15, 36, 30, 123456000)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn ldt_to_tod_conversion() {
    let src = "
    FUNCTION main : TOD
        main := LDT_TO_TOD(LDT#2120-02-12-20:15:11.543);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_milli_opt(20, 15, 11, 543)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn dt_to_ldt_conversion() {
    let src = "
    FUNCTION main : LDT
        main := DT_TO_LDT(DT#2021-04-20-22:33:14);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(2021, 4, 20)
            .unwrap()
            .and_hms_opt(22, 33, 14)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn dt_to_date_conversion() {
    let src = "
    FUNCTION main : DATE
        main := DT_TO_DATE(DT#2000-01-01-20:15:11);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(2000, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn dt_to_ltod_conversion() {
    let src = "
    FUNCTION main : LTOD
        main := DT_TO_LTOD(DT#2000-01-01-15:36:30.123);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_milli_opt(15, 36, 30, 123)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn dt_to_tod_conversion() {
    let src = "
    FUNCTION main : TOD
        main := DT_TO_TOD(DT#2120-02-12-20:15:11.543);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_milli_opt(20, 15, 11, 543)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn ltod_to_tod_conversion() {
    let src = "
    FUNCTION main : TOD
        main := LTOD_TO_TOD(LTOD#10:20:30);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_opt(10, 20, 30)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}

#[test]
fn tod_to_ltod_conversion() {
    let src = "
    FUNCTION main : LTOD
        main := TOD_TO_LTOD(TOD#10:20:30);
    END_FUNCTION";
    let sources = vec![src.into()];
    let includes = get_includes(&["date_time_conversion.st"]);
    let mut maintype = MainType::default();
    let res: i64 = compile_and_run(sources, includes, &mut maintype);
    assert_eq!(
        res,
        chrono::NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .and_hms_opt(10, 20, 30)
            .unwrap()
            .and_utc()
            .timestamp_nanos_opt()
            .unwrap()
    );
}
