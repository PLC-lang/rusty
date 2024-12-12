use std::{
    char::{decode_utf16, DecodeUtf16Error},
    cmp::Ordering,
};

use num::PrimInt;

/// # Helper function
///
/// Gets the amount of continuous characters before
/// the first null-terminator.
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// May return an incorrect value if passed an
/// array filled with (non-zero) garbage values.
pub unsafe fn get_null_terminated_len<T: PrimInt>(src: *const T) -> usize {
    if src.is_null() {
        return 0;
    }

    (0..).take_while(|&i| !(*src.add(i)).is_zero()).count()
}

/// # Helper function
///
/// Get slice from null-terminated pointer.
/// If no number of bytes is given, nbytes will be determined
/// by finding the nul-terminator.
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
pub unsafe fn ptr_to_slice<'a, T: PrimInt>(src: *const T) -> &'a [T] {
    let nbytes = get_null_terminated_len(src);
    std::slice::from_raw_parts(src, nbytes)
}

type Utf16Iterator<'a> = std::char::DecodeUtf16<std::iter::Copied<std::slice::Iter<'a, u16>>>;
type Utf8Iterator<'a> = core::str::Chars<'a>;

pub trait CharsDecoder<T: PrimInt> {
    type IteratorType: Iterator;
    /// Decodes raw UTF-8 or UTF-16 codepoints into a character iterator. Does not account
    /// for grapheme clusters.
    ///
    /// # Safety
    ///
    /// Works on raw pointers, inherently unsafe.
    unsafe fn decode(src: *const T) -> EncodedCharsIter<Self::IteratorType>;
}

pub trait CharsEncoder<T: PrimInt>: Iterator {
    /// Encodes UTF-8 or UTF-16 character iterator. Its raw codepoints are written
    /// into given destination buffer address.
    ///
    /// # Safety
    ///
    /// Works on raw pointers, inherently unsafe. Does not ensure that the buffer at the
    /// given address large enough. It will continue to write unchecked until all characters
    /// have been processed and can therefore result in UB.
    unsafe fn encode(self, dest: &mut *mut T);
}

#[derive(Debug)]
pub struct EncodedCharsIter<T: Iterator> {
    iter: T,
}

impl<T: Iterator> Iterator for EncodedCharsIter<T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> CharsDecoder<u8> for EncodedCharsIter<Utf8Iterator<'a>> {
    type IteratorType = Utf8Iterator<'a>;
    unsafe fn decode(src: *const u8) -> Self {
        let slice = ptr_to_slice(src);
        Self { iter: std::str::from_utf8(slice).unwrap().chars() }
    }
}

impl<I: Iterator<Item = char>> CharsEncoder<u8> for I {
    unsafe fn encode(self, dest: &mut *mut u8) {
        for char in self {
            let mut temp = [0; 4];
            let slice = char.encode_utf8(&mut temp);
            for byte in slice.as_bytes() {
                **dest = *byte;
                *dest = dest.add(1);
            }
        }

        **dest = 0;
    }
}

impl<I: Iterator<Item = Result<char, DecodeUtf16Error>>> CharsEncoder<u16> for I {
    unsafe fn encode(self, dest: &mut *mut u16) {
        for c in self {
            let mut temp = [0_u16; 2];
            let slice = c.unwrap().encode_utf16(&mut temp);
            for word in slice {
                **dest = *word;
                *dest = dest.add(1);
            }
        }

        **dest = 0;
    }
}

impl<'a> CharsDecoder<u16> for EncodedCharsIter<Utf16Iterator<'a>> {
    type IteratorType = Utf16Iterator<'a>;
    unsafe fn decode(src: *const u16) -> Self {
        let src = ptr_to_slice(src);
        Self { iter: decode_utf16(src.iter().copied()) }
    }
}

/// Gets length of the given character string.
/// Encoding: UTF-8
///
/// Works on raw pointers, inherently unsafe.
/// May return an incorrect value if passed an
/// array filled with (non-zero) garbage values.
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LEN__STRING(src: *const u8) -> i32 {
    EncodedCharsIter::decode(src).count() as i32
}

/// Gets length of the given string.
/// Encoding: UTF-16
///
/// Works on raw pointers, inherently unsafe.
/// May return an incorrect value if passed an
/// array filled with (non-zero) garbage values.
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn LEN__WSTRING(src: *const u16) -> i32 {
    EncodedCharsIter::decode(src).count() as i32
}

/// Finds the first occurance of the second string (in2)
/// in the first string (in1).
/// Encoding: UTF-8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn FIND__STRING(src1: *const u8, src2: *const u8) -> i32 {
    let haystack = ptr_to_slice(src1);
    let needle = ptr_to_slice(src2);

    if needle.len() > haystack.len() || haystack.is_empty() || needle.is_empty() {
        return 0;
    }

    if let Some(idx) = haystack.windows(needle.len()).position(|window| window == needle) {
        // get chars until byte index
        let char_index = core::str::from_utf8(std::slice::from_raw_parts(src1, idx)).unwrap().chars().count();
        // correct for ST indexing
        char_index as i32 + 1
    } else {
        0
    }
}
/// Finds the first occurance of the second string (src2)
/// within the first string (src1).
/// Encoding: UTF-16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn FIND__WSTRING(src1: *const u16, src2: *const u16) -> i32 {
    let haystack = ptr_to_slice(src1);
    let needle = ptr_to_slice(src2);

    if needle.len() > haystack.len() || haystack.is_empty() || needle.is_empty() {
        return 0;
    }

    if let Some(idx) = haystack.windows(needle.len()).position(|window| window == needle) {
        // match found. count utf16 chars to window position
        let char_index = decode_utf16(std::slice::from_raw_parts(src1, idx).iter().copied()).count();

        // correct indexing for ST
        char_index as i32 + 1
    } else {
        0
    }
}

