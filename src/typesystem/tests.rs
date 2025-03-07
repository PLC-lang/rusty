use plc_ast::ast::{Operator, TypeNature};
use plc_source::source_location::SourceLocation;

use crate::{
    index::Index,
    test_utils::tests::index,
    typesystem::{
        self, get_equals_function_name_for, get_signed_type, Dimension, BOOL_TYPE, BYTE_TYPE, CHAR_TYPE,
        DATE_AND_TIME_TYPE, DATE_TYPE, DINT_TYPE, DWORD_TYPE, INT_TYPE, LINT_TYPE, LREAL_TYPE, LWORD_TYPE,
        REAL_TYPE, SINT_TYPE, STRING_TYPE, TIME_OF_DAY_TYPE, TIME_TYPE, UDINT_TYPE, UINT_TYPE, ULINT_TYPE,
        USINT_TYPE, WCHAR_TYPE, WORD_TYPE, WSTRING_TYPE,
    },
};

use super::TypeSize;

macro_rules! assert_signed_type {
    ($expected:expr, $actual:expr, $index:expr) => {
        assert_eq!(
            $index.find_effective_type_info($expected),
            get_signed_type($index.find_effective_type_info($actual).unwrap(), &$index)
        );
    };
}

#[test]
pub fn signed_types_tests() {
    // Given an initialized index
    let index = get_builtin_index();
    assert_signed_type!(SINT_TYPE, BYTE_TYPE, index);
    assert_signed_type!(SINT_TYPE, USINT_TYPE, index);
    assert_signed_type!(INT_TYPE, WORD_TYPE, index);
    assert_signed_type!(INT_TYPE, UINT_TYPE, index);
    assert_signed_type!(DINT_TYPE, DWORD_TYPE, index);
    assert_signed_type!(DINT_TYPE, UDINT_TYPE, index);
    assert_signed_type!(LINT_TYPE, ULINT_TYPE, index);
    assert_signed_type!(LINT_TYPE, LWORD_TYPE, index);

    assert_eq!(
        Some(index.find_effective_type_by_name(STRING_TYPE).as_ref().unwrap().get_type_information()),
        get_signed_type(
            index.find_effective_type_by_name(STRING_TYPE).as_ref().unwrap().get_type_information(),
            &index
        )
    );
}

#[test]
pub fn equal_method_function_names() {
    assert_eq!(Some("STRING_EQUAL".to_string()), get_equals_function_name_for("STRING", &Operator::Equal));
    assert_eq!(Some("MY_TYPE_EQUAL".to_string()), get_equals_function_name_for("MY_TYPE", &Operator::Equal));
    assert_eq!(Some("STRING_LESS".to_string()), get_equals_function_name_for("STRING", &Operator::Less));
    assert_eq!(Some("MY_TYPE_LESS".to_string()), get_equals_function_name_for("MY_TYPE", &Operator::Less));
    assert_eq!(
        Some("STRING_GREATER".to_string()),
        get_equals_function_name_for("STRING", &Operator::Greater)
    );
    assert_eq!(
        Some("MY_TYPE_GREATER".to_string()),
        get_equals_function_name_for("MY_TYPE", &Operator::Greater)
    );
}

#[test]
fn get_bigger_size_integers_test() {
    // Given an initialized index
    let index = get_builtin_index();
    //Given integer types
    let sint_type = index.get_type_or_panic(SINT_TYPE);
    let int_type = index.get_type_or_panic(INT_TYPE);
    let dint_type = index.get_type_or_panic(DINT_TYPE);
    let lint_type = index.get_type_or_panic(LINT_TYPE);
    //Unsigned
    let usint_type = index.get_type_or_panic(USINT_TYPE);
    let uint_type = index.get_type_or_panic(UINT_TYPE);
    let udint_type = index.get_type_or_panic(UDINT_TYPE);
    let ulint_type = index.get_type_or_panic(ULINT_TYPE);

    //The bigger type is the one with the bigger size
    assert_eq!(int_type, typesystem::get_bigger_type(sint_type, int_type, &index));
    assert_eq!(dint_type, typesystem::get_bigger_type(int_type, dint_type, &index));
    assert_eq!(lint_type, typesystem::get_bigger_type(lint_type, dint_type, &index));
    assert_eq!(uint_type, typesystem::get_bigger_type(usint_type, uint_type, &index));
    assert_eq!(udint_type, typesystem::get_bigger_type(uint_type, udint_type, &index));
    assert_eq!(ulint_type, typesystem::get_bigger_type(ulint_type, udint_type, &index));
}

