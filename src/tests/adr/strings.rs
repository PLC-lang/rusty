use crate::test_utils::tests::codegen;

/// # Architecture Design Record: Strings
/// A String is a defined as a sequence of i18 or i16 elements with a fixed size in ST.
/// A string has 2 defined datatypes the `STRING` type, which holds a UTF-8 sequence, or a `WSTRING`
/// which holds a UTF-16 sequence.
/// Unless otherwise specified, a string is 80 charachters long
/// A String always provides an extra charachter for a null terminator
#[test]
fn string_types_are_byte_sequence() {

}

/// String literals are stored as global static values
/// Cast strings are stored in literals
/// Cast strings are marked with `STRING#'value'` or `WSTRING#"value"`
#[test]
fn literal_strings_are_stored_as_static() {

}

/// A String literal is terminated by a null terminator `\0`
#[test]
fn strings_termitated_with_nul() {

}

/// String parameters are copied to the exact size of the target parameter
/// This is done by using a compination of memset to zero out the space,
/// followed by a memcpy to the target size
#[test]
fn strings_in_pous_are_copied() {

}

/// Strings being bassed to a function are allocated as temporary variables
/// This ensures that we have a variable with the target size of the parameter to store
#[test]
fn strings_passed_to_functions_allocated() {

}

/// Strings, including literals can be passed to auto pointer type blocks like a VAR_IN_OUT
/// This includes literal strings
/// For literal string, a pointer to the statically allocated string value is passed
#[test]
fn literals_can_be_passed_as_autopointers() {

}