/// Writes a substring of a specified length from the given string,
/// to the destination buffer, beginning with the first (leftmost) character.
/// Encoding: UTF-8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the requested substring length is either negative or
/// longer than the base string.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn LEFT_EXT__STRING(src: *const u8, substr_len: i32, dest: *mut u8) -> i32 {
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }
    let mut dest = dest;
    let substr_len = substr_len as usize;
    let nchars = EncodedCharsIter::decode(src).count();
    if nchars < substr_len {
        panic!("Requested substring length exceeds string length.")
    }
    let chars = EncodedCharsIter::decode(src).take(substr_len);
    chars.encode(&mut dest);

    0
}

/// Writes a substring of a specified length from the given string,
/// to the destination buffer, beginning with the first (leftmost) character.
/// Encoding: UTF-16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the requested substring length is either negative or
/// longer than the base string.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn LEFT_EXT__WSTRING(src: *const u16, substr_len: i32, dest: *mut u16) -> i32 {
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }
    let mut dest = dest;
    let substr_len = substr_len as usize;
    let nchars = EncodedCharsIter::decode(src).count();
    if nchars < substr_len {
        panic!("Requested substring length exceeds string length.")
    }
    let chars = EncodedCharsIter::decode(src).take(substr_len);
    chars.encode(&mut dest);

    0
}

/// Writes a substring of a specified length from the given string,
/// to the destination buffer, ending with the last (rightmost) character.
/// Encoding: UTF-8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the requested substring length is either negative or
/// longer than the base string.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn RIGHT_EXT__STRING(src: *const u8, substr_len: i32, dest: *mut u8) -> i32 {
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }
    let mut dest = dest;
    let substr_len = substr_len as usize;
    let nchars = EncodedCharsIter::decode(src).count();
    if nchars < substr_len {
        panic!("Requested substring length exceeds string length.")
    }
    let chars = EncodedCharsIter::decode(src).skip(nchars - substr_len);
    chars.encode(&mut dest);

    0
}

/// Writes a substring of a specified length from the given string
/// to the destination buffer, ending with the last (rightmost) character.
/// Encoding: UTF-16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the requested substring length is either negative or
/// longer than the base string.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn RIGHT_EXT__WSTRING(src: *const u16, substr_len: i32, dest: *mut u16) -> i32 {
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }
    let mut dest = dest;
    let substr_len = substr_len as usize;
    let nchars = EncodedCharsIter::decode(src).count();
    if nchars < substr_len {
        panic!("Requested substring length exceeds string length.")
    }
    let chars = EncodedCharsIter::decode(src).skip(nchars - substr_len);
    chars.encode(&mut dest);
    0
}

/// Writes a substring of a specified length from the given string
/// to the destination buffer, beginning at the given index.
/// Encoding: UTF-8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the requested substring length or position are negative
/// or the substring length exceeds the remaining characters from the
/// starting position of the base string.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn MID_EXT__STRING(
    src: *const u8,
    substr_len: i32,
    start_index: i32,
    dest: *mut u8,
) -> i32 {
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }
    let mut dest = dest;
    let substr_len = substr_len as usize;
    let start_index = start_index as usize;
    let nchars = EncodedCharsIter::decode(src).count();
    if start_index < 1 || start_index > nchars {
        panic!("Position is out of bounds.")
    }
    // correct for 0-indexing
    let start_index = start_index - 1;
    if nchars < substr_len + start_index {
        panic!("Requested substring length {substr_len} from position {start_index} exceeds string length.")
    }
    let chars = EncodedCharsIter::decode(src).skip(start_index).take(substr_len);
    chars.encode(&mut dest);

    0
}

/// Writes a substring of a specified length from the given string
/// to the destination buffer, beginning at the given index.
/// Encoding: UTF-16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the requested substring length or position are negative
/// or the substring length exceeds the remaining characters from the
/// starting position of the base string.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn MID_EXT__WSTRING(
    src: *const u16,
    substr_len: i32,
    start_index: i32,
    dest: *mut u16,
) -> i32 {
    if substr_len < 0 {
        panic!("Length parameter cannot be negative.");
    }
    let mut dest = dest;
    let substr_len = substr_len as usize;
    let start_index = start_index as usize;
    let nchars = EncodedCharsIter::decode(src).count();
    if start_index < 1 || start_index > nchars {
        panic!("Position is out of bounds.")
    }
    // correct for 0-indexing
    let start_index = start_index - 1;
    if nchars < substr_len + start_index {
        panic!("Requested substring length {substr_len} from position {start_index} exceeds string length.")
    }
    let chars = EncodedCharsIter::decode(src).skip(start_index).take(substr_len);
    chars.encode(&mut dest);

    0
}

/// Inserts a string into another string at the
/// specified position and writes the resulting string to
/// the destination buffer.
/// Encoding: UTF-8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the position parameter exceeds the
/// source array bounds.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn INSERT_EXT__STRING(
    src_base: *const u8,
    src_to_insert: *const u8,
    pos: i32,
    dest: *mut u8,
) -> i32 {
    let mut dest = dest;
    let nchars = EncodedCharsIter::decode(src_base).count();
    if pos < 0 || pos > nchars as i32 {
        panic! {"Positional parameter is out of bounds."}
    }
    let pos = pos as usize;
    EncodedCharsIter::decode(src_base)
        .take(pos)
        .chain(EncodedCharsIter::decode(src_to_insert))
        .chain(EncodedCharsIter::decode(src_base).skip(pos))
        .encode(&mut dest);

    0
}

