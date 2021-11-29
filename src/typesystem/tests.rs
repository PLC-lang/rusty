use crate::{
    ast::{CompilationUnit, Operator, TypeNature},
    index::visitor::visit,
    lexer::IdProvider,
    typesystem::{
        self, get_equals_function_name_for, get_signed_type, Dimension, BYTE_TYPE, DINT_TYPE,
        DWORD_TYPE, INT_TYPE, LINT_TYPE, LREAL_TYPE, LWORD_TYPE, REAL_TYPE, SINT_TYPE, STRING_TYPE,
        UDINT_TYPE, UINT_TYPE, ULINT_TYPE, USINT_TYPE, WORD_TYPE,
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
    let index = visit(&CompilationUnit::default(), IdProvider::default());
    assert_signed_type!(SINT_TYPE, BYTE_TYPE, index);
    assert_signed_type!(SINT_TYPE, USINT_TYPE, index);
    assert_signed_type!(INT_TYPE, WORD_TYPE, index);
    assert_signed_type!(INT_TYPE, UINT_TYPE, index);
    assert_signed_type!(DINT_TYPE, DWORD_TYPE, index);
    assert_signed_type!(DINT_TYPE, UDINT_TYPE, index);
    assert_signed_type!(LINT_TYPE, ULINT_TYPE, index);
    assert_signed_type!(LINT_TYPE, LWORD_TYPE, index);

    assert_eq!(
        Some(
            index
                .find_effective_type(STRING_TYPE)
                .as_ref()
                .unwrap()
                .get_type_information()
        ),
        get_signed_type(
            index
                .find_effective_type(STRING_TYPE)
                .as_ref()
                .unwrap()
                .get_type_information(),
            &index
        )
    );
}

#[test]
pub fn equal_method_function_names() {
    assert_eq!(
        Some("STRING_EQUAL".to_string()),
        get_equals_function_name_for("STRING", &Operator::Equal)
    );
    assert_eq!(
        Some("MY_TYPE_EQUAL".to_string()),
        get_equals_function_name_for("MY_TYPE", &Operator::Equal)
    );
    assert_eq!(
        Some("STRING_LESS".to_string()),
        get_equals_function_name_for("STRING", &Operator::Less)
    );
    assert_eq!(
        Some("MY_TYPE_LESS".to_string()),
        get_equals_function_name_for("MY_TYPE", &Operator::Less)
    );
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
    let index = visit(&CompilationUnit::default(), IdProvider::default());
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
    assert_eq!(
        int_type,
        typesystem::get_bigger_type(sint_type, int_type, &index)
    );
    assert_eq!(
        dint_type,
        typesystem::get_bigger_type(int_type, dint_type, &index)
    );
    assert_eq!(
        lint_type,
        typesystem::get_bigger_type(lint_type, dint_type, &index)
    );
    assert_eq!(
        uint_type,
        typesystem::get_bigger_type(usint_type, uint_type, &index)
    );
    assert_eq!(
        udint_type,
        typesystem::get_bigger_type(uint_type, udint_type, &index)
    );
    assert_eq!(
        ulint_type,
        typesystem::get_bigger_type(ulint_type, udint_type, &index)
    );
}

#[test]
fn get_bigger_size_integers_mix_test() {
    // Given an initialized index
    let index = visit(&CompilationUnit::default(), IdProvider::default());
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

    assert_eq!(
        int_type,
        typesystem::get_bigger_type(sint_type, int_type, &index)
    );
    assert_eq!(
        dint_type,
        typesystem::get_bigger_type(int_type, dint_type, &index)
    );
    assert_eq!(
        lint_type,
        typesystem::get_bigger_type(lint_type, dint_type, &index)
    );
    assert_eq!(
        uint_type,
        typesystem::get_bigger_type(usint_type, uint_type, &index)
    );
    assert_eq!(
        udint_type,
        typesystem::get_bigger_type(uint_type, udint_type, &index)
    );
    assert_eq!(
        ulint_type,
        typesystem::get_bigger_type(ulint_type, udint_type, &index)
    );
    //The bigger type is the signed
    assert_eq!(
        sint_type,
        typesystem::get_bigger_type(sint_type, usint_type, &index)
    );
    assert_eq!(
        int_type,
        typesystem::get_bigger_type(int_type, uint_type, &index)
    );
    assert_eq!(
        dint_type,
        typesystem::get_bigger_type(dint_type, udint_type, &index)
    );
}

#[test]
fn get_bigger_size_real_test() {
    // Given an initialized index
    let index = visit(&CompilationUnit::default(), IdProvider::default());
    //Given two float numbers (REAL/LREAL)
    let real_type = index.get_type_or_panic(REAL_TYPE);
    let lreal_type = index.get_type_or_panic(LREAL_TYPE);
    //LREAL is bigger than REAL
    assert_eq!(
        lreal_type,
        typesystem::get_bigger_type(lreal_type, real_type, &index)
    );
}

#[test]
fn get_bigger_size_numeric_test() {
    // Given an initialized index
    let index = visit(&CompilationUnit::default(), IdProvider::default());
    //Given a float and an int
    //integer types
    let int_type = index.get_type_or_panic(INT_TYPE);
    let dint_type = index.get_type_or_panic(DINT_TYPE);
    let lint_type = index.get_type_or_panic(LINT_TYPE);

    //Float types
    let real_type = index.get_type_or_panic(REAL_TYPE);
    let lreal_type = index.get_type_or_panic(LREAL_TYPE);
    //The bigger type is the float
    assert_eq!(
        real_type,
        typesystem::get_bigger_type(real_type, int_type, &index)
    );
    assert_eq!(
        real_type,
        typesystem::get_bigger_type(real_type, dint_type, &index)
    );
    //Given an int that is bigger than a float in size (LINT)
    //The bigger type is an LREAL
    assert_eq!(
        lreal_type,
        typesystem::get_bigger_type(lint_type, real_type, &index)
    );
}

#[test]
fn get_bigger_size_string_test() {
    // Given an initialized index
    let index = visit(&CompilationUnit::default(), IdProvider::default());
    //Given two STRING
    let string_1024 = typesystem::DataType {
        name: "STRING_1024".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::String {
            size: TypeSize::LiteralInteger(1024),
            encoding: typesystem::StringEncoding::Utf8,
        },
        natures: vec![
            TypeNature::Any,
            TypeNature::Elementary,
            TypeNature::Num,
            TypeNature::Chars,
            TypeNature::String,
        ],
    };
    let string_30 = typesystem::DataType {
        name: "STRING_30".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::String {
            size: TypeSize::LiteralInteger(30),
            encoding: typesystem::StringEncoding::Utf8,
        },
        natures: vec![
            TypeNature::Any,
            TypeNature::Elementary,
            TypeNature::Num,
            TypeNature::Chars,
            TypeNature::String,
        ],
    };
    //The string with the bigger length is the bigger string
    assert_eq!(
        &string_1024,
        typesystem::get_bigger_type(&string_1024, &string_30, &index)
    );
    assert_eq!(
        &string_1024,
        typesystem::get_bigger_type(&string_30, &string_1024, &index)
    );

    //TODO : Strings with constant sizes
}

#[test]
fn get_bigger_size_array_test_returns_first() {
    // Given an initialized index
    let index = visit(&CompilationUnit::default(), IdProvider::default());
    //Given two ARRAY of the same type and dimensions
    let array_1024 = typesystem::DataType {
        name: "ARRAYG_1024".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::Array {
            name: "ARRAY_1024".into(),
            inner_type_name: "INT".into(),
            dimensions: vec![Dimension {
                start_offset: TypeSize::LiteralInteger(0),
                end_offset: TypeSize::LiteralInteger(1023),
            }],
        },
        natures: vec![TypeNature::Any],
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
        natures: vec![TypeNature::Any],
    };
    //The array with the most elements is bigger
    assert_eq!(
        &array_1024,
        typesystem::get_bigger_type(&array_1024, &array_30, &index)
    );
    assert_eq!(
        &array_30,
        typesystem::get_bigger_type(&array_30, &array_1024, &index)
    );
}

#[test]
fn get_bigger_size_mixed_test_no_() {
    // Given an initialized index
    let index = visit(&CompilationUnit::default(), IdProvider::default());
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
        natures: vec![
            TypeNature::Any,
            TypeNature::Elementary,
            TypeNature::Num,
            TypeNature::Chars,
            TypeNature::String,
        ],
    };
    let wstring_1024 = typesystem::DataType {
        name: "WSTRING_1024".into(),
        initial_value: None,
        information: typesystem::DataTypeInformation::String {
            size: TypeSize::LiteralInteger(1024),
            encoding: typesystem::StringEncoding::Utf16,
        },
        natures: vec![
            TypeNature::Any,
            TypeNature::Elementary,
            TypeNature::Num,
            TypeNature::Chars,
            TypeNature::String,
        ],
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
        natures: vec![TypeNature::Any],
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
        natures: vec![TypeNature::Any],
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
        natures: vec![TypeNature::Any],
    };

    //Given two incompatible types
    //The first given type is returned
    assert_eq!(
        &array_30,
        typesystem::get_bigger_type(&array_30, &array_30_30, &index)
    );
    assert_eq!(
        &string_1024,
        typesystem::get_bigger_type(&string_1024, &array_30, &index)
    );
    assert_eq!(
        &string_1024,
        typesystem::get_bigger_type(&string_1024, &wstring_1024, &index)
    );
    assert_eq!(
        &wstring_1024,
        typesystem::get_bigger_type(&wstring_1024, &string_1024, &index)
    );
    assert_eq!(
        &array_string_30,
        typesystem::get_bigger_type(&array_string_30, &array_30, &index)
    );
    assert_eq!(
        int_type,
        typesystem::get_bigger_type(int_type, &array_30, &index)
    );
}