fn get_builtin_index() -> Index {
    let (_, index) = index("");
    index
}

#[test]
fn get_bigger_size_integers_mix_test() {
    // Given an initialized index
    let index = get_builtin_index();
    //Given integer types
    let sint_type = index.get_type_or_panic(SINT_TYPE);
    let int_type = index.get_type_or_panic(INT_TYPE);
    let dint_type = index.get_type_or_panic(DINT_TYPE);
    let lint_type = index.get_type_or_panic(LINT_TYPE);
    //Unsigned
    let usint_type = index.get_type_or_panic(USINT_TYPE);
    let uint_type = index.get_type_or_panic(UINT_TYPE);
    let udint_type = index.get_type_or_panic(UDINT_TYPE);
    let ulint_type = index.get_type_or_panic(ULINT_TYPE);

    assert_eq!(int_type, typesystem::get_bigger_type(sint_type, int_type, &index));
    assert_eq!(dint_type, typesystem::get_bigger_type(int_type, dint_type, &index));
    assert_eq!(lint_type, typesystem::get_bigger_type(lint_type, dint_type, &index));
    assert_eq!(uint_type, typesystem::get_bigger_type(usint_type, uint_type, &index));
    assert_eq!(udint_type, typesystem::get_bigger_type(uint_type, udint_type, &index));
    assert_eq!(ulint_type, typesystem::get_bigger_type(ulint_type, udint_type, &index));
    //The bigger type is the signed
    assert_eq!(sint_type, typesystem::get_bigger_type(sint_type, usint_type, &index));
    assert_eq!(int_type, typesystem::get_bigger_type(int_type, uint_type, &index));
    assert_eq!(dint_type, typesystem::get_bigger_type(dint_type, udint_type, &index));
}

#[test]
fn get_bigger_size_real_test() {
    // Given an initialized index
    let index = get_builtin_index();
    //Given two float numbers (REAL/LREAL)
    let real_type = index.get_type_or_panic(REAL_TYPE);
    let lreal_type = index.get_type_or_panic(LREAL_TYPE);
    //LREAL is bigger than REAL
    assert_eq!(lreal_type, typesystem::get_bigger_type(lreal_type, real_type, &index));
}

#[test]
fn get_bigger_size_numeric_test() {
    // Given an initialized index
    let index = get_builtin_index();
    //Given a float and an int
    //integer types
    let int_type = index.get_type_or_panic(INT_TYPE);
    let dint_type = index.get_type_or_panic(DINT_TYPE);
    let lint_type = index.get_type_or_panic(LINT_TYPE);

    //Float types
    let real_type = index.get_type_or_panic(REAL_TYPE);
    let lreal_type = index.get_type_or_panic(LREAL_TYPE);
    //The bigger type is the float
    assert_eq!(real_type, typesystem::get_bigger_type(real_type, int_type, &index));
    assert_eq!(real_type, typesystem::get_bigger_type(real_type, dint_type, &index));
    //Given an int that is bigger than a float in size (LINT)
    //The bigger type is an LREAL
    assert_eq!(lreal_type, typesystem::get_bigger_type(lint_type, real_type, &index));
}