/// Inserts a string into another string at the
/// specified position and writes the resulting string to
/// the destination buffer.
/// Encoding: UTF-16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the position parameter exceeds the
/// source array bounds.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn INSERT_EXT__WSTRING(
    src_base: *const u16,
    src_to_insert: *const u16,
    pos: i32,
    dest: *mut u16,
) -> i32 {
    let mut dest = dest;
    let nchars = EncodedCharsIter::decode(src_base).count();
    if pos < 0 || pos > nchars as i32 {
        panic! {"Positional parameter is out of bounds."}
    }
    let pos = pos as usize;
    EncodedCharsIter::decode(src_base)
        .take(pos)
        .chain(EncodedCharsIter::decode(src_to_insert))
        .chain(EncodedCharsIter::decode(src_base).skip(pos))
        .encode(&mut dest);

    0
}

/// Deletes the given amount of characters in a string,
/// starting from the specified position. Writes the resulting
/// string into a destination buffer.
/// Encoding: UTF-8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the position parameter is out of bounds of the
/// array or if trying to delete too many characters.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn DELETE_EXT__STRING(
    src: *const u8,
    num_chars_to_delete: i32,
    pos: i32,
    dest: *mut u8,
) -> i32 {
    let mut dest = dest;
    let nchars = EncodedCharsIter::decode(src).count();
    if pos < 1 || pos > nchars as i32 {
        panic!("Index out of bounds.")
    }
    // correct for 0-indexing
    let pos = pos as usize - 1;
    let ndel = num_chars_to_delete as usize;
    if ndel + pos > nchars {
        panic!(
            r#"Cannot delete {} characters starting from index {}.
            Index out of bounds.
            "#,
            num_chars_to_delete,
            pos + 1
        )
    }

    EncodedCharsIter::decode(src)
        .take(pos)
        .chain(EncodedCharsIter::decode(src).skip(ndel + pos))
        .encode(&mut dest);
    0
}

/// Deletes the given amount of characters in a string,
/// starting from the specified position. Writes the resulting
/// string into a destination buffer.
/// Encoding: UTF-16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if the position parameter is out of bounds of the
/// array or if trying to delete too many characters.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn DELETE_EXT__WSTRING(
    src: *const u16,
    num_chars_to_delete: i32,
    pos: i32,
    dest: *mut u16,
) -> i32 {
    let mut dest = dest;
    let nchars = EncodedCharsIter::decode(src).count();
    if pos < 1 || pos > nchars as i32 {
        panic!("Index out of bounds.")
    }
    // correct for 0-indexing
    let pos = pos as usize - 1;
    let ndel = num_chars_to_delete as usize;
    if ndel + pos > nchars {
        panic!(
            r#"Cannot delete {} characters starting from index {}.
            Index out of bounds.
            "#,
            num_chars_to_delete,
            pos + 1
        )
    }

    EncodedCharsIter::decode(src)
        .take(pos)
        .chain(EncodedCharsIter::decode(src).skip(pos + ndel))
        .encode(&mut dest);

    0
}

/// Replaces the given amount of characters in a string, starting
/// from a specified location in the string, with another string and
/// writes it to the destination buffer.
/// Encoding: UTF-8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if trying to index outside of the array or trying
/// to replace more characters than remaining.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn REPLACE_EXT__STRING(
    src_base: *const u8,
    src_replacement: *const u8,
    num_chars_to_replace: i32,
    pos: i32,
    dest: *mut u8,
) -> i32 {
    let mut dest = dest;
    let nbase = EncodedCharsIter::decode(src_base).count();
    if pos < 1 || pos > nbase as i32 {
        panic!("Index out of bounds.")
    }
    // correct for 0-indexing
    let pos = (pos - 1) as usize;
    let nreplace = num_chars_to_replace as usize;

    if nreplace + pos > nbase {
        panic!(
            r#"Cannot replace {} characters starting from index {}.
            Index out of bounds.
            "#,
            nreplace,
            pos + 1
        )
    }
    EncodedCharsIter::decode(src_base)
        .take(pos)
        .chain(
            EncodedCharsIter::decode(src_replacement)
                .chain(EncodedCharsIter::decode(src_base).skip(pos + nreplace)),
        )
        .encode(&mut dest);

    0
}

/// Replaces the given amount of characters in a string, starting
/// from a specified location in the string, with another string and
/// writes it to the destination buffer.
/// Encoding: UTF-16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if trying to index outside of the array or trying
/// to replace more characters than remaining.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn REPLACE_EXT__WSTRING(
    src_base: *const u16,
    src_replacement: *const u16,
    num_chars_to_replace: i32,
    pos: i32,
    dest: *mut u16,
) -> i32 {
    let mut dest = dest;
    let nbase = EncodedCharsIter::decode(src_base).count();
    if pos < 1 || pos > nbase as i32 {
        panic!("Index out of bounds.")
    }
    // correct for 0-indexing
    let pos = (pos - 1) as usize;
    let nreplace = num_chars_to_replace as usize;
    if nreplace + pos > nbase {
        panic!(
            r#"Cannot replace {} characters starting from index {}.
            Index out of bounds.
            "#,
            nreplace,
            pos + 1
        )
    }
    EncodedCharsIter::decode(src_base)
        .take(pos)
        .chain(
            EncodedCharsIter::decode(src_replacement)
                .chain(EncodedCharsIter::decode(src_base).skip(pos + nreplace)),
        )
        .encode(&mut dest);

    0
}

