use std::path::PathBuf;

use plc::codegen::{CodegenContext, GeneratedModule};
use plc_driver::runner::compile;
use plc_source::{Compilable, SourceCode, SourceContainer};

#[allow(unused_macros)] //This is actually used in subtests
macro_rules! add_std {
    ($src:expr, $($name:expr),* ) => {
        {
            let mut res = vec![$src.into()];
            $(
               res.push(crate::common::get_st_file($name));
            )*
            res
        }
    };
}

#[allow(unused_imports)] //This is actually used in subtests
pub(crate) use add_std;

#[macro_export]
macro_rules! assert_almost_eq {
    ($left:expr, $right:expr, $prec:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                let diff = (left_val - right_val).abs();

                if diff > $prec {
                    panic!(
                        "assertion failed: `(left == right)`\n      left: `{:?}`,\n     right: `{:?}`",
                        &*left_val, &*right_val
                    )
                }
            }
        }
    }};
}

/// Gets a file from the ST defined standard functions
#[allow(dead_code)]
pub fn get_st_file(name: &str) -> SourceCode {
    let mut data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_path.push("iec61131-st");
    data_path.push(name);

    assert!(data_path.exists());

    data_path.load_source(None).expect("Could not load source")
}

/// Compiles code with all native functions included
/// Should be updated for each native function we add
pub fn compile_with_native<T: Compilable>(context: &CodegenContext, source: T) -> GeneratedModule {
    let functions = vec![
        ("ROUND__REAL", iec61131std::numerical_functions::ROUND__REAL as usize),
        ("ROUND__LREAL", iec61131std::numerical_functions::ROUND__LREAL as usize),
        ("SQRT__REAL", iec61131std::arithmetic_functions::SQRT__REAL as usize),
        ("SQRT__LREAL", iec61131std::arithmetic_functions::SQRT__LREAL as usize),
        ("LN__REAL", iec61131std::arithmetic_functions::LN__REAL as usize),
        ("LN__LREAL", iec61131std::arithmetic_functions::LN__LREAL as usize),
        ("LOG__REAL", iec61131std::arithmetic_functions::LOG__REAL as usize),
        ("LOG__LREAL", iec61131std::arithmetic_functions::LOG__LREAL as usize),
        ("EXP__REAL", iec61131std::arithmetic_functions::EXP__REAL as usize),
        ("EXP__LREAL", iec61131std::arithmetic_functions::EXP__LREAL as usize),
        ("SIN__REAL", iec61131std::arithmetic_functions::SIN__REAL as usize),
        ("SIN__LREAL", iec61131std::arithmetic_functions::SIN__LREAL as usize),
        ("COS__REAL", iec61131std::arithmetic_functions::COS__REAL as usize),
        ("COS__LREAL", iec61131std::arithmetic_functions::COS__LREAL as usize),
        ("TAN__REAL", iec61131std::arithmetic_functions::TAN__REAL as usize),
        ("TAN__LREAL", iec61131std::arithmetic_functions::TAN__LREAL as usize),
        ("ASIN__REAL", iec61131std::arithmetic_functions::ASIN__REAL as usize),
        ("ASIN__LREAL", iec61131std::arithmetic_functions::ASIN__LREAL as usize),
        ("ACOS__REAL", iec61131std::arithmetic_functions::ACOS__REAL as usize),
        ("ACOS__LREAL", iec61131std::arithmetic_functions::ACOS__LREAL as usize),
        ("ATAN__REAL", iec61131std::arithmetic_functions::ATAN__REAL as usize),
        ("ATAN__LREAL", iec61131std::arithmetic_functions::ATAN__LREAL as usize),
        ("ATAN2__REAL", iec61131std::arithmetic_functions::ATAN2__REAL as usize),
        ("ATAN2__LREAL", iec61131std::arithmetic_functions::ATAN2__LREAL as usize),
        ("LWORD_TO_LREAL", iec61131std::bit_num_conversion::LWORD_TO_LREAL as usize),
        ("DWORD_TO_REAL", iec61131std::bit_num_conversion::DWORD_TO_REAL as usize),
        ("LREAL_TO_LWORD", iec61131std::bit_num_conversion::LREAL_TO_LWORD as usize),
        ("REAL_TO_DWORD", iec61131std::bit_num_conversion::REAL_TO_DWORD as usize),
        ("WSTRING_TO_STRING_EXT", iec61131std::string_conversion::WSTRING_TO_STRING_EXT as usize),
        ("STRING_TO_WSTRING_EXT", iec61131std::string_conversion::STRING_TO_WSTRING_EXT as usize),
        ("WCHAR_TO_CHAR", iec61131std::string_conversion::WCHAR_TO_CHAR as usize),
        ("CHAR_TO_WCHAR", iec61131std::string_conversion::CHAR_TO_WCHAR as usize),
        ("SHL__BYTE", iec61131std::bit_shift_functions::SHL__BYTE as usize),
        ("SHL__WORD", iec61131std::bit_shift_functions::SHL__WORD as usize),
        ("SHL__DWORD", iec61131std::bit_shift_functions::SHL__DWORD as usize),
        ("SHL__LWORD", iec61131std::bit_shift_functions::SHL__LWORD as usize),
        ("SHR__BYTE", iec61131std::bit_shift_functions::SHR__BYTE as usize),
        ("SHR__WORD", iec61131std::bit_shift_functions::SHR__WORD as usize),
        ("SHR__DWORD", iec61131std::bit_shift_functions::SHR__DWORD as usize),
        ("SHR__LWORD", iec61131std::bit_shift_functions::SHR__LWORD as usize),
        ("ROL__BYTE", iec61131std::bit_shift_functions::ROL__BYTE as usize),
        ("ROL__WORD", iec61131std::bit_shift_functions::ROL__WORD as usize),
        ("ROL__DWORD", iec61131std::bit_shift_functions::ROL__DWORD as usize),
        ("ROL__LWORD", iec61131std::bit_shift_functions::ROL__LWORD as usize),
        ("ROR__BYTE", iec61131std::bit_shift_functions::ROR__BYTE as usize),
        ("ROR__WORD", iec61131std::bit_shift_functions::ROR__WORD as usize),
        ("ROR__DWORD", iec61131std::bit_shift_functions::ROR__DWORD as usize),
        ("ROR__LWORD", iec61131std::bit_shift_functions::ROR__LWORD as usize),
        ("DATE_AND_TIME_TO_DATE", iec61131std::date_time_conversion::DATE_AND_TIME_TO_DATE as usize),
        (
            "DATE_AND_TIME_TO_TIME_OF_DAY",
            iec61131std::date_time_conversion::DATE_AND_TIME_TO_TIME_OF_DAY as usize,
        ),
        ("CONCAT_DATE_TOD", iec61131std::date_time_extra_functions::CONCAT_DATE_TOD as usize),
        ("CONCAT_DATE__INT", iec61131std::date_time_extra_functions::CONCAT_DATE__INT as usize),
        ("CONCAT_DATE__UINT", iec61131std::date_time_extra_functions::CONCAT_DATE__UINT as usize),
        ("CONCAT_DATE__DINT", iec61131std::date_time_extra_functions::CONCAT_DATE__DINT as usize),
        ("CONCAT_TOD__SINT", iec61131std::date_time_extra_functions::CONCAT_TOD__SINT as usize),
        ("CONCAT_TOD__USINT", iec61131std::date_time_extra_functions::CONCAT_TOD__USINT as usize),
        ("CONCAT_TOD__INT", iec61131std::date_time_extra_functions::CONCAT_TOD__INT as usize),
        ("CONCAT_TOD__UINT", iec61131std::date_time_extra_functions::CONCAT_TOD__UINT as usize),
        ("CONCAT_TOD__DINT", iec61131std::date_time_extra_functions::CONCAT_TOD__DINT as usize),
        ("CONCAT_TOD__UDINT", iec61131std::date_time_extra_functions::CONCAT_TOD__UDINT as usize),
        ("CONCAT_TOD__LINT", iec61131std::date_time_extra_functions::CONCAT_TOD__LINT as usize),
        ("CONCAT_TOD__ULINT", iec61131std::date_time_extra_functions::CONCAT_TOD__ULINT as usize),
        ("CONCAT_DATE__UDINT", iec61131std::date_time_extra_functions::CONCAT_DATE__UDINT as usize),
        ("CONCAT_DATE__LINT", iec61131std::date_time_extra_functions::CONCAT_DATE__LINT as usize),
        ("CONCAT_DATE__ULINT", iec61131std::date_time_extra_functions::CONCAT_DATE__ULINT as usize),
        ("SPLIT_DATE__INT", iec61131std::date_time_extra_functions::SPLIT_DATE__INT as usize),
        ("SPLIT_DATE__UINT", iec61131std::date_time_extra_functions::SPLIT_DATE__UINT as usize),
        ("SPLIT_DATE__DINT", iec61131std::date_time_extra_functions::SPLIT_DATE__DINT as usize),
        ("SPLIT_DATE__UDINT", iec61131std::date_time_extra_functions::SPLIT_DATE__UDINT as usize),
        ("SPLIT_DATE__LINT", iec61131std::date_time_extra_functions::SPLIT_DATE__LINT as usize),
        ("SPLIT_DATE__ULINT", iec61131std::date_time_extra_functions::SPLIT_DATE__ULINT as usize),
        ("SPLIT_TOD__INT", iec61131std::date_time_extra_functions::SPLIT_TOD__INT as usize),
        ("SPLIT_TOD__UINT", iec61131std::date_time_extra_functions::SPLIT_TOD__UINT as usize),
        ("SPLIT_TOD__DINT", iec61131std::date_time_extra_functions::SPLIT_TOD__DINT as usize),
        ("SPLIT_TOD__UDINT", iec61131std::date_time_extra_functions::SPLIT_TOD__UDINT as usize),
        ("SPLIT_TOD__LINT", iec61131std::date_time_extra_functions::SPLIT_TOD__LINT as usize),
        ("SPLIT_TOD__ULINT", iec61131std::date_time_extra_functions::SPLIT_TOD__ULINT as usize),
        ("SPLIT_DT__INT", iec61131std::date_time_extra_functions::SPLIT_DT__INT as usize),
        ("SPLIT_DT__UINT", iec61131std::date_time_extra_functions::SPLIT_DT__UINT as usize),
        ("SPLIT_DT__DINT", iec61131std::date_time_extra_functions::SPLIT_DT__DINT as usize),
        ("SPLIT_DT__UDINT", iec61131std::date_time_extra_functions::SPLIT_DT__UDINT as usize),
        ("SPLIT_DT__LINT", iec61131std::date_time_extra_functions::SPLIT_DT__LINT as usize),
        ("SPLIT_DT__ULINT", iec61131std::date_time_extra_functions::SPLIT_DT__ULINT as usize),
        ("DAY_OF_WEEK", iec61131std::date_time_extra_functions::DAY_OF_WEEK as usize),
        ("ADD_TIME", iec61131std::date_time_numeric_functions::ADD_TIME as usize),
        ("ADD_TOD_TIME", iec61131std::date_time_numeric_functions::ADD_TOD_TIME as usize),
        ("ADD_DT_TIME", iec61131std::date_time_numeric_functions::ADD_DT_TIME as usize),
        ("SUB_TIME", iec61131std::date_time_numeric_functions::SUB_TIME as usize),
        ("SUB_TIME", iec61131std::date_time_numeric_functions::SUB_TIME as usize),
        ("SUB_DATE_DATE", iec61131std::date_time_numeric_functions::SUB_DATE_DATE as usize),
        ("SUB_TOD_TIME", iec61131std::date_time_numeric_functions::SUB_TOD_TIME as usize),
        ("SUB_TOD_TOD", iec61131std::date_time_numeric_functions::SUB_TOD_TOD as usize),
        ("SUB_DT_TIME", iec61131std::date_time_numeric_functions::SUB_DT_TIME as usize),
        ("SUB_DT_DT", iec61131std::date_time_numeric_functions::SUB_DT_DT as usize),
        ("MUL__TIME__SINT", iec61131std::date_time_numeric_functions::MUL__TIME__SINT as usize),
        ("MUL__TIME__INT", iec61131std::date_time_numeric_functions::MUL__TIME__INT as usize),
        ("MUL__TIME__DINT", iec61131std::date_time_numeric_functions::MUL__TIME__DINT as usize),
        ("MUL__TIME__LINT", iec61131std::date_time_numeric_functions::MUL__TIME__LINT as usize),
        ("MUL_TIME__SINT", iec61131std::date_time_numeric_functions::MUL_TIME__SINT as usize),
        ("MUL_TIME__INT", iec61131std::date_time_numeric_functions::MUL_TIME__INT as usize),
        ("MUL_TIME__DINT", iec61131std::date_time_numeric_functions::MUL_TIME__DINT as usize),
        ("MUL_TIME__LINT", iec61131std::date_time_numeric_functions::MUL_TIME__LINT as usize),
        ("MUL_LTIME__SINT", iec61131std::date_time_numeric_functions::MUL_LTIME__SINT as usize),
        ("MUL_LTIME__INT", iec61131std::date_time_numeric_functions::MUL_LTIME__INT as usize),
        ("MUL_LTIME__DINT", iec61131std::date_time_numeric_functions::MUL_LTIME__DINT as usize),
        ("MUL_LTIME__LINT", iec61131std::date_time_numeric_functions::MUL_LTIME__LINT as usize),
        ("MUL__TIME__USINT", iec61131std::date_time_numeric_functions::MUL__TIME__USINT as usize),
        ("MUL__TIME__UINT", iec61131std::date_time_numeric_functions::MUL__TIME__UINT as usize),
        ("MUL__TIME__UDINT", iec61131std::date_time_numeric_functions::MUL__TIME__UDINT as usize),
        ("MUL__TIME__ULINT", iec61131std::date_time_numeric_functions::MUL__TIME__ULINT as usize),
        ("MUL_TIME__USINT", iec61131std::date_time_numeric_functions::MUL_TIME__USINT as usize),
        ("MUL_TIME__UINT", iec61131std::date_time_numeric_functions::MUL_TIME__UINT as usize),
        ("MUL_TIME__UDINT", iec61131std::date_time_numeric_functions::MUL_TIME__UDINT as usize),
        ("MUL_TIME__ULINT", iec61131std::date_time_numeric_functions::MUL_TIME__ULINT as usize),
        ("MUL_LTIME__USINT", iec61131std::date_time_numeric_functions::MUL_LTIME__USINT as usize),
        ("MUL_LTIME__UINT", iec61131std::date_time_numeric_functions::MUL_LTIME__UINT as usize),
        ("MUL_LTIME__UDINT", iec61131std::date_time_numeric_functions::MUL_LTIME__UDINT as usize),
        ("MUL_LTIME__ULINT", iec61131std::date_time_numeric_functions::MUL_LTIME__ULINT as usize),
        ("DIV__TIME__USINT", iec61131std::date_time_numeric_functions::DIV__TIME__USINT as usize),
        ("DIV__TIME__UINT", iec61131std::date_time_numeric_functions::DIV__TIME__UINT as usize),
        ("DIV__TIME__UDINT", iec61131std::date_time_numeric_functions::DIV__TIME__UDINT as usize),
        ("DIV__TIME__ULINT", iec61131std::date_time_numeric_functions::DIV__TIME__ULINT as usize),
        ("DIV_TIME__USINT", iec61131std::date_time_numeric_functions::DIV_TIME__USINT as usize),
        ("DIV_TIME__UINT", iec61131std::date_time_numeric_functions::DIV_TIME__UINT as usize),
        ("DIV_TIME__UDINT", iec61131std::date_time_numeric_functions::DIV_TIME__UDINT as usize),
        ("DIV_TIME__ULINT", iec61131std::date_time_numeric_functions::DIV_TIME__ULINT as usize),
        ("DIV_LTIME__USINT", iec61131std::date_time_numeric_functions::DIV_LTIME__USINT as usize),
        ("DIV_LTIME__UINT", iec61131std::date_time_numeric_functions::DIV_LTIME__UINT as usize),
        ("DIV_LTIME__UDINT", iec61131std::date_time_numeric_functions::DIV_LTIME__UDINT as usize),
        ("DIV_LTIME__ULINT", iec61131std::date_time_numeric_functions::DIV_LTIME__ULINT as usize),
        ("DIV__TIME__SINT", iec61131std::date_time_numeric_functions::DIV__TIME__SINT as usize),
        ("DIV__TIME__INT", iec61131std::date_time_numeric_functions::DIV__TIME__INT as usize),
        ("DIV__TIME__DINT", iec61131std::date_time_numeric_functions::DIV__TIME__DINT as usize),
        ("DIV__TIME__LINT", iec61131std::date_time_numeric_functions::DIV__TIME__LINT as usize),
        ("DIV_TIME__SINT", iec61131std::date_time_numeric_functions::DIV_TIME__SINT as usize),
        ("DIV_TIME__INT", iec61131std::date_time_numeric_functions::DIV_TIME__INT as usize),
        ("DIV_TIME__DINT", iec61131std::date_time_numeric_functions::DIV_TIME__DINT as usize),
        ("DIV_TIME__LINT", iec61131std::date_time_numeric_functions::DIV_TIME__LINT as usize),
        ("DIV_LTIME__SINT", iec61131std::date_time_numeric_functions::DIV_LTIME__SINT as usize),
        ("DIV_LTIME__INT", iec61131std::date_time_numeric_functions::DIV_LTIME__INT as usize),
        ("DIV_LTIME__DINT", iec61131std::date_time_numeric_functions::DIV_LTIME__DINT as usize),
        ("DIV_LTIME__LINT", iec61131std::date_time_numeric_functions::DIV_LTIME__LINT as usize),
        ("MUL__TIME__REAL", iec61131std::date_time_numeric_functions::MUL__TIME__REAL as usize),
        ("MUL_TIME__REAL", iec61131std::date_time_numeric_functions::MUL_TIME__REAL as usize),
        ("MUL_LTIME__REAL", iec61131std::date_time_numeric_functions::MUL_LTIME__REAL as usize),
        ("MUL__TIME__LREAL", iec61131std::date_time_numeric_functions::MUL__TIME__LREAL as usize),
        ("MUL_TIME__LREAL", iec61131std::date_time_numeric_functions::MUL_TIME__LREAL as usize),
        ("MUL_LTIME__LREAL", iec61131std::date_time_numeric_functions::MUL_LTIME__LREAL as usize),
        ("DIV__TIME__REAL", iec61131std::date_time_numeric_functions::DIV__TIME__REAL as usize),
        ("DIV_TIME__REAL", iec61131std::date_time_numeric_functions::DIV_TIME__REAL as usize),
        ("DIV_LTIME__REAL", iec61131std::date_time_numeric_functions::DIV_LTIME__REAL as usize),
        ("DIV__TIME__LREAL", iec61131std::date_time_numeric_functions::DIV__TIME__LREAL as usize),
        ("DIV_TIME__LREAL", iec61131std::date_time_numeric_functions::DIV_TIME__LREAL as usize),
        ("DIV_LTIME__LREAL", iec61131std::date_time_numeric_functions::DIV_LTIME__LREAL as usize),
        ("IS_VALID__REAL", iec61131std::validation_functions::IS_VALID__REAL as usize),
        ("IS_VALID__LREAL", iec61131std::validation_functions::IS_VALID__LREAL as usize),
        ("IS_VALID_BCD__BYTE", iec61131std::validation_functions::IS_VALID_BCD__BYTE as usize),
        ("IS_VALID_BCD__WORD", iec61131std::validation_functions::IS_VALID_BCD__WORD as usize),
        ("IS_VALID_BCD__DWORD", iec61131std::validation_functions::IS_VALID_BCD__DWORD as usize),
        ("IS_VALID_BCD__LWORD", iec61131std::validation_functions::IS_VALID_BCD__LWORD as usize),
        ("TP", iec61131std::timers::TP as usize),
        ("TP_TIME", iec61131std::timers::TP_TIME as usize),
        ("TP_LTIME", iec61131std::timers::TP_LTIME as usize),
        ("TON", iec61131std::timers::TON as usize),
        ("TON_TIME", iec61131std::timers::TON_TIME as usize),
        ("TON_LTIME", iec61131std::timers::TON_LTIME as usize),
        ("TOF", iec61131std::timers::TOF as usize),
        ("TOF_TIME", iec61131std::timers::TOF_TIME as usize),
        ("TOF_LTIME", iec61131std::timers::TOF_LTIME as usize),
        ("SR", iec61131std::bistable_functionblocks::SR as usize),
        ("RS", iec61131std::bistable_functionblocks::RS as usize),
        ("R_TRIG", iec61131std::flanks::R_TRIG as usize),
        ("F_TRIG", iec61131std::flanks::F_TRIG as usize),
        ("MIN__BOOL", iec61131std::types::MIN__BOOL as usize),
        ("MIN__SINT", iec61131std::types::MIN__SINT as usize),
        ("MIN__USINT", iec61131std::types::MIN__USINT as usize),
        ("MIN__BYTE", iec61131std::types::MIN__BYTE as usize),
        ("MIN__CHAR", iec61131std::types::MIN__CHAR as usize),
        ("MIN__INT", iec61131std::types::MIN__INT as usize),
        ("MIN__UINT", iec61131std::types::MIN__UINT as usize),
        ("MIN__WORD", iec61131std::types::MIN__WORD as usize),
        ("MIN__WCHAR", iec61131std::types::MIN__WCHAR as usize),
        ("MIN__DINT", iec61131std::types::MIN__DINT as usize),
        ("MIN__UDINT", iec61131std::types::MIN__UDINT as usize),
        ("MIN__DWORD", iec61131std::types::MIN__DWORD as usize),
        ("MIN__LINT", iec61131std::types::MIN__LINT as usize),
        ("MIN__ULINT", iec61131std::types::MIN__ULINT as usize),
        ("MIN__LWORD", iec61131std::types::MIN__LWORD as usize),
        ("MIN__REAL", iec61131std::types::MIN__REAL as usize),
        ("MIN__LREAL", iec61131std::types::MIN__LREAL as usize),
        ("MIN__DATE", iec61131std::types::MIN__DATE as usize),
        ("MIN__DATE_AND_TIME", iec61131std::types::MIN__DATE_AND_TIME as usize),
        ("MIN__TIME", iec61131std::types::MIN__TIME as usize),
        ("MIN__TIME_OF_DAY", iec61131std::types::MIN__TIME_OF_DAY as usize),
        ("MAX__BOOL", iec61131std::types::MAX__BOOL as usize),
        ("MAX__SINT", iec61131std::types::MAX__SINT as usize),
        ("MAX__USINT", iec61131std::types::MAX__USINT as usize),
        ("MAX__BYTE", iec61131std::types::MAX__BYTE as usize),
        ("MAX__CHAR", iec61131std::types::MAX__CHAR as usize),
        ("MAX__INT", iec61131std::types::MAX__INT as usize),
        ("MAX__UINT", iec61131std::types::MAX__UINT as usize),
        ("MAX__WORD", iec61131std::types::MAX__WORD as usize),
        ("MAX__WCHAR", iec61131std::types::MAX__WCHAR as usize),
        ("MAX__DINT", iec61131std::types::MAX__DINT as usize),
        ("MAX__UDINT", iec61131std::types::MAX__UDINT as usize),
        ("MAX__DWORD", iec61131std::types::MAX__DWORD as usize),
        ("MAX__LINT", iec61131std::types::MAX__LINT as usize),
        ("MAX__ULINT", iec61131std::types::MAX__ULINT as usize),
        ("MAX__LWORD", iec61131std::types::MAX__LWORD as usize),
        ("MAX__REAL", iec61131std::types::MAX__REAL as usize),
        ("MAX__LREAL", iec61131std::types::MAX__LREAL as usize),
        ("MAX__DATE", iec61131std::types::MAX__DATE as usize),
        ("MAX__DATE_AND_TIME", iec61131std::types::MAX__DATE_AND_TIME as usize),
        ("MAX__TIME", iec61131std::types::MAX__TIME as usize),
        ("MAX__TIME_OF_DAY", iec61131std::types::MAX__TIME_OF_DAY as usize),
        ("LIMIT__BOOL", iec61131std::types::LIMIT__BOOL as usize),
        ("LIMIT__SINT", iec61131std::types::LIMIT__SINT as usize),
        ("LIMIT__USINT", iec61131std::types::LIMIT__USINT as usize),
        ("LIMIT__BYTE", iec61131std::types::LIMIT__BYTE as usize),
        ("LIMIT__CHAR", iec61131std::types::LIMIT__CHAR as usize),
        ("LIMIT__INT", iec61131std::types::LIMIT__INT as usize),
        ("LIMIT__UINT", iec61131std::types::LIMIT__UINT as usize),
        ("LIMIT__WORD", iec61131std::types::LIMIT__WORD as usize),
        ("LIMIT__WCHAR", iec61131std::types::LIMIT__WCHAR as usize),
        ("LIMIT__DINT", iec61131std::types::LIMIT__DINT as usize),
        ("LIMIT__UDINT", iec61131std::types::LIMIT__UDINT as usize),
        ("LIMIT__DWORD", iec61131std::types::LIMIT__DWORD as usize),
        ("LIMIT__LINT", iec61131std::types::LIMIT__LINT as usize),
        ("LIMIT__ULINT", iec61131std::types::LIMIT__ULINT as usize),
        ("LIMIT__LWORD", iec61131std::types::LIMIT__LWORD as usize),
        ("LIMIT__REAL", iec61131std::types::LIMIT__REAL as usize),
        ("LIMIT__LREAL", iec61131std::types::LIMIT__LREAL as usize),
        ("LIMIT__DATE", iec61131std::types::LIMIT__DATE as usize),
        ("LIMIT__DATE_AND_TIME", iec61131std::types::LIMIT__DATE_AND_TIME as usize),
        ("LIMIT__TIME", iec61131std::types::LIMIT__TIME as usize),
        ("LIMIT__TIME_OF_DAY", iec61131std::types::LIMIT__TIME_OF_DAY as usize),
        ("CTU", iec61131std::counters::CTU as usize),
        ("CTU_INT", iec61131std::counters::CTU_INT as usize),
        ("CTU_DINT", iec61131std::counters::CTU_DINT as usize),
        ("CTU_UDINT", iec61131std::counters::CTU_UDINT as usize),
        ("CTU_LINT", iec61131std::counters::CTU_LINT as usize),
        ("CTU_ULINT", iec61131std::counters::CTU_ULINT as usize),
        ("CTD", iec61131std::counters::CTD as usize),
        ("CTD_INT", iec61131std::counters::CTD_INT as usize),
        ("CTD_DINT", iec61131std::counters::CTD_DINT as usize),
        ("CTD_UDINT", iec61131std::counters::CTD_UDINT as usize),
        ("CTD_LINT", iec61131std::counters::CTD_LINT as usize),
        ("CTD_ULINT", iec61131std::counters::CTD_ULINT as usize),
        ("CTUD", iec61131std::counters::CTUD as usize),
        ("CTUD_INT", iec61131std::counters::CTUD_INT as usize),
        ("CTUD_DINT", iec61131std::counters::CTUD_DINT as usize),
        ("CTUD_UDINT", iec61131std::counters::CTUD_UDINT as usize),
        ("CTUD_LINT", iec61131std::counters::CTUD_LINT as usize),
        ("CTUD_ULINT", iec61131std::counters::CTUD_ULINT as usize),
        ("LEN__STRING", iec61131std::string_functions::LEN__STRING as usize),
        ("LEN__WSTRING", iec61131std::string_functions::LEN__WSTRING as usize),
        ("FIND__STRING", iec61131std::string_functions::FIND__STRING as usize),
        ("FIND__WSTRING", iec61131std::string_functions::FIND__WSTRING as usize),
        ("LEFT_EXT__STRING", iec61131std::string_functions::LEFT_EXT__STRING as usize),
        ("LEFT_EXT__WSTRING", iec61131std::string_functions::LEFT_EXT__WSTRING as usize),
        ("RIGHT_EXT__STRING", iec61131std::string_functions::RIGHT_EXT__STRING as usize),
        ("RIGHT_EXT__WSTRING", iec61131std::string_functions::RIGHT_EXT__WSTRING as usize),
        ("MID_EXT__STRING", iec61131std::string_functions::MID_EXT__STRING as usize),
        ("MID_EXT__WSTRING", iec61131std::string_functions::MID_EXT__WSTRING as usize),
        ("INSERT_EXT__STRING", iec61131std::string_functions::INSERT_EXT__STRING as usize),
        ("INSERT_EXT__WSTRING", iec61131std::string_functions::INSERT_EXT__WSTRING as usize),
        ("DELETE_EXT__STRING", iec61131std::string_functions::DELETE_EXT__STRING as usize),
        ("DELETE_EXT__WSTRING", iec61131std::string_functions::DELETE_EXT__WSTRING as usize),
        ("REPLACE_EXT__STRING", iec61131std::string_functions::REPLACE_EXT__STRING as usize),
        ("REPLACE_EXT__WSTRING", iec61131std::string_functions::REPLACE_EXT__WSTRING as usize),
        ("CONCAT__STRING", iec61131std::string_functions::CONCAT__STRING as usize),
        ("CONCAT_EXT__STRING", iec61131std::string_functions::CONCAT_EXT__STRING as usize),
        ("CONCAT__WSTRING", iec61131std::string_functions::CONCAT__WSTRING as usize),
        ("CONCAT_EXT__WSTRING", iec61131std::string_functions::CONCAT_EXT__WSTRING as usize),
        ("GT__STRING", iec61131std::string_functions::GT__STRING as usize),
        ("GT__WSTRING", iec61131std::string_functions::GT__WSTRING as usize),
        ("GE__STRING", iec61131std::string_functions::GE__STRING as usize),
        ("GE__WSTRING", iec61131std::string_functions::GE__WSTRING as usize),
        ("EQ__STRING", iec61131std::string_functions::EQ__STRING as usize),
        ("EQ__WSTRING", iec61131std::string_functions::EQ__WSTRING as usize),
        ("LE__STRING", iec61131std::string_functions::LE__STRING as usize),
        ("LE__WSTRING", iec61131std::string_functions::LE__WSTRING as usize),
        ("LT__STRING", iec61131std::string_functions::LT__STRING as usize),
        ("LT__WSTRING", iec61131std::string_functions::LT__WSTRING as usize),
        ("NE__STRING", iec61131std::string_functions::NE__STRING as usize),
        ("NE__WSTRING", iec61131std::string_functions::NE__WSTRING as usize),
        ("EXPT__REAL__DINT", iec61131std::arithmetic_functions::EXPT__REAL__DINT as usize),
        ("EXPT__REAL__REAL", iec61131std::arithmetic_functions::EXPT__REAL__REAL as usize),
        ("EXPT__REAL__LREAL", iec61131std::arithmetic_functions::EXPT__REAL__LREAL as usize),
        ("EXPT__LREAL__DINT", iec61131std::arithmetic_functions::EXPT__LREAL__DINT as usize),
        ("EXPT__LREAL__REAL", iec61131std::arithmetic_functions::EXPT__LREAL__REAL as usize),
        ("EXPT__LREAL__LREAL", iec61131std::arithmetic_functions::EXPT__LREAL__LREAL as usize),
        ("TO_BIG_ENDIAN__INT", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__INT as usize),
        (
            "TO_LITTLE_ENDIAN__INT",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__INT as usize,
        ),
        ("FROM_BIG_ENDIAN__INT", iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__INT as usize),
        (
            "FROM_LITTLE_ENDIAN__INT",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__INT as usize,
        ),
        ("TO_BIG_ENDIAN__DINT", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__DINT as usize),
        (
            "TO_LITTLE_ENDIAN__DINT",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__DINT as usize,
        ),
        (
            "FROM_BIG_ENDIAN__DINT",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__DINT as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__DINT",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__DINT as usize,
        ),
        ("TO_BIG_ENDIAN__LINT", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__LINT as usize),
        (
            "TO_LITTLE_ENDIAN__LINT",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__LINT as usize,
        ),
        (
            "FROM_BIG_ENDIAN__LINT",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__LINT as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__LINT",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__LINT as usize,
        ),
        ("TO_BIG_ENDIAN__UINT", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__UINT as usize),
        (
            "TO_LITTLE_ENDIAN__UINT",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__UINT as usize,
        ),
        (
            "FROM_BIG_ENDIAN__UINT",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__UINT as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__UINT",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__UINT as usize,
        ),
        ("TO_BIG_ENDIAN__UDINT", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__UDINT as usize),
        (
            "TO_LITTLE_ENDIAN__UDINT",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__UDINT as usize,
        ),
        (
            "FROM_BIG_ENDIAN__UDINT",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__UDINT as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__UDINT",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__UDINT as usize,
        ),
        ("TO_BIG_ENDIAN__ULINT", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__ULINT as usize),
        (
            "TO_LITTLE_ENDIAN__ULINT",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__ULINT as usize,
        ),
        (
            "FROM_BIG_ENDIAN__ULINT",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__ULINT as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__ULINT",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__ULINT as usize,
        ),
        ("TO_BIG_ENDIAN__REAL", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__REAL as usize),
        (
            "TO_LITTLE_ENDIAN__REAL",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__REAL as usize,
        ),
        (
            "FROM_BIG_ENDIAN__REAL",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__REAL as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__REAL",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__REAL as usize,
        ),
        ("TO_BIG_ENDIAN__LREAL", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__LREAL as usize),
        (
            "TO_LITTLE_ENDIAN__LREAL",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__LREAL as usize,
        ),
        (
            "FROM_BIG_ENDIAN__LREAL",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__LREAL as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__LREAL",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__LREAL as usize,
        ),
        ("TO_BIG_ENDIAN__WORD", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__WORD as usize),
        (
            "TO_LITTLE_ENDIAN__WORD",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__WORD as usize,
        ),
        (
            "FROM_BIG_ENDIAN__WORD",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__WORD as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__WORD",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__WORD as usize,
        ),
        ("TO_BIG_ENDIAN__DWORD", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__DWORD as usize),
        (
            "TO_LITTLE_ENDIAN__DWORD",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__DWORD as usize,
        ),
        (
            "FROM_BIG_ENDIAN__DWORD",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__DWORD as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__DWORD",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__DWORD as usize,
        ),
        ("TO_BIG_ENDIAN__LWORD", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__LWORD as usize),
        (
            "TO_LITTLE_ENDIAN__LWORD",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__LWORD as usize,
        ),
        (
            "FROM_BIG_ENDIAN__LWORD",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__LWORD as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__LWORD",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__LWORD as usize,
        ),
        ("TO_BIG_ENDIAN__WCHAR", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__WCHAR as usize),
        (
            "TO_LITTLE_ENDIAN__WCHAR",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__WCHAR as usize,
        ),
        (
            "FROM_BIG_ENDIAN__WCHAR",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__WCHAR as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__WCHAR",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__WCHAR as usize,
        ),
        ("TO_BIG_ENDIAN__DATE", iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__DATE as usize),
        (
            "TO_LITTLE_ENDIAN__DATE",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__DATE as usize,
        ),
        (
            "FROM_BIG_ENDIAN__DATE",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__DATE as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__DATE",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__DATE as usize,
        ),
        (
            "TO_BIG_ENDIAN__TIME_OF_DAY",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__TIME_OF_DAY as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__TIME_OF_DAY",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__TIME_OF_DAY as usize,
        ),
        (
            "FROM_BIG_ENDIAN__TIME_OF_DAY",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__TIME_OF_DAY as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__TIME_OF_DAY",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__TIME_OF_DAY as usize,
        ),
        (
            "TO_BIG_ENDIAN__DATE_AND_TIME",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__DATE_AND_TIME as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__DATE_AND_TIME",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__DATE_AND_TIME as usize,
        ),
        (
            "FROM_BIG_ENDIAN__DATE_AND_TIME",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__DATE_AND_TIME as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__DATE_AND_TIME",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__DATE_AND_TIME as usize,
        ),
        ("BYTE_TO_STRING_EXT", iec61131std::extra_functions::BYTE_TO_STRING_EXT as usize),
        ("LWORD_TO_STRING_EXT", iec61131std::extra_functions::LWORD_TO_STRING_EXT as usize),
        ("LINT_TO_STRING_EXT", iec61131std::extra_functions::LINT_TO_STRING_EXT as usize),
        ("LREAL_TO_STRING_EXT", iec61131std::extra_functions::LREAL_TO_STRING_EXT as usize),
        ("STRING_TO_LINT", iec61131std::extra_functions::STRING_TO_LINT as usize),
        ("STRING_TO_DINT", iec61131std::extra_functions::STRING_TO_DINT as usize),
        ("STRING_TO_LREAL", iec61131std::extra_functions::STRING_TO_LREAL as usize),
        ("STRING_TO_REAL", iec61131std::extra_functions::STRING_TO_REAL as usize),
        ("TIME", iec61131std::extra_functions::TIME as usize),
        ("DT_TO_STRING_EXT", iec61131std::extra_functions::DT_TO_STRING_EXT as usize),
        ("TIME_TO_STRING_EXT", iec61131std::extra_functions::TIME_TO_STRING_EXT as usize),
        ("DATE_TO_STRING_EXT", iec61131std::extra_functions::DATE_TO_STRING_EXT as usize),
        ("TOD_TO_STRING_EXT", iec61131std::extra_functions::TOD_TO_STRING_EXT as usize),
    ];

    let variables = vec![
        ("PI_REAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::PI_REAL) as usize),
        ("PI_LREAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::PI_LREAL) as usize),
        ("FRAC_PI_2_REAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::FRAC_PI_2_REAL) as usize),
        ("FRAC_PI_2_LREAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::FRAC_PI_2_LREAL) as usize),
        ("FRAC_PI_4_REAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::FRAC_PI_4_REAL) as usize),
        ("FRAC_PI_4_LREAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::FRAC_PI_4_LREAL) as usize),
        ("E_REAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::E_REAL) as usize),
        ("E_LREAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::E_LREAL) as usize),
    ];

    let module = compile(context, source);
    log::debug!("{}", module.persist_to_string());

    for (fn_name, fn_addr) in functions {
        module.add_global_function_mapping(fn_name, fn_addr);
    }

    for (var_name, var_address) in variables {
        module.add_global_variable_mapping(var_name, var_address);
    }

    module
}

///
/// A Convenience method to compile and then run the given source
///
#[allow(dead_code)] //Not all test modules call the compile and run
pub fn compile_and_run<T, U, S: Compilable>(source: S, params: &mut T) -> U {
    let context = CodegenContext::create();
    let module = compile_with_native(&context, source);
    module.run::<T, U>("main", params)
}

///
/// A Convenience method to compile and then run the given source with no parameters
///
#[allow(dead_code)] //Not all test modules call the compile and run
pub fn compile_and_run_no_params<T, S: Compilable>(source: S) -> T {
    let context = CodegenContext::create();
    let module = compile_with_native(&context, source);
    module.run_no_param::<T>("main")
}