#[test]
fn get_bigger_size_string_test() {
    // Given an initialized index
    let index = get_builtin_index();
    //Given two STRING
    let string_1024 = typesystem::DataType {
        name: "STRING_1024".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::String {
            size: TypeSize::LiteralInteger(1024),
            encoding: typesystem::StringEncoding::Utf8,
        },

        nature: TypeNature::String,
        location: SourceLocation::internal(),
    };
    let string_30 = typesystem::DataType {
        name: "STRING_30".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::String {
            size: TypeSize::LiteralInteger(30),
            encoding: typesystem::StringEncoding::Utf8,
        },
        nature: TypeNature::String,
        location: SourceLocation::internal(),
    };
    //The string with the bigger length is the bigger string
    assert_eq!(&string_1024, typesystem::get_bigger_type(&string_1024, &string_30, &index));
    assert_eq!(&string_1024, typesystem::get_bigger_type(&string_30, &string_1024, &index));

    //TODO : Strings with constant sizes
}

#[test]
fn get_bigger_size_array_test_returns_first() {
    // Given an initialized index
    let index = get_builtin_index();
    //Given two ARRAY of the same type and dimensions
    let array_1024 = typesystem::DataType {
        name: "ARRAY_1024".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::Array {
            name: "ARRAY_1024".into(),
            inner_type_name: "INT".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(0),
                end_offset: TypeSize::LiteralInteger(1023),
            }],
        },
        nature: TypeNature::Any,
        location: SourceLocation::internal(),
    };
    let array_30 = typesystem::DataType {
        name: "ARRAY_30".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::Array {
            name: "ARRAY_30".into(),
            inner_type_name: "INT".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(30),
            }],
        },
        nature: TypeNature::Any,
        location: SourceLocation::internal(),
    };
    //The array with the most elements is bigger
    assert_eq!(&array_1024, typesystem::get_bigger_type(&array_1024, &array_30, &index));
    assert_eq!(&array_30, typesystem::get_bigger_type(&array_30, &array_1024, &index));
}

#[test]
fn get_bigger_size_mixed_test_no_() {
    // Given an initialized index
    let index = get_builtin_index();
    //Int
    let int_type = index.get_type_or_panic(INT_TYPE);
    //String
    let string_1024 = typesystem::DataType {
        name: "STRING_1024".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::String {
            size: TypeSize::LiteralInteger(1024),
            encoding: typesystem::StringEncoding::Utf8,
        },
        nature: TypeNature::String,
        location: SourceLocation::internal(),
    };
    let wstring_1024 = typesystem::DataType {
        name: "WSTRING_1024".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::String {
            size: TypeSize::LiteralInteger(1024),
            encoding: typesystem::StringEncoding::Utf16,
        },
        nature: TypeNature::String,
        location: SourceLocation::internal(),
    };
    //Array of string
    let array_string_30 = typesystem::DataType {
        name: "ARRAY_STRING_30".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::Array {
            name: "ARRAY_STRING_30".into(),
            inner_type_name: "STRING".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(30),
            }],
        },
        nature: TypeNature::Any,
        location: SourceLocation::internal(),
    };
    //Array of int
    let array_30 = typesystem::DataType {
        name: "ARRAY_30".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::Array {
            name: "ARRAY_30".into(),
            inner_type_name: "INT".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(30),
            }],
        },
        nature: TypeNature::Any,
        location: SourceLocation::internal(),
    };
    //2-dim array of int
    let array_30_30 = typesystem::DataType {
        name: "ARRAY_30_30".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::Array {
            name: "ARRAY_30_30".into(),
            inner_type_name: "INT".into(),
            dimensions: vec![
                Dimension {
                    start_offset: TypeSize::LiteralInteger(1),
                    end_offset: TypeSize::LiteralInteger(30),
                },
                Dimension {
                    start_offset: TypeSize::LiteralInteger(1),
                    end_offset: TypeSize::LiteralInteger(30),
                },
            ],
        },
        nature: TypeNature::Any,
        location: SourceLocation::internal(),
    };

    //Given two incompatible types
    //The first given type is returned
    assert_eq!(&array_30, typesystem::get_bigger_type(&array_30, &array_30_30, &index));
    assert_eq!(&string_1024, typesystem::get_bigger_type(&string_1024, &array_30, &index));
    assert_eq!(&string_1024, typesystem::get_bigger_type(&string_1024, &wstring_1024, &index));
    assert_eq!(&wstring_1024, typesystem::get_bigger_type(&wstring_1024, &string_1024, &index));
    assert_eq!(&array_string_30, typesystem::get_bigger_type(&array_string_30, &array_30, &index));
    assert_eq!(int_type, typesystem::get_bigger_type(int_type, &array_30, &index));
}