/// Concatenates all given strings in the order in which they are given.
/// Strings are passed as pointer of pointer to u8, where each pointer represents
/// the starting address of each string. The amount of strings must be passed as
/// argument.
/// Encoding: UTF-8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if trying to index outside of the array or trying
/// to replace more characters than remaining.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CONCAT__STRING(dest: *mut u8, argc: i32, argv: *const *const u8) {
    let _ = CONCAT_EXT__STRING(dest, argc, argv);
}

/// Concatenates all given strings in the order in which they are given.
/// Strings are passed as pointer of pointer to u8, where each pointer represents
/// the starting address of each string. The amount of strings must be passed as
/// argument.
/// Encoding: UTF-8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if trying to index outside of the array or trying
/// to replace more characters than remaining.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn CONCAT_EXT__STRING(dest: *mut u8, argc: i32, argv: *const *const u8) -> i32 {
    if argv.is_null() || dest.is_null() {
        panic!("Received null-pointer.")
    }
    let mut dest = dest;
    let mut argv = argv;
    for _ in 0..argc {
        EncodedCharsIter::decode(*argv).encode(&mut dest);
        argv = argv.add(1);
    }

    0
}

/// Concatenates all given strings in the order in which they are given.
/// Strings are passed as pointer of pointer to u8, where each pointer represents
/// the starting address of each string. The amount of strings must be passed as
/// argument.
/// Encoding: UTF-16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if trying to index outside of the array or trying
/// to replace more characters than remaining.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CONCAT__WSTRING(dest: *mut u16, argc: i32, argv: *const *const u16) {
    let _ = CONCAT_EXT__WSTRING(dest, argc, argv);
}

/// Concatenates all given strings in the order in which they are given.
/// Strings are passed as pointer of pointer to u8, where each pointer represents
/// the starting address of each string. The amount of strings must be passed as
/// argument.
/// Encoding: UTF-16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
/// Will panic if trying to index outside of the array or trying
/// to replace more characters than remaining.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn CONCAT_EXT__WSTRING(
    dest: *mut u16,
    argc: i32,
    argv: *const *const u16,
) -> i32 {
    if argv.is_null() || dest.is_null() {
        panic!("Received null-pointer.")
    }
    let mut dest = dest;
    let mut argv = argv;
    for _ in 0..argc {
        EncodedCharsIter::decode(*argv).encode(&mut dest);
        argv = argv.add(1);
    }

    0
}

/// Helper function for generic, variadic string equality functions.
fn compare<T>(argc: i32, argv: *const *const T, predicate_func: fn(Ordering) -> bool) -> bool
where
    T: Ord + PrimInt,
{
    if argc < 2 {
        panic!("Too few arguments for function call.")
    }
    let mut argv = argv;
    unsafe {
        let mut previous = ptr_to_slice(*argv);
        for _ in 0..argc - 1 {
            argv = argv.add(1);
            let current = ptr_to_slice(*argv);
            if !(predicate_func(previous.cmp(current))) {
                return false;
            }
            previous = current;
        }
    }
    true
}

/// Extensible "greater than" comparison function.
/// Encoding: UTF-8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn __STRING_GREATER(argc: i32, argv: *const *const u8) -> bool {
    compare(argc, argv, Ordering::is_gt)
}

/// Extensible "greater than" comparison function.
/// Encoding: UTF-16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn __WSTRING_GREATER(argc: i32, argv: *const *const u16) -> bool {
    compare(argc, argv, Ordering::is_gt)
}

/// Extensible "equal" comparison function.
/// Encoding: UTF-8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn __STRING_EQUAL(argc: i32, argv: *const *const u8) -> bool {
    compare(argc, argv, Ordering::is_eq)
}

/// Extensible "equal" comparison function.
/// Encoding: UTF-16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn __WSTRING_EQUAL(argc: i32, argv: *const *const u16) -> bool {
    compare(argc, argv, Ordering::is_eq)
}

/// Extensible "less than" comparison function.
/// Encoding: UTF-8
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn __STRING_LESS(argc: i32, argv: *const *const u8) -> bool {
    compare(argc, argv, Ordering::is_lt)
}

/// Extensible "less than" comparison function.
/// Encoding: UTF-16
///
/// # Safety
///
/// Works on raw pointers, inherently unsafe.
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C-unwind" fn __WSTRING_LESS(argc: i32, argv: *const *const u16) -> bool {
    compare(argc, argv, Ordering::is_lt)
}

