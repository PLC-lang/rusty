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
pub fn compile_with_native<T: Compilable>(context: &CodegenContext, source: T) -> GeneratedModule<'_> {
    let functions = vec![
        ("ROUND__REAL", iec61131std::numerical_functions::ROUND__REAL as *const () as usize),
        ("ROUND__LREAL", iec61131std::numerical_functions::ROUND__LREAL as *const () as usize),
        ("SQRT__REAL", iec61131std::arithmetic_functions::SQRT__REAL as *const () as usize),
        ("SQRT__LREAL", iec61131std::arithmetic_functions::SQRT__LREAL as *const () as usize),
        ("LN__REAL", iec61131std::arithmetic_functions::LN__REAL as *const () as usize),
        ("LN__LREAL", iec61131std::arithmetic_functions::LN__LREAL as *const () as usize),
        ("LOG__REAL", iec61131std::arithmetic_functions::LOG__REAL as *const () as usize),
        ("LOG__LREAL", iec61131std::arithmetic_functions::LOG__LREAL as *const () as usize),
        ("EXP__REAL", iec61131std::arithmetic_functions::EXP__REAL as *const () as usize),
        ("EXP__LREAL", iec61131std::arithmetic_functions::EXP__LREAL as *const () as usize),
        ("SIN__REAL", iec61131std::arithmetic_functions::SIN__REAL as *const () as usize),
        ("SIN__LREAL", iec61131std::arithmetic_functions::SIN__LREAL as *const () as usize),
        ("COS__REAL", iec61131std::arithmetic_functions::COS__REAL as *const () as usize),
        ("COS__LREAL", iec61131std::arithmetic_functions::COS__LREAL as *const () as usize),
        ("TAN__REAL", iec61131std::arithmetic_functions::TAN__REAL as *const () as usize),
        ("TAN__LREAL", iec61131std::arithmetic_functions::TAN__LREAL as *const () as usize),
        ("ASIN__REAL", iec61131std::arithmetic_functions::ASIN__REAL as *const () as usize),
        ("ASIN__LREAL", iec61131std::arithmetic_functions::ASIN__LREAL as *const () as usize),
        ("ACOS__REAL", iec61131std::arithmetic_functions::ACOS__REAL as *const () as usize),
        ("ACOS__LREAL", iec61131std::arithmetic_functions::ACOS__LREAL as *const () as usize),
        ("ATAN__REAL", iec61131std::arithmetic_functions::ATAN__REAL as *const () as usize),
        ("ATAN__LREAL", iec61131std::arithmetic_functions::ATAN__LREAL as *const () as usize),
        ("ATAN2__REAL", iec61131std::arithmetic_functions::ATAN2__REAL as *const () as usize),
        ("ATAN2__LREAL", iec61131std::arithmetic_functions::ATAN2__LREAL as *const () as usize),
        ("LWORD_TO_LREAL", iec61131std::bit_num_conversion::LWORD_TO_LREAL as *const () as usize),
        ("DWORD_TO_REAL", iec61131std::bit_num_conversion::DWORD_TO_REAL as *const () as usize),
        ("LREAL_TO_LWORD", iec61131std::bit_num_conversion::LREAL_TO_LWORD as *const () as usize),
        ("REAL_TO_DWORD", iec61131std::bit_num_conversion::REAL_TO_DWORD as *const () as usize),
        (
            "WSTRING_TO_STRING_EXT",
            iec61131std::string_conversion::WSTRING_TO_STRING_EXT as *const () as usize,
        ),
        (
            "STRING_TO_WSTRING_EXT",
            iec61131std::string_conversion::STRING_TO_WSTRING_EXT as *const () as usize,
        ),
        ("STRING_TO_CHAR", iec61131std::string_conversion::STRING_TO_CHAR as *const () as usize),
        ("WSTRING_TO_WCHAR", iec61131std::string_conversion::WSTRING_TO_WCHAR as *const () as usize),
        ("CHAR_TO_STRING", iec61131std::string_conversion::CHAR_TO_STRING as *const () as usize),
        ("WCHAR_TO_WSTRING", iec61131std::string_conversion::WCHAR_TO_WSTRING as *const () as usize),
        ("WCHAR_TO_CHAR", iec61131std::string_conversion::WCHAR_TO_CHAR as *const () as usize),
        ("CHAR_TO_WCHAR", iec61131std::string_conversion::CHAR_TO_WCHAR as *const () as usize),
        ("ROL__BYTE", iec61131std::bit_shift_functions::ROL__BYTE as *const () as usize),
        ("ROL__WORD", iec61131std::bit_shift_functions::ROL__WORD as *const () as usize),
        ("ROL__DWORD", iec61131std::bit_shift_functions::ROL__DWORD as *const () as usize),
        ("ROL__LWORD", iec61131std::bit_shift_functions::ROL__LWORD as *const () as usize),
        ("ROR__BYTE", iec61131std::bit_shift_functions::ROR__BYTE as *const () as usize),
        ("ROR__WORD", iec61131std::bit_shift_functions::ROR__WORD as *const () as usize),
        ("ROR__DWORD", iec61131std::bit_shift_functions::ROR__DWORD as *const () as usize),
        ("ROR__LWORD", iec61131std::bit_shift_functions::ROR__LWORD as *const () as usize),
        (
            "DATE_AND_TIME_TO_DATE",
            iec61131std::date_time_conversion::DATE_AND_TIME_TO_DATE as *const () as usize,
        ),
        (
            "DATE_AND_TIME_TO_TIME_OF_DAY",
            iec61131std::date_time_conversion::DATE_AND_TIME_TO_TIME_OF_DAY as *const () as usize,
        ),
        ("CONCAT_DATE_TOD", iec61131std::date_time_extra_functions::CONCAT_DATE_TOD as *const () as usize),
        ("CONCAT_DATE__INT", iec61131std::date_time_extra_functions::CONCAT_DATE__INT as *const () as usize),
        (
            "CONCAT_DATE__UINT",
            iec61131std::date_time_extra_functions::CONCAT_DATE__UINT as *const () as usize,
        ),
        (
            "CONCAT_DATE__DINT",
            iec61131std::date_time_extra_functions::CONCAT_DATE__DINT as *const () as usize,
        ),
        ("CONCAT_TOD__SINT", iec61131std::date_time_extra_functions::CONCAT_TOD__SINT as *const () as usize),
        (
            "CONCAT_TOD__USINT",
            iec61131std::date_time_extra_functions::CONCAT_TOD__USINT as *const () as usize,
        ),
        ("CONCAT_TOD__INT", iec61131std::date_time_extra_functions::CONCAT_TOD__INT as *const () as usize),
        ("CONCAT_TOD__UINT", iec61131std::date_time_extra_functions::CONCAT_TOD__UINT as *const () as usize),
        ("CONCAT_TOD__DINT", iec61131std::date_time_extra_functions::CONCAT_TOD__DINT as *const () as usize),
        (
            "CONCAT_TOD__UDINT",
            iec61131std::date_time_extra_functions::CONCAT_TOD__UDINT as *const () as usize,
        ),
        ("CONCAT_TOD__LINT", iec61131std::date_time_extra_functions::CONCAT_TOD__LINT as *const () as usize),
        (
            "CONCAT_TOD__ULINT",
            iec61131std::date_time_extra_functions::CONCAT_TOD__ULINT as *const () as usize,
        ),
        (
            "CONCAT_DATE__UDINT",
            iec61131std::date_time_extra_functions::CONCAT_DATE__UDINT as *const () as usize,
        ),
        (
            "CONCAT_DATE__LINT",
            iec61131std::date_time_extra_functions::CONCAT_DATE__LINT as *const () as usize,
        ),
        (
            "CONCAT_DATE__ULINT",
            iec61131std::date_time_extra_functions::CONCAT_DATE__ULINT as *const () as usize,
        ),
        ("SPLIT_DATE__INT", iec61131std::date_time_extra_functions::SPLIT_DATE__INT as *const () as usize),
        ("SPLIT_DATE__UINT", iec61131std::date_time_extra_functions::SPLIT_DATE__UINT as *const () as usize),
        ("SPLIT_DATE__DINT", iec61131std::date_time_extra_functions::SPLIT_DATE__DINT as *const () as usize),
        (
            "SPLIT_DATE__UDINT",
            iec61131std::date_time_extra_functions::SPLIT_DATE__UDINT as *const () as usize,
        ),
        ("SPLIT_DATE__LINT", iec61131std::date_time_extra_functions::SPLIT_DATE__LINT as *const () as usize),
        (
            "SPLIT_DATE__ULINT",
            iec61131std::date_time_extra_functions::SPLIT_DATE__ULINT as *const () as usize,
        ),
        ("SPLIT_TOD__INT", iec61131std::date_time_extra_functions::SPLIT_TOD__INT as *const () as usize),
        ("SPLIT_TOD__UINT", iec61131std::date_time_extra_functions::SPLIT_TOD__UINT as *const () as usize),
        ("SPLIT_TOD__DINT", iec61131std::date_time_extra_functions::SPLIT_TOD__DINT as *const () as usize),
        ("SPLIT_TOD__UDINT", iec61131std::date_time_extra_functions::SPLIT_TOD__UDINT as *const () as usize),
        ("SPLIT_TOD__LINT", iec61131std::date_time_extra_functions::SPLIT_TOD__LINT as *const () as usize),
        ("SPLIT_TOD__ULINT", iec61131std::date_time_extra_functions::SPLIT_TOD__ULINT as *const () as usize),
        ("SPLIT_DT__INT", iec61131std::date_time_extra_functions::SPLIT_DT__INT as *const () as usize),
        ("SPLIT_DT__UINT", iec61131std::date_time_extra_functions::SPLIT_DT__UINT as *const () as usize),
        ("SPLIT_DT__DINT", iec61131std::date_time_extra_functions::SPLIT_DT__DINT as *const () as usize),
        ("SPLIT_DT__UDINT", iec61131std::date_time_extra_functions::SPLIT_DT__UDINT as *const () as usize),
        ("SPLIT_DT__LINT", iec61131std::date_time_extra_functions::SPLIT_DT__LINT as *const () as usize),
        ("SPLIT_DT__ULINT", iec61131std::date_time_extra_functions::SPLIT_DT__ULINT as *const () as usize),
        ("DAY_OF_WEEK", iec61131std::date_time_extra_functions::DAY_OF_WEEK as *const () as usize),
        ("ADD_TIME", iec61131std::date_time_numeric_functions::ADD_TIME as *const () as usize),
        ("ADD_TOD_TIME", iec61131std::date_time_numeric_functions::ADD_TOD_TIME as *const () as usize),
        ("ADD_DT_TIME", iec61131std::date_time_numeric_functions::ADD_DT_TIME as *const () as usize),
        ("SUB_TIME", iec61131std::date_time_numeric_functions::SUB_TIME as *const () as usize),
        ("SUB_TIME", iec61131std::date_time_numeric_functions::SUB_TIME as *const () as usize),
        ("SUB_DATE_DATE", iec61131std::date_time_numeric_functions::SUB_DATE_DATE as *const () as usize),
        ("SUB_TOD_TIME", iec61131std::date_time_numeric_functions::SUB_TOD_TIME as *const () as usize),
        ("SUB_TOD_TOD", iec61131std::date_time_numeric_functions::SUB_TOD_TOD as *const () as usize),
        ("SUB_DT_TIME", iec61131std::date_time_numeric_functions::SUB_DT_TIME as *const () as usize),
        ("SUB_DT_DT", iec61131std::date_time_numeric_functions::SUB_DT_DT as *const () as usize),
        ("MUL__TIME__SINT", iec61131std::date_time_numeric_functions::MUL__TIME__SINT as *const () as usize),
        ("MUL__TIME__INT", iec61131std::date_time_numeric_functions::MUL__TIME__INT as *const () as usize),
        ("MUL__TIME__DINT", iec61131std::date_time_numeric_functions::MUL__TIME__DINT as *const () as usize),
        ("MUL__TIME__LINT", iec61131std::date_time_numeric_functions::MUL__TIME__LINT as *const () as usize),
        ("MUL_TIME__SINT", iec61131std::date_time_numeric_functions::MUL_TIME__SINT as *const () as usize),
        ("MUL_TIME__INT", iec61131std::date_time_numeric_functions::MUL_TIME__INT as *const () as usize),
        ("MUL_TIME__DINT", iec61131std::date_time_numeric_functions::MUL_TIME__DINT as *const () as usize),
        ("MUL_TIME__LINT", iec61131std::date_time_numeric_functions::MUL_TIME__LINT as *const () as usize),
        ("MUL_LTIME__SINT", iec61131std::date_time_numeric_functions::MUL_LTIME__SINT as *const () as usize),
        ("MUL_LTIME__INT", iec61131std::date_time_numeric_functions::MUL_LTIME__INT as *const () as usize),
        ("MUL_LTIME__DINT", iec61131std::date_time_numeric_functions::MUL_LTIME__DINT as *const () as usize),
        ("MUL_LTIME__LINT", iec61131std::date_time_numeric_functions::MUL_LTIME__LINT as *const () as usize),
        (
            "MUL__TIME__USINT",
            iec61131std::date_time_numeric_functions::MUL__TIME__USINT as *const () as usize,
        ),
        ("MUL__TIME__UINT", iec61131std::date_time_numeric_functions::MUL__TIME__UINT as *const () as usize),
        (
            "MUL__TIME__UDINT",
            iec61131std::date_time_numeric_functions::MUL__TIME__UDINT as *const () as usize,
        ),
        (
            "MUL__TIME__ULINT",
            iec61131std::date_time_numeric_functions::MUL__TIME__ULINT as *const () as usize,
        ),
        ("MUL_TIME__USINT", iec61131std::date_time_numeric_functions::MUL_TIME__USINT as *const () as usize),
        ("MUL_TIME__UINT", iec61131std::date_time_numeric_functions::MUL_TIME__UINT as *const () as usize),
        ("MUL_TIME__UDINT", iec61131std::date_time_numeric_functions::MUL_TIME__UDINT as *const () as usize),
        ("MUL_TIME__ULINT", iec61131std::date_time_numeric_functions::MUL_TIME__ULINT as *const () as usize),
        (
            "MUL_LTIME__USINT",
            iec61131std::date_time_numeric_functions::MUL_LTIME__USINT as *const () as usize,
        ),
        ("MUL_LTIME__UINT", iec61131std::date_time_numeric_functions::MUL_LTIME__UINT as *const () as usize),
        (
            "MUL_LTIME__UDINT",
            iec61131std::date_time_numeric_functions::MUL_LTIME__UDINT as *const () as usize,
        ),
        (
            "MUL_LTIME__ULINT",
            iec61131std::date_time_numeric_functions::MUL_LTIME__ULINT as *const () as usize,
        ),
        (
            "DIV__TIME__USINT",
            iec61131std::date_time_numeric_functions::DIV__TIME__USINT as *const () as usize,
        ),
        ("DIV__TIME__UINT", iec61131std::date_time_numeric_functions::DIV__TIME__UINT as *const () as usize),
        (
            "DIV__TIME__UDINT",
            iec61131std::date_time_numeric_functions::DIV__TIME__UDINT as *const () as usize,
        ),
        (
            "DIV__TIME__ULINT",
            iec61131std::date_time_numeric_functions::DIV__TIME__ULINT as *const () as usize,
        ),
        ("DIV_TIME__USINT", iec61131std::date_time_numeric_functions::DIV_TIME__USINT as *const () as usize),
        ("DIV_TIME__UINT", iec61131std::date_time_numeric_functions::DIV_TIME__UINT as *const () as usize),
        ("DIV_TIME__UDINT", iec61131std::date_time_numeric_functions::DIV_TIME__UDINT as *const () as usize),
        ("DIV_TIME__ULINT", iec61131std::date_time_numeric_functions::DIV_TIME__ULINT as *const () as usize),
        (
            "DIV_LTIME__USINT",
            iec61131std::date_time_numeric_functions::DIV_LTIME__USINT as *const () as usize,
        ),
        ("DIV_LTIME__UINT", iec61131std::date_time_numeric_functions::DIV_LTIME__UINT as *const () as usize),
        (
            "DIV_LTIME__UDINT",
            iec61131std::date_time_numeric_functions::DIV_LTIME__UDINT as *const () as usize,
        ),
        (
            "DIV_LTIME__ULINT",
            iec61131std::date_time_numeric_functions::DIV_LTIME__ULINT as *const () as usize,
        ),
        ("DIV__TIME__SINT", iec61131std::date_time_numeric_functions::DIV__TIME__SINT as *const () as usize),
        ("DIV__TIME__INT", iec61131std::date_time_numeric_functions::DIV__TIME__INT as *const () as usize),
        ("DIV__TIME__DINT", iec61131std::date_time_numeric_functions::DIV__TIME__DINT as *const () as usize),
        ("DIV__TIME__LINT", iec61131std::date_time_numeric_functions::DIV__TIME__LINT as *const () as usize),
        ("DIV_TIME__SINT", iec61131std::date_time_numeric_functions::DIV_TIME__SINT as *const () as usize),
        ("DIV_TIME__INT", iec61131std::date_time_numeric_functions::DIV_TIME__INT as *const () as usize),
        ("DIV_TIME__DINT", iec61131std::date_time_numeric_functions::DIV_TIME__DINT as *const () as usize),
        ("DIV_TIME__LINT", iec61131std::date_time_numeric_functions::DIV_TIME__LINT as *const () as usize),
        ("DIV_LTIME__SINT", iec61131std::date_time_numeric_functions::DIV_LTIME__SINT as *const () as usize),
        ("DIV_LTIME__INT", iec61131std::date_time_numeric_functions::DIV_LTIME__INT as *const () as usize),
        ("DIV_LTIME__DINT", iec61131std::date_time_numeric_functions::DIV_LTIME__DINT as *const () as usize),
        ("DIV_LTIME__LINT", iec61131std::date_time_numeric_functions::DIV_LTIME__LINT as *const () as usize),
        ("MUL__TIME__REAL", iec61131std::date_time_numeric_functions::MUL__TIME__REAL as *const () as usize),
        ("MUL_TIME__REAL", iec61131std::date_time_numeric_functions::MUL_TIME__REAL as *const () as usize),
        ("MUL_LTIME__REAL", iec61131std::date_time_numeric_functions::MUL_LTIME__REAL as *const () as usize),
        (
            "MUL__TIME__LREAL",
            iec61131std::date_time_numeric_functions::MUL__TIME__LREAL as *const () as usize,
        ),
        ("MUL_TIME__LREAL", iec61131std::date_time_numeric_functions::MUL_TIME__LREAL as *const () as usize),
        (
            "MUL_LTIME__LREAL",
            iec61131std::date_time_numeric_functions::MUL_LTIME__LREAL as *const () as usize,
        ),
        ("DIV__TIME__REAL", iec61131std::date_time_numeric_functions::DIV__TIME__REAL as *const () as usize),
        ("DIV_TIME__REAL", iec61131std::date_time_numeric_functions::DIV_TIME__REAL as *const () as usize),
        ("DIV_LTIME__REAL", iec61131std::date_time_numeric_functions::DIV_LTIME__REAL as *const () as usize),
        (
            "DIV__TIME__LREAL",
            iec61131std::date_time_numeric_functions::DIV__TIME__LREAL as *const () as usize,
        ),
        ("DIV_TIME__LREAL", iec61131std::date_time_numeric_functions::DIV_TIME__LREAL as *const () as usize),
        (
            "DIV_LTIME__LREAL",
            iec61131std::date_time_numeric_functions::DIV_LTIME__LREAL as *const () as usize,
        ),
        ("IS_VALID__REAL", iec61131std::validation_functions::IS_VALID__REAL as *const () as usize),
        ("IS_VALID__LREAL", iec61131std::validation_functions::IS_VALID__LREAL as *const () as usize),
        ("IS_VALID_BCD__BYTE", iec61131std::validation_functions::IS_VALID_BCD__BYTE as *const () as usize),
        ("IS_VALID_BCD__WORD", iec61131std::validation_functions::IS_VALID_BCD__WORD as *const () as usize),
        ("IS_VALID_BCD__DWORD", iec61131std::validation_functions::IS_VALID_BCD__DWORD as *const () as usize),
        ("IS_VALID_BCD__LWORD", iec61131std::validation_functions::IS_VALID_BCD__LWORD as *const () as usize),
        ("TP", iec61131std::timers::TP as *const () as usize),
        ("TP_TIME", iec61131std::timers::TP_TIME as *const () as usize),
        ("TP_LTIME", iec61131std::timers::TP_LTIME as *const () as usize),
        ("TON", iec61131std::timers::TON as *const () as usize),
        ("TON_TIME", iec61131std::timers::TON_TIME as *const () as usize),
        ("TON_LTIME", iec61131std::timers::TON_LTIME as *const () as usize),
        ("TOF", iec61131std::timers::TOF as *const () as usize),
        ("TOF_TIME", iec61131std::timers::TOF_TIME as *const () as usize),
        ("TOF_LTIME", iec61131std::timers::TOF_LTIME as *const () as usize),
        ("SR", iec61131std::bistable_functionblocks::SR as *const () as usize),
        ("RS", iec61131std::bistable_functionblocks::RS as *const () as usize),
        ("R_TRIG", iec61131std::flanks::R_TRIG as *const () as usize),
        ("F_TRIG", iec61131std::flanks::F_TRIG as *const () as usize),
        ("MIN__BOOL", iec61131std::types::MIN__BOOL as *const () as usize),
        ("MIN__SINT", iec61131std::types::MIN__SINT as *const () as usize),
        ("MIN__USINT", iec61131std::types::MIN__USINT as *const () as usize),
        ("MIN__BYTE", iec61131std::types::MIN__BYTE as *const () as usize),
        ("MIN__CHAR", iec61131std::types::MIN__CHAR as *const () as usize),
        ("MIN__INT", iec61131std::types::MIN__INT as *const () as usize),
        ("MIN__UINT", iec61131std::types::MIN__UINT as *const () as usize),
        ("MIN__WORD", iec61131std::types::MIN__WORD as *const () as usize),
        ("MIN__WCHAR", iec61131std::types::MIN__WCHAR as *const () as usize),
        ("MIN__DINT", iec61131std::types::MIN__DINT as *const () as usize),
        ("MIN__UDINT", iec61131std::types::MIN__UDINT as *const () as usize),
        ("MIN__DWORD", iec61131std::types::MIN__DWORD as *const () as usize),
        ("MIN__LINT", iec61131std::types::MIN__LINT as *const () as usize),
        ("MIN__ULINT", iec61131std::types::MIN__ULINT as *const () as usize),
        ("MIN__LWORD", iec61131std::types::MIN__LWORD as *const () as usize),
        ("MIN__REAL", iec61131std::types::MIN__REAL as *const () as usize),
        ("MIN__LREAL", iec61131std::types::MIN__LREAL as *const () as usize),
        ("MIN__DATE", iec61131std::types::MIN__DATE as *const () as usize),
        ("MIN__DATE_AND_TIME", iec61131std::types::MIN__DATE_AND_TIME as *const () as usize),
        ("MIN__TIME", iec61131std::types::MIN__TIME as *const () as usize),
        ("MIN__TIME_OF_DAY", iec61131std::types::MIN__TIME_OF_DAY as *const () as usize),
        ("MAX__BOOL", iec61131std::types::MAX__BOOL as *const () as usize),
        ("MAX__SINT", iec61131std::types::MAX__SINT as *const () as usize),
        ("MAX__USINT", iec61131std::types::MAX__USINT as *const () as usize),
        ("MAX__BYTE", iec61131std::types::MAX__BYTE as *const () as usize),
        ("MAX__CHAR", iec61131std::types::MAX__CHAR as *const () as usize),
        ("MAX__INT", iec61131std::types::MAX__INT as *const () as usize),
        ("MAX__UINT", iec61131std::types::MAX__UINT as *const () as usize),
        ("MAX__WORD", iec61131std::types::MAX__WORD as *const () as usize),
        ("MAX__WCHAR", iec61131std::types::MAX__WCHAR as *const () as usize),
        ("MAX__DINT", iec61131std::types::MAX__DINT as *const () as usize),
        ("MAX__UDINT", iec61131std::types::MAX__UDINT as *const () as usize),
        ("MAX__DWORD", iec61131std::types::MAX__DWORD as *const () as usize),
        ("MAX__LINT", iec61131std::types::MAX__LINT as *const () as usize),
        ("MAX__ULINT", iec61131std::types::MAX__ULINT as *const () as usize),
        ("MAX__LWORD", iec61131std::types::MAX__LWORD as *const () as usize),
        ("MAX__REAL", iec61131std::types::MAX__REAL as *const () as usize),
        ("MAX__LREAL", iec61131std::types::MAX__LREAL as *const () as usize),
        ("MAX__DATE", iec61131std::types::MAX__DATE as *const () as usize),
        ("MAX__DATE_AND_TIME", iec61131std::types::MAX__DATE_AND_TIME as *const () as usize),
        ("MAX__TIME", iec61131std::types::MAX__TIME as *const () as usize),
        ("MAX__TIME_OF_DAY", iec61131std::types::MAX__TIME_OF_DAY as *const () as usize),
        ("LIMIT__BOOL", iec61131std::types::LIMIT__BOOL as *const () as usize),
        ("LIMIT__SINT", iec61131std::types::LIMIT__SINT as *const () as usize),
        ("LIMIT__USINT", iec61131std::types::LIMIT__USINT as *const () as usize),
        ("LIMIT__BYTE", iec61131std::types::LIMIT__BYTE as *const () as usize),
        ("LIMIT__CHAR", iec61131std::types::LIMIT__CHAR as *const () as usize),
        ("LIMIT__INT", iec61131std::types::LIMIT__INT as *const () as usize),
        ("LIMIT__UINT", iec61131std::types::LIMIT__UINT as *const () as usize),
        ("LIMIT__WORD", iec61131std::types::LIMIT__WORD as *const () as usize),
        ("LIMIT__WCHAR", iec61131std::types::LIMIT__WCHAR as *const () as usize),
        ("LIMIT__DINT", iec61131std::types::LIMIT__DINT as *const () as usize),
        ("LIMIT__UDINT", iec61131std::types::LIMIT__UDINT as *const () as usize),
        ("LIMIT__DWORD", iec61131std::types::LIMIT__DWORD as *const () as usize),
        ("LIMIT__LINT", iec61131std::types::LIMIT__LINT as *const () as usize),
        ("LIMIT__ULINT", iec61131std::types::LIMIT__ULINT as *const () as usize),
        ("LIMIT__LWORD", iec61131std::types::LIMIT__LWORD as *const () as usize),
        ("LIMIT__REAL", iec61131std::types::LIMIT__REAL as *const () as usize),
        ("LIMIT__LREAL", iec61131std::types::LIMIT__LREAL as *const () as usize),
        ("LIMIT__DATE", iec61131std::types::LIMIT__DATE as *const () as usize),
        ("LIMIT__DATE_AND_TIME", iec61131std::types::LIMIT__DATE_AND_TIME as *const () as usize),
        ("LIMIT__TIME", iec61131std::types::LIMIT__TIME as *const () as usize),
        ("LIMIT__TIME_OF_DAY", iec61131std::types::LIMIT__TIME_OF_DAY as *const () as usize),
        ("CTU", iec61131std::counters::CTU as *const () as usize),
        ("CTU_INT", iec61131std::counters::CTU_INT as *const () as usize),
        ("CTU_DINT", iec61131std::counters::CTU_DINT as *const () as usize),
        ("CTU_UDINT", iec61131std::counters::CTU_UDINT as *const () as usize),
        ("CTU_LINT", iec61131std::counters::CTU_LINT as *const () as usize),
        ("CTU_ULINT", iec61131std::counters::CTU_ULINT as *const () as usize),
        ("CTD", iec61131std::counters::CTD as *const () as usize),
        ("CTD_INT", iec61131std::counters::CTD_INT as *const () as usize),
        ("CTD_DINT", iec61131std::counters::CTD_DINT as *const () as usize),
        ("CTD_UDINT", iec61131std::counters::CTD_UDINT as *const () as usize),
        ("CTD_LINT", iec61131std::counters::CTD_LINT as *const () as usize),
        ("CTD_ULINT", iec61131std::counters::CTD_ULINT as *const () as usize),
        ("CTUD", iec61131std::counters::CTUD as *const () as usize),
        ("CTUD_INT", iec61131std::counters::CTUD_INT as *const () as usize),
        ("CTUD_DINT", iec61131std::counters::CTUD_DINT as *const () as usize),
        ("CTUD_UDINT", iec61131std::counters::CTUD_UDINT as *const () as usize),
        ("CTUD_LINT", iec61131std::counters::CTUD_LINT as *const () as usize),
        ("CTUD_ULINT", iec61131std::counters::CTUD_ULINT as *const () as usize),
        ("LEN__STRING", iec61131std::string_functions::LEN__STRING as *const () as usize),
        ("LEN__WSTRING", iec61131std::string_functions::LEN__WSTRING as *const () as usize),
        ("FIND__STRING", iec61131std::string_functions::FIND__STRING as *const () as usize),
        ("FIND__WSTRING", iec61131std::string_functions::FIND__WSTRING as *const () as usize),
        ("LEFT_EXT__STRING", iec61131std::string_functions::LEFT_EXT__STRING as *const () as usize),
        ("LEFT_EXT__WSTRING", iec61131std::string_functions::LEFT_EXT__WSTRING as *const () as usize),
        ("RIGHT_EXT__STRING", iec61131std::string_functions::RIGHT_EXT__STRING as *const () as usize),
        ("RIGHT_EXT__WSTRING", iec61131std::string_functions::RIGHT_EXT__WSTRING as *const () as usize),
        ("MID_EXT__STRING", iec61131std::string_functions::MID_EXT__STRING as *const () as usize),
        ("MID_EXT__WSTRING", iec61131std::string_functions::MID_EXT__WSTRING as *const () as usize),
        ("INSERT_EXT__STRING", iec61131std::string_functions::INSERT_EXT__STRING as *const () as usize),
        ("INSERT_EXT__WSTRING", iec61131std::string_functions::INSERT_EXT__WSTRING as *const () as usize),
        ("DELETE_EXT__STRING", iec61131std::string_functions::DELETE_EXT__STRING as *const () as usize),
        ("DELETE_EXT__WSTRING", iec61131std::string_functions::DELETE_EXT__WSTRING as *const () as usize),
        ("REPLACE_EXT__STRING", iec61131std::string_functions::REPLACE_EXT__STRING as *const () as usize),
        ("REPLACE_EXT__WSTRING", iec61131std::string_functions::REPLACE_EXT__WSTRING as *const () as usize),
        ("CONCAT__STRING", iec61131std::string_functions::CONCAT__STRING as *const () as usize),
        ("CONCAT_EXT__STRING", iec61131std::string_functions::CONCAT_EXT__STRING as *const () as usize),
        ("CONCAT__WSTRING", iec61131std::string_functions::CONCAT__WSTRING as *const () as usize),
        ("CONCAT_EXT__WSTRING", iec61131std::string_functions::CONCAT_EXT__WSTRING as *const () as usize),
        ("__STRING_GREATER", iec61131std::string_functions::__STRING_GREATER as *const () as usize),
        ("__WSTRING_GREATER", iec61131std::string_functions::__WSTRING_GREATER as *const () as usize),
        ("__STRING_EQUAL", iec61131std::string_functions::__STRING_EQUAL as *const () as usize),
        ("__WSTRING_EQUAL", iec61131std::string_functions::__WSTRING_EQUAL as *const () as usize),
        ("__STRING_LESS", iec61131std::string_functions::__STRING_LESS as *const () as usize),
        ("__WSTRING_LESS", iec61131std::string_functions::__WSTRING_LESS as *const () as usize),
        ("EXPT__REAL__DINT", iec61131std::arithmetic_functions::EXPT__REAL__DINT as *const () as usize),
        ("EXPT__REAL__REAL", iec61131std::arithmetic_functions::EXPT__REAL__REAL as *const () as usize),
        ("EXPT__REAL__LREAL", iec61131std::arithmetic_functions::EXPT__REAL__LREAL as *const () as usize),
        ("EXPT__LREAL__DINT", iec61131std::arithmetic_functions::EXPT__LREAL__DINT as *const () as usize),
        ("EXPT__LREAL__REAL", iec61131std::arithmetic_functions::EXPT__LREAL__REAL as *const () as usize),
        ("EXPT__LREAL__LREAL", iec61131std::arithmetic_functions::EXPT__LREAL__LREAL as *const () as usize),
        (
            "TO_BIG_ENDIAN__INT",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__INT as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__INT",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__INT as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__INT",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__INT as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__INT",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__INT as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__DINT",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__DINT as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__DINT",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__DINT as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__DINT",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__DINT as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__DINT",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__DINT as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__LINT",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__LINT as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__LINT",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__LINT as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__LINT",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__LINT as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__LINT",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__LINT as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__UINT",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__UINT as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__UINT",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__UINT as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__UINT",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__UINT as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__UINT",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__UINT as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__UDINT",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__UDINT as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__UDINT",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__UDINT as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__UDINT",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__UDINT as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__UDINT",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__UDINT as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__ULINT",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__ULINT as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__ULINT",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__ULINT as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__ULINT",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__ULINT as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__ULINT",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__ULINT as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__REAL",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__REAL as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__REAL",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__REAL as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__REAL",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__REAL as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__REAL",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__REAL as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__LREAL",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__LREAL as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__LREAL",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__LREAL as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__LREAL",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__LREAL as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__LREAL",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__LREAL as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__WORD",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__WORD as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__WORD",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__WORD as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__WORD",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__WORD as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__WORD",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__WORD as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__DWORD",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__DWORD as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__DWORD",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__DWORD as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__DWORD",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__DWORD as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__DWORD",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__DWORD as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__LWORD",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__LWORD as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__LWORD",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__LWORD as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__LWORD",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__LWORD as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__LWORD",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__LWORD as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__WCHAR",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__WCHAR as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__WCHAR",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__WCHAR as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__WCHAR",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__WCHAR as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__WCHAR",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__WCHAR as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__DATE",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__DATE as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__DATE",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__DATE as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__DATE",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__DATE as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__DATE",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__DATE as *const () as usize,
        ),
        (
            "TO_BIG_ENDIAN__TIME_OF_DAY",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__TIME_OF_DAY as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__TIME_OF_DAY",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__TIME_OF_DAY as *const () as usize,
        ),
        (
            "FROM_BIG_ENDIAN__TIME_OF_DAY",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__TIME_OF_DAY as *const () as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__TIME_OF_DAY",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__TIME_OF_DAY as *const ()
                as usize,
        ),
        (
            "TO_BIG_ENDIAN__DATE_AND_TIME",
            iec61131std::endianness_conversion_functions::TO_BIG_ENDIAN__DATE_AND_TIME as *const () as usize,
        ),
        (
            "TO_LITTLE_ENDIAN__DATE_AND_TIME",
            iec61131std::endianness_conversion_functions::TO_LITTLE_ENDIAN__DATE_AND_TIME as *const ()
                as usize,
        ),
        (
            "FROM_BIG_ENDIAN__DATE_AND_TIME",
            iec61131std::endianness_conversion_functions::FROM_BIG_ENDIAN__DATE_AND_TIME as *const ()
                as usize,
        ),
        (
            "FROM_LITTLE_ENDIAN__DATE_AND_TIME",
            iec61131std::endianness_conversion_functions::FROM_LITTLE_ENDIAN__DATE_AND_TIME as *const ()
                as usize,
        ),
        ("BYTE_TO_STRING_EXT", iec61131std::extra_functions::BYTE_TO_STRING_EXT as *const () as usize),
        ("LWORD_TO_STRING_EXT", iec61131std::extra_functions::LWORD_TO_STRING_EXT as *const () as usize),
        ("LINT_TO_STRING_EXT", iec61131std::extra_functions::LINT_TO_STRING_EXT as *const () as usize),
        ("LREAL_TO_STRING_EXT", iec61131std::extra_functions::LREAL_TO_STRING_EXT as *const () as usize),
        ("STRING_TO_LINT", iec61131std::extra_functions::STRING_TO_LINT as *const () as usize),
        ("STRING_TO_DINT", iec61131std::extra_functions::STRING_TO_DINT as *const () as usize),
        ("STRING_TO_LREAL", iec61131std::extra_functions::STRING_TO_LREAL as *const () as usize),
        ("STRING_TO_REAL", iec61131std::extra_functions::STRING_TO_REAL as *const () as usize),
        ("TIME", iec61131std::extra_functions::TIME as *const () as usize),
        ("DT_TO_STRING_EXT", iec61131std::extra_functions::DT_TO_STRING_EXT as *const () as usize),
        ("TIME_TO_STRING_EXT", iec61131std::extra_functions::TIME_TO_STRING_EXT as *const () as usize),
        ("DATE_TO_STRING_EXT", iec61131std::extra_functions::DATE_TO_STRING_EXT as *const () as usize),
        ("TOD_TO_STRING_EXT", iec61131std::extra_functions::TOD_TO_STRING_EXT as *const () as usize),
    ];

    let variables = vec![
        ("PI_REAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::PI_REAL) as *const () as usize),
        ("PI_LREAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::PI_LREAL) as *const () as usize),
        (
            "FRAC_PI_2_REAL",
            std::ptr::addr_of!(iec61131std::arithmetic_functions::FRAC_PI_2_REAL) as *const () as usize,
        ),
        (
            "FRAC_PI_2_LREAL",
            std::ptr::addr_of!(iec61131std::arithmetic_functions::FRAC_PI_2_LREAL) as *const () as usize,
        ),
        (
            "FRAC_PI_4_REAL",
            std::ptr::addr_of!(iec61131std::arithmetic_functions::FRAC_PI_4_REAL) as *const () as usize,
        ),
        (
            "FRAC_PI_4_LREAL",
            std::ptr::addr_of!(iec61131std::arithmetic_functions::FRAC_PI_4_LREAL) as *const () as usize,
        ),
        ("E_REAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::E_REAL) as *const () as usize),
        ("E_LREAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::E_LREAL) as *const () as usize),
        ("INF_REAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::INF_REAL) as *const () as usize),
        ("INF_LREAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::INF_LREAL) as *const () as usize),
        ("NAN_REAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::NAN_REAL) as *const () as usize),
        ("NAN_LREAL", std::ptr::addr_of!(iec61131std::arithmetic_functions::NAN_LREAL) as *const () as usize),
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