fn get_index() -> Index {
    let mut index = Index::default();
    for t in typesystem::get_builtin_types() {
        index.register_type(t)
    }
    index
}

#[test]
fn any_signed_type_test() {
    let index = get_index();
    let sint = index.get_type_or_panic(SINT_TYPE);
    let int = index.get_type_or_panic(INT_TYPE);
    let dint = index.get_type_or_panic(DINT_TYPE);
    let lint = index.get_type_or_panic(LINT_TYPE);

    assert!(sint.has_nature(TypeNature::Signed, &index));
    assert!(int.has_nature(TypeNature::Signed, &index));
    assert!(dint.has_nature(TypeNature::Signed, &index));
    assert!(lint.has_nature(TypeNature::Signed, &index));

    assert!(sint.has_nature(TypeNature::Int, &index));
    assert!(int.has_nature(TypeNature::Int, &index));
    assert!(dint.has_nature(TypeNature::Int, &index));
    assert!(lint.has_nature(TypeNature::Int, &index));

    assert!(sint.has_nature(TypeNature::Num, &index));
    assert!(int.has_nature(TypeNature::Num, &index));
    assert!(dint.has_nature(TypeNature::Num, &index));
    assert!(lint.has_nature(TypeNature::Num, &index));

    assert!(sint.has_nature(TypeNature::Magnitude, &index));
    assert!(int.has_nature(TypeNature::Magnitude, &index));
    assert!(dint.has_nature(TypeNature::Magnitude, &index));
    assert!(lint.has_nature(TypeNature::Magnitude, &index));

    assert!(sint.has_nature(TypeNature::Elementary, &index));
    assert!(int.has_nature(TypeNature::Elementary, &index));
    assert!(dint.has_nature(TypeNature::Elementary, &index));
    assert!(lint.has_nature(TypeNature::Elementary, &index));

    assert!(sint.has_nature(TypeNature::Any, &index));
    assert!(int.has_nature(TypeNature::Any, &index));
    assert!(dint.has_nature(TypeNature::Any, &index));
    assert!(lint.has_nature(TypeNature::Any, &index));
}

#[test]
fn any_unsigned_type_test() {
    let index = get_index();
    let usint = index.get_type_or_panic(USINT_TYPE);
    let uint = index.get_type_or_panic(UINT_TYPE);
    let udint = index.get_type_or_panic(UDINT_TYPE);
    let ulint = index.get_type_or_panic(ULINT_TYPE);

    assert!(usint.has_nature(TypeNature::Unsigned, &index));
    assert!(uint.has_nature(TypeNature::Unsigned, &index));
    assert!(udint.has_nature(TypeNature::Unsigned, &index));
    assert!(ulint.has_nature(TypeNature::Unsigned, &index));

    assert!(usint.has_nature(TypeNature::Int, &index));
    assert!(uint.has_nature(TypeNature::Int, &index));
    assert!(udint.has_nature(TypeNature::Int, &index));
    assert!(ulint.has_nature(TypeNature::Int, &index));

    assert!(usint.has_nature(TypeNature::Num, &index));
    assert!(uint.has_nature(TypeNature::Num, &index));
    assert!(udint.has_nature(TypeNature::Num, &index));
    assert!(ulint.has_nature(TypeNature::Num, &index));

    assert!(usint.has_nature(TypeNature::Magnitude, &index));
    assert!(uint.has_nature(TypeNature::Magnitude, &index));
    assert!(udint.has_nature(TypeNature::Magnitude, &index));
    assert!(ulint.has_nature(TypeNature::Magnitude, &index));

    assert!(usint.has_nature(TypeNature::Elementary, &index));
    assert!(uint.has_nature(TypeNature::Elementary, &index));
    assert!(udint.has_nature(TypeNature::Elementary, &index));
    assert!(ulint.has_nature(TypeNature::Elementary, &index));

    assert!(usint.has_nature(TypeNature::Any, &index));
    assert!(uint.has_nature(TypeNature::Any, &index));
    assert!(udint.has_nature(TypeNature::Any, &index));
    assert!(ulint.has_nature(TypeNature::Any, &index));
}