// -------------------------------------------------unit tests-----------------------------------------
#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CStr;

    const DEFAULT_STRING_SIZE: usize = 2048;
    // -----------------------------------UTF8
    #[test]
    fn test_len_correct_utf8_character_count() {
        let src = "ϕϚϡϗabcd\0";
        unsafe {
            let res = LEN__STRING(src.as_ptr());
            assert_eq!(res, 8)
        }
    }

    #[ignore = "The user is responsible for correctly counting composed characters if they choose to use them."]
    #[test]
    fn test_len_with_precomposed_characters() {
        // these characters are not the same. one is precomposed (len 1)
        // and the other is composed of two characters. they are merely rendered the same.
        let src = "éé\0";
        unsafe {
            let res = LEN__STRING(src.as_ptr());
            assert_eq!(res, 2)
        }
    }

    #[test]
    fn test_find_index_correct() {
        let haystack = "hϗllo wϕrld\0";
        let needle = "llo\0";
        unsafe {
            let res = FIND__STRING(haystack.as_ptr(), needle.as_ptr());
            assert_eq!(res, 3)
        }
    }

    #[test]
    fn test_find_index_correct_edge_case() {
        let haystack = "hello wϕrld\0";
        let needle = "h\0";
        unsafe {
            let res = FIND__STRING(haystack.as_ptr(), needle.as_ptr());
            assert_eq!(res, 1)
        }
    }

    #[test]
    fn test_find_index_correct_edge_case2() {
        let haystack = "hello world\0";
        let needle = "d\0";
        unsafe {
            let res = FIND__STRING(haystack.as_ptr(), needle.as_ptr());
            assert_eq!(res, 11)
        }
    }

    #[test]
    fn test_find_index_correct_multibyte() {
        let haystack = "hello ϕϚϡϗ\0";
        let needle = "ϗ\0";
        unsafe {
            let res = FIND__STRING(haystack.as_ptr(), needle.as_ptr());
            assert_eq!(res, 10)
        }
    }

    #[test]
    fn test_left_ext_str() {
        let src = "ϕϚϡϗ hello\0";
        let len = 7;
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            LEFT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("ϕϚϡϗ he", string)
        }
    }

    #[test]
    fn test_left_ext_long_str() {
        let src = "     this is   a  very   long           sentence   with plenty  of    characters and weird  spacing.\0";
        let len = 85;
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            LEFT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!(
                "     this is   a  very   long           sentence   with plenty  of    characters and ",
                string
            )
        }
    }

    #[test]
    fn test_left_ext_str_w_escape_sequence() {
        let src = "ϕ\"Ϛ\"ϡϗ hello\0";
        let len = 6;
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            LEFT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("ϕ\"Ϛ\"ϡϗ", string)
        }
    }

    #[test]
    fn test_left_ext_str_edge_case() {
        let src = "ϕϚϡϗ hello\0";
        let len = 10;
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            LEFT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("ϕϚϡϗ hello", string)
        }
    }

    #[test]
    #[should_panic]
    fn test_left_ext_str_len_out_of_range() {
        let src = "hello\0 world";
        let len = 7;
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            LEFT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_right_ext_str() {
        let src = "ϕϚϡϗ hello\0";
        let len = 5;
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            RIGHT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("hello", string)
        }
    }

    #[test]
    fn test_right_ext_str_multi_byte() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let len = 8;
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            RIGHT_EXT__STRING(src.as_ptr(), len, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("xϗ wϕrld", string)
        }
    }

    #[test]
    fn test_mid_ext_str() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let len = 6;
        let start_index = 3;
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            MID_EXT__STRING(src.as_ptr(), len, start_index, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("ϡxϗ wϕ", string)
        }
    }

    #[test]
    fn test_mid_ext_str_edge_case() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let len = 11;
        let start_index = 1;
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            MID_EXT__STRING(src.as_ptr(), len, start_index, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("ϕϚϡxϗ wϕrld", string)
        }
    }

    #[test]
    #[should_panic]
    fn test_mid_ext_str_start_index_out_of_range() {
        let src = "hello world\0";
        let len = 5;
        let start_index = 12;
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe { MID_EXT__STRING(src.as_ptr(), len, start_index, dest.as_mut_ptr()) };
    }

    #[test]
    fn test_insert_ext_str() {
        let base = "ϕϚϡxϗ wϕrld\0";
        let insert = "brave new \0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            INSERT_EXT__STRING(base.as_ptr(), insert.as_ptr(), 6, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("ϕϚϡxϗ brave new wϕrld", string)
        }
    }

    #[test]
    fn test_insert_ext_str_insert_at_zero() {
        let base = "hello world\0";
        let insert = "ϕϚϡxϗ new \0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            INSERT_EXT__STRING(base.as_ptr(), insert.as_ptr(), 0, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("ϕϚϡxϗ new hello world", string)
        }
    }

    #[test]
    fn test_insert_ext_str_insert_at_end() {
        let base = "hello world\0";
        let insert = "ϕϚϡxϗ new \0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            INSERT_EXT__STRING(base.as_ptr(), insert.as_ptr(), (base.len() - 1) as i32, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("hello worldϕϚϡxϗ new ", string)
        }
    }

    #[test]
    #[should_panic]
    fn test_insert_ext_str_pos_out_of_range() {
        let base = "hello world\0";
        let insert = "brave new \0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            INSERT_EXT__STRING(base.as_ptr(), insert.as_ptr(), base.len() as i32, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_insert_ext_str_pos_negative() {
        let base = "hello world\0";
        let insert = "brave new \0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            INSERT_EXT__STRING(base.as_ptr(), insert.as_ptr(), -2, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_delete_ext_str() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 9, 3, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("ϕϚ", string)
        }
    }

    #[test]
    fn test_delete_ext_str_delete_all() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 11, 1, dest.as_mut_ptr());
            let c_str: &CStr = CStr::from_ptr(dest.as_mut_ptr() as *const _);
            let string: &str = c_str.to_str().unwrap();
            assert_eq!("", string)
        }
    }

    #[test]
    fn test_delete_ext_str_delete_last() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 1, 11, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("ϕϚϡxϗ wϕrl", string)
        }
    }

    #[test]
    fn test_delete_ext_str_delete_first() {
        let src = "ϕϚϡxϗ wϕrld\0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 1, 1, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("Ϛϡxϗ wϕrld", string)
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_str_too_many_del_chars() {
        let src = "hello world\0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 12, 1, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_str_pos_out_of_range_lower() {
        let src = "hello world\0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 11, 0, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_str_pos_out_of_range_upper() {
        let src = "hello world\0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            DELETE_EXT__STRING(src.as_ptr(), 11, 12, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_replace_ext_str_replace_at_beginning() {
        let base = "ϕϚϡxϗ wϕrld\0";
        let replacement = "brϡxϗ new \0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            REPLACE_EXT__STRING(base.as_ptr(), replacement.as_ptr(), 6, 1, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("brϡxϗ new wϕrld", string)
        }
    }

    #[test]
    fn test_replace_ext_str_replace_at_middle() {
        let base = "hellϕ wϕrld\0";
        let replacement = "brϡxϗ new\0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            REPLACE_EXT__STRING(base.as_ptr(), replacement.as_ptr(), 3, 5, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("hellbrϡxϗ newϕrld", string)
        }
    }

    #[test]
    fn test_replace_ext_str_replace_at_end() {
        let base = "hællø wørlÞ\0";
        let replacement = "aldø, how are you😀\0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            REPLACE_EXT__STRING(base.as_ptr(), replacement.as_ptr(), 4, 8, dest.as_mut_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("hællø waldø, how are you😀", string)
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_str_replace_too_many_chars() {
        let base = "hello world\0";
        let replacement = "aldo, how are you\0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            REPLACE_EXT__STRING(base.as_ptr(), replacement.as_ptr(), 12, 1, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_str_pos_out_of_bounds_lower() {
        let base = "hello world\0";
        let replacement = "aldo, how are you\0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            REPLACE_EXT__STRING(base.as_ptr(), replacement.as_ptr(), 8, 0, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_str_pos_out_of_bounds_upper() {
        let base = "hello world\0";
        let replacement = "aldo, how are you\0";
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            REPLACE_EXT__STRING(base.as_ptr(), replacement.as_ptr(), 8, 12, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_concat_str() {
        let argv = ["hællø wørlÞ\0".as_ptr(), "hello world\0".as_ptr(), "𝄞music\0".as_ptr()];
        unsafe {
            let mut arr = [0_u8; 2049];
            let dest = arr.as_mut_ptr();
            CONCAT__STRING(dest, argv.len() as i32, argv.as_ptr());
            let string = String::from_utf8_lossy(ptr_to_slice(dest));
            let result = string.trim_end_matches('\0');
            assert_eq!("hællø wørlÞhello world𝄞music", result)
        }
    }

    #[test]
    fn test_concat_ext_str() {
        let argv = ["hællø wørlÞ\0".as_ptr(), "hello world\0".as_ptr(), "𝄞music\0".as_ptr()];
        let argc = argv.len() as i32;
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            CONCAT_EXT__STRING(dest.as_mut_ptr(), argc, argv.as_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("hællø wørlÞhello world𝄞music", string)
        }
    }

    #[test]
    fn test_concat_ext_no_args() {
        let argv = [];
        let argc = argv.len() as i32;
        let mut dest: [u8; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            CONCAT_EXT__STRING(dest.as_mut_ptr(), argc, argv.as_ptr());
            let string = CStr::from_ptr(dest.as_ptr() as *const _).to_str().unwrap();
            assert_eq!("", string)
        }
    }
    #[test]
    fn test_greater_than_string_is_false_for_equal_strings() {
        let argv = ["hællø wørlÞ\0".as_ptr(), "hællø wørlÞ\0".as_ptr()];
        let argc = argv.len() as i32;
        unsafe { assert!(!__STRING_GREATER(argc, argv.as_ptr())) }
    }

    #[test]
    fn test_greater_than_string_is_true_for_decreasing_sequence() {
        let argv = ["zyxZabcdefghijklmn\0".as_ptr(), "zyxA\0".as_ptr(), "zyx\0".as_ptr()];
        let argc = argv.len() as i32;
        unsafe { assert!(__STRING_GREATER(argc, argv.as_ptr())) }
    }

    #[test]
    fn test_greater_than_string_is_false_for_increasing_sequence() {
        let argv = ["abc\0".as_ptr(), "bce\0".as_ptr(), "xyz\0".as_ptr()];
        let argc = argv.len() as i32;
        unsafe { assert!(!__STRING_GREATER(argc, argv.as_ptr())) }
    }

    #[test]
    fn test_greater_than_string_works_correctly_for_two_params() {
        let argv = ["zyxAabcdefghijklmn\0".as_ptr(), "zyxZ".as_ptr()];
        let argc = argv.len() as i32;
        unsafe { assert!(!__STRING_GREATER(argc, argv.as_ptr())) }
    }

    #[test]
    fn test_equal_string() {
        let argv = ["hællø wørlÞ\0".as_ptr(), "hællø wørlÞ\0".as_ptr()];
        let argc = argv.len() as i32;
        unsafe { assert!(__STRING_EQUAL(argc, argv.as_ptr())) }
    }

    #[test]
    fn test_equal_string_is_false_for_inequality() {
        let argv = [
            "hællø wørlÞabc\0".as_ptr(),
            "hællø wørlÞabc\0".as_ptr(),
            "hællø wørlÞabc\0".as_ptr(),
            "hællø wørlÞZZc\0".as_ptr(),
        ];
        let argc = argv.len() as i32;
        unsafe { assert!(!__STRING_EQUAL(argc, argv.as_ptr())) }
    }

    #[test]
    fn test_lesser_than_string() {
        let argv = ["hællø wørlÞabc\0".as_ptr(), "hællø wørlÞz\0".as_ptr()];
        let argc = argv.len() as i32;
        unsafe { assert!(__STRING_LESS(argc, argv.as_ptr())) }
    }

    #[test]
    fn test_lesser_than_string_is_false() {
        let argv = ["z\0".as_ptr(), "hællø wørlÞzbc\0".as_ptr()];
        let argc = argv.len() as i32;
        unsafe { assert!(!__STRING_LESS(argc, argv.as_ptr())) }
    }

    // -----------------------------------UTF16
    #[test]
    fn test_len_correct_utf16_character_count() {
        let src = "𝄞music𝄞 😀𝄞ϕϚϡϗ😀\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        unsafe {
            let res = LEN__WSTRING(src_ptr);
            assert_eq!(res, 15)
        }
    }

    #[test]
    fn test_find_wstring() {
        let base = "𝄞music𝄞 world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let find = "c𝄞\0".encode_utf16().collect::<Vec<u16>>();
        let find_ptr = find.as_slice().as_ptr();
        unsafe {
            let res = FIND__WSTRING(base_ptr, find_ptr);
            assert_eq!(6, res)
        }
    }

    #[test]
    fn test_find_wstring_no_match() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let find = "zzzzzz\0".encode_utf16().collect::<Vec<u16>>();
        let find_ptr = find.as_slice().as_ptr();
        unsafe {
            let res = FIND__WSTRING(base_ptr, find_ptr);
            assert_eq!(0, res)
        }
    }

    #[test]
    fn test_find_wstring_base_string_too_short() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let find = "hello world oachkatzlschwoaf\0".encode_utf16().collect::<Vec<u16>>();
        let find_ptr = find.as_slice().as_ptr();
        unsafe {
            let res = FIND__WSTRING(base_ptr, find_ptr);
            assert_eq!(0, res)
        }
    }

    #[test]
    fn test_left_ext_wstring() {
        let src = "𝄞musϗ😀ic world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            LEFT_EXT__WSTRING(src_ptr, 7, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("𝄞musϗ😀i", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_left_ext_wstring_len_out_of_range() {
        let src = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            LEFT_EXT__WSTRING(src_ptr, 14, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_right_ext_wstring() {
        let src = "hello 𝄞musϗ😀\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            RIGHT_EXT__WSTRING(src_ptr, 8, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));

            assert_eq!("o 𝄞musϗ😀", res)
        }
    }

    #[test]
    fn test_right_ext_wstring_zero_length_strings() {
        let src = "\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            RIGHT_EXT__WSTRING(src_ptr, 0, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));

            assert_eq!("", res)
        }
    }

    #[test]
    fn test_mid_ext_wstring() {
        let src = "𝄞muϗ😀 world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            MID_EXT__WSTRING(src_ptr, 5, 5, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("😀 wor", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_mid_ext_wstring_index_out_of_range() {
        let src = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            MID_EXT__WSTRING(src_ptr, 4, 12, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_insert_ext_wstring() {
        let base = "𝄞muϗ😀 world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let to_insert = "brave 𝄞muϗ😀 \0".encode_utf16().collect::<Vec<u16>>();
        let to_insert_ptr = to_insert.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            INSERT_EXT__WSTRING(base_ptr, to_insert_ptr, 6, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("𝄞muϗ😀 brave 𝄞muϗ😀 world", res)
        }
    }

    #[test]
    fn test_insert_ext_wstring_insert_at_zero() {
        let base = "hello 𝄞muϗ😀\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let to_insert = "𝄞muϗ😀 new \0".encode_utf16().collect::<Vec<u16>>();
        let to_insert_ptr = to_insert.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            INSERT_EXT__WSTRING(base_ptr, to_insert_ptr, 0, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("𝄞muϗ😀 new hello 𝄞muϗ😀", res)
        }
    }

    #[test]
    fn test_insert_ext_wstring_insert_at_end() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_ptr();
        let to_insert = "brave 𝄞muϗ😀 \0".encode_utf16().collect::<Vec<u16>>();
        let to_insert_ptr = to_insert.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            INSERT_EXT__WSTRING(base_ptr, to_insert_ptr, 11, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("hello worldbrave 𝄞muϗ😀 ", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_insert_ext_wstring_pos_out_of_range() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let to_insert = "brave new \0".encode_utf16().collect::<Vec<u16>>();
        let to_insert_ptr = to_insert.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            INSERT_EXT__WSTRING(base_ptr, to_insert_ptr, 12, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_delete_ext_wstring() {
        let src = "h𝄞muϗ w😀rld\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            DELETE_EXT__WSTRING(src_ptr, 5, 3, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("h𝄞😀rld", res)
        }
    }

    #[test]
    fn test_delete_ext_wstring_delete_all() {
        let src = "h𝄞muϗ w😀rld\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            DELETE_EXT__WSTRING(src_ptr, 11, 1, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));

            assert_eq!("", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_wstring_too_many_del_chars() {
        let src = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            DELETE_EXT__WSTRING(src_ptr, 10, 3, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_wstring_pos_out_of_range_lower() {
        let src = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            DELETE_EXT__WSTRING(src_ptr, 9, 0, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_delete_ext_wstring_pos_out_of_range_upper() {
        let src = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let src_ptr = src.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            DELETE_EXT__WSTRING(src_ptr, 9, 12, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_replace_ext_wstring_replace_at_beginning() {
        let base = "h𝄞muϗ w😀rld\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let replacement = "brave new \0".encode_utf16().collect::<Vec<u16>>();
        let replacement_ptr = replacement.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            REPLACE_EXT__WSTRING(base_ptr, replacement_ptr, 6, 1, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));

            assert_eq!("brave new w😀rld", res)
        }
    }

    #[test]
    fn test_replace_ext_wstring_replace_at_middle() {
        let base = "hello w😀rld𝄞\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let replacement = " is out of this \0".encode_utf16().collect::<Vec<u16>>();
        let replacement_ptr = replacement.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            REPLACE_EXT__WSTRING(base_ptr, replacement_ptr, 2, 5, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("hell is out of this w😀rld𝄞", res)
        }
    }

    #[test]
    fn test_replace_ext_wstring_replace_at_end() {
        let base = "hello w😀rld𝄞\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let replacement = "aldo, how are you? 😀\0".encode_utf16().collect::<Vec<u16>>();
        let replacement_ptr = replacement.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            REPLACE_EXT__WSTRING(base_ptr, replacement_ptr, 5, 8, dest.as_mut_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));

            assert_eq!("hello waldo, how are you? 😀", res)
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_wstring_replace_too_many_chars() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let replacement = " is out of this \0".encode_utf16().collect::<Vec<u16>>();
        let replacement_ptr = replacement.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            REPLACE_EXT__WSTRING(base_ptr, replacement_ptr, 12, 1, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_wstring_pos_out_of_bounds_lower() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let replacement = " is out of this \0".encode_utf16().collect::<Vec<u16>>();
        let replacement_ptr = replacement.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            REPLACE_EXT__WSTRING(base_ptr, replacement_ptr, 8, 0, dest.as_mut_ptr());
        }
    }

    #[test]
    #[should_panic]
    fn test_replace_ext_wstring_pos_out_of_bounds_upper() {
        let base = "hello world\0".encode_utf16().collect::<Vec<u16>>();
        let base_ptr = base.as_slice().as_ptr();
        let replacement = " is out of this \0".encode_utf16().collect::<Vec<u16>>();
        let replacement_ptr = replacement.as_slice().as_ptr();
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            REPLACE_EXT__WSTRING(base_ptr, replacement_ptr, 8, 12, dest.as_mut_ptr());
        }
    }

    #[test]
    fn test_concat_wstring() {
        let argvec: [Vec<u16>; 3] = [
            "hællø wørlÞ\0".encode_utf16().collect(),
            "hello world\0".encode_utf16().collect(),
            "𝄞music\0".encode_utf16().collect(),
        ];
        let mut argv: [*const u16; 3] = [std::ptr::null(); 3];
        for (i, arg) in argvec.iter().enumerate() {
            argv[i] = arg.as_ptr();
        }
        unsafe {
            let mut arr = [0_u16; 2049];
            let dest = arr.as_mut_ptr();
            CONCAT__WSTRING(dest, argv.len() as i32, argv.as_ptr());
            let string = String::from_utf16_lossy(ptr_to_slice(dest));
            let result = string.trim_end_matches('\0');
            assert_eq!("hællø wørlÞhello world𝄞music", result)
        }
    }

    #[test]
    fn test_concat_ext_wstring() {
        let argvec: [Vec<u16>; 3] = [
            "hællø wørlÞ\0".encode_utf16().collect(),
            "hello world\0".encode_utf16().collect(),
            "𝄞music\0".encode_utf16().collect(),
        ];
        let mut argv: [*const u16; 3] = [std::ptr::null(); 3];
        for (i, arg) in argvec.iter().enumerate() {
            argv[i] = arg.as_ptr();
        }
        let argc = argv.len() as i32;
        let mut dest: [u16; DEFAULT_STRING_SIZE] = [0; DEFAULT_STRING_SIZE];
        unsafe {
            CONCAT_EXT__WSTRING(dest.as_mut_ptr(), argc, argv.as_ptr());
            let res = String::from_utf16_lossy(std::slice::from_raw_parts(
                dest.as_ptr(),
                get_null_terminated_len(dest.as_ptr()),
            ));
            assert_eq!("hællø wørlÞhello world𝄞music", res)
        }
    }

    #[test]
    fn test_gt_wstring() {
        let argvec: [Vec<u16>; 3] = [
            "hællø wørlÞ\0".encode_utf16().collect(),
            "hello world\0".encode_utf16().collect(),
            "hel\0".encode_utf16().collect(),
        ];
        let mut argv: [*const u16; 3] = [std::ptr::null(); 3];
        for (i, arg) in argvec.iter().enumerate() {
            argv[i] = arg.as_ptr();
        }
        let argc = argv.len() as i32;
        unsafe { assert!(__WSTRING_GREATER(argc, argv.as_ptr())) }
    }

    #[test]
    fn test_eq_wstring() {
        let argvec: [Vec<u16>; 3] = [
            "hællø wørlÞ\0".encode_utf16().collect(),
            "hællø wørlÞ\0".encode_utf16().collect(),
            "hællø wørlÞ\0".encode_utf16().collect(),
        ];
        let mut argv: [*const u16; 3] = [std::ptr::null(); 3];
        for (i, arg) in argvec.iter().enumerate() {
            argv[i] = arg.as_ptr();
        }
        let argc = argv.len() as i32;
        unsafe { assert!(__WSTRING_EQUAL(argc, argv.as_ptr())) }
    }

    #[test]
    fn test_lt_wstring() {
        let argvec: [Vec<u16>; 3] = [
            "hello world\0".encode_utf16().collect(),
            "hællø wørlÞ\0".encode_utf16().collect(),
            "hællø wør𝄞Þ\0".encode_utf16().collect(),
        ];
        let mut argv: [*const u16; 3] = [std::ptr::null(); 3];
        for (i, arg) in argvec.iter().enumerate() {
            argv[i] = arg.as_ptr();
        }
        let argc = argv.len() as i32;
        unsafe { assert!(__WSTRING_LESS(argc, argv.as_ptr())) }
    }
}