#[test]
fn any_real_type_test() {
    let index = get_index();
    let real = index.get_type_or_panic(REAL_TYPE);
    let lreal = index.get_type_or_panic(LREAL_TYPE);

    assert!(real.has_nature(TypeNature::Real, &index));
    assert!(lreal.has_nature(TypeNature::Real, &index));

    assert!(real.has_nature(TypeNature::Num, &index));
    assert!(lreal.has_nature(TypeNature::Num, &index));

    assert!(real.has_nature(TypeNature::Magnitude, &index));
    assert!(lreal.has_nature(TypeNature::Magnitude, &index));

    assert!(real.has_nature(TypeNature::Elementary, &index));
    assert!(lreal.has_nature(TypeNature::Elementary, &index));

    assert!(real.has_nature(TypeNature::Any, &index));
    assert!(lreal.has_nature(TypeNature::Any, &index));
}

#[test]
fn any_duration_type_test() {
    let index = get_index();
    let time = index.get_type_or_panic(TIME_TYPE);
    // let ltime = index.get_type_or_panic(LTIME_TYTE);

    assert!(time.has_nature(TypeNature::Duration, &index));

    assert!(time.has_nature(TypeNature::Magnitude, &index));

    assert!(time.has_nature(TypeNature::Elementary, &index));

    assert!(time.has_nature(TypeNature::Any, &index));
}

#[test]
fn any_bit_type_test() {
    let index = get_index();
    let bool_type = index.get_type_or_panic(BOOL_TYPE);
    let byte = index.get_type_or_panic(BYTE_TYPE);
    let word = index.get_type_or_panic(WORD_TYPE);
    let dword = index.get_type_or_panic(DWORD_TYPE);
    let lword = index.get_type_or_panic(LWORD_TYPE);

    assert!(bool_type.has_nature(TypeNature::Bit, &index));
    assert!(byte.has_nature(TypeNature::Bit, &index));
    assert!(word.has_nature(TypeNature::Bit, &index));
    assert!(dword.has_nature(TypeNature::Bit, &index));
    assert!(lword.has_nature(TypeNature::Bit, &index));

    assert!(bool_type.has_nature(TypeNature::Elementary, &index));
    assert!(byte.has_nature(TypeNature::Elementary, &index));
    assert!(word.has_nature(TypeNature::Elementary, &index));
    assert!(dword.has_nature(TypeNature::Elementary, &index));
    assert!(lword.has_nature(TypeNature::Elementary, &index));

    assert!(bool_type.has_nature(TypeNature::Any, &index));
    assert!(byte.has_nature(TypeNature::Any, &index));
    assert!(word.has_nature(TypeNature::Any, &index));
    assert!(dword.has_nature(TypeNature::Any, &index));
    assert!(lword.has_nature(TypeNature::Any, &index));
}

#[test]
fn any_string_type_test() {
    let index = get_index();
    let string = index.get_type_or_panic(STRING_TYPE);
    let wstring = index.get_type_or_panic(WSTRING_TYPE);

    assert!(string.has_nature(TypeNature::Chars, &index));
    assert!(wstring.has_nature(TypeNature::Chars, &index));

    assert!(string.has_nature(TypeNature::String, &index));
    assert!(wstring.has_nature(TypeNature::String, &index));

    assert!(string.has_nature(TypeNature::Elementary, &index));
    assert!(wstring.has_nature(TypeNature::Elementary, &index));

    assert!(string.has_nature(TypeNature::Any, &index));
    assert!(wstring.has_nature(TypeNature::Any, &index));
}

#[test]
fn any_char_type_test() {
    let index = get_index();
    let char = index.get_type_or_panic(CHAR_TYPE);
    let wchar = index.get_type_or_panic(WCHAR_TYPE);

    assert!(char.has_nature(TypeNature::Chars, &index));
    assert!(wchar.has_nature(TypeNature::Chars, &index));

    assert!(char.has_nature(TypeNature::Char, &index));
    assert!(wchar.has_nature(TypeNature::Char, &index));

    assert!(char.has_nature(TypeNature::Elementary, &index));
    assert!(wchar.has_nature(TypeNature::Elementary, &index));

    assert!(char.has_nature(TypeNature::Any, &index));
    assert!(wchar.has_nature(TypeNature::Any, &index));
}

#[test]
fn any_date_type_test() {
    let index = get_index();
    let date = index.get_type_or_panic(DATE_TYPE);
    let date_time = index.get_type_or_panic(DATE_AND_TIME_TYPE);
    let tod = index.get_type_or_panic(TIME_OF_DAY_TYPE);

    assert!(date.has_nature(TypeNature::Date, &index));
    assert!(date_time.has_nature(TypeNature::Date, &index));
    assert!(tod.has_nature(TypeNature::Date, &index));

    assert!(date.has_nature(TypeNature::Elementary, &index));
    assert!(date_time.has_nature(TypeNature::Elementary, &index));
    assert!(tod.has_nature(TypeNature::Elementary, &index));

    assert!(date.has_nature(TypeNature::Any, &index));
    assert!(date_time.has_nature(TypeNature::Any, &index));
    assert!(tod.has_nature(TypeNature::Any, &index));
}

#[test]
fn array_size_single_dim_tests() {
    let index = get_index();
    //Given an ARRAY[1..20] OF INT
    let array_20 = typesystem::DataType {
        name: "ARRAY_20".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::Array {
            name: "ARRAY_20".into(),
            inner_type_name: "INT".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(20),
            }],
        },
        nature: TypeNature::Any,
        location: SourceLocation::internal(),
    };
    //the size of the array is 20*size(int)
    assert_eq!(320, array_20.get_type_information().get_size_in_bits(&index).unwrap());
}

#[test]
fn array_size_multi_dim_tests() {
    let index = get_index();
    //Given an ARRAY[1..20] OF INT
    let array_20_20 = typesystem::DataType {
        name: "ARRAY_20_20".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::Array {
            name: "ARRAY_20_20".into(),
            inner_type_name: "INT".into(),
            dimensions: vec![
                Dimension {
                    start_offset: TypeSize::LiteralInteger(1),
                    end_offset: TypeSize::LiteralInteger(20),
                },
                Dimension {
                    start_offset: TypeSize::LiteralInteger(-1),
                    end_offset: TypeSize::LiteralInteger(18),
                },
            ],
        },
        nature: TypeNature::Any,
        location: SourceLocation::internal(),
    };
    //the size of the array is 20*size(int)
    assert_eq!(6400, array_20_20.get_type_information().get_size_in_bits(&index).unwrap());
}

#[test]
fn array_size_nested_tests() {
    let mut index = get_index();
    //Given an ARRAY[1..20] OF INT
    let array_20 = typesystem::DataType {
        name: "ARRAY_20".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::Array {
            name: "ARRAY_20".into(),
            inner_type_name: "INT".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(20),
            }],
        },
        nature: TypeNature::Any,
        location: SourceLocation::internal(),
    };
    index.register_type(array_20);
    let nested_array = typesystem::DataType {
        name: "NESTED_ARRAY".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::Array {
            name: "NESTED_ARRAY".into(),
            inner_type_name: "ARRAY_20".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(1),
                end_offset: TypeSize::LiteralInteger(20),
            }],
        },
        nature: TypeNature::Any,
        location: SourceLocation::internal(),
    };

    //the size of the array is 20*size(int)
    assert_eq!(6400, nested_array.get_type_information().get_size_in_bits(&index).unwrap());
}
