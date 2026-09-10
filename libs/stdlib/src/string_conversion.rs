use std::char::DecodeUtf16Error;

use crate::string_functions::{CharsDecoder, CharsEncoder, EncodedCharsIter, STRING_RESULT_LEN};

/// .
/// Converts WSTRING to STRING
/// Unpaired surrogates become U+FFFD; output is truncated at the
/// result-buffer capacity
///
/// # Safety
///
/// Works on string pointer conversion, inherently unsafe
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn WSTRING_TO_STRING_EXT(src: *const u16, dest: *mut u8) -> i32 {
    let mut dest = dest;
    EncodedCharsIter::decode(src)
        .map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER))
        .encode_bounded(&mut dest, STRING_RESULT_LEN);

    0
}

/// .
/// Converts STRING to WSTRING
/// Invalid UTF-8 bytes become U+FFFD; output is truncated at the
/// result-buffer capacity
///
/// # Safety
///
/// Works on string pointer conversion, inherently unsafe
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_WSTRING_EXT(src: *const u8, dest: *mut u16) -> i32 {
    let mut dest = dest;
    // Wrapping each char in Ok selects the UTF-16 encoder; the decode itself is lossy.
    EncodedCharsIter::decode(src)
        .map(Ok::<char, DecodeUtf16Error>)
        .encode_bounded(&mut dest, STRING_RESULT_LEN);

    0
}

/// .
/// Converts WCHAR to CHAR
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn WCHAR_TO_CHAR(input: u16) -> u8 {
    let u16_arr = [input];
    let mut res_iter = char::decode_utf16(u16_arr).map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER));
    let mut res_arr = [u8::MAX; 80];
    if let Some(res) = res_iter.next() {
        if res_iter.next().is_none() {
            res.encode_utf8(&mut res_arr);
        }
    }
    res_arr[0]
}

/// .
/// Converts CHAR to WCHAR
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn CHAR_TO_WCHAR(input: u8) -> u16 {
    let res: char = input.into();
    let mut arr = [u16::MAX; 2];
    res.encode_utf16(&mut arr);
    arr[0]
}

///.
/// Converts STRING to CHAR
/// # Safety
/// uses raw pointer
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_CHAR(input: *const u8) -> u8 {
    *input
}

///.
/// Converts WSTRING to WCHAR
/// # Safety
/// uses raw pointer
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn WSTRING_TO_WCHAR(input: *const u16) -> u16 {
    *input
}

///.
/// Converts CHAR to STRING
/// # Safety
/// uses raw pointer
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn CHAR_TO_STRING(dest: *mut u8, input: u8) -> i32 {
    *dest = input;
    // Do not rely on the destination being zero-initialized.
    *dest.add(1) = 0;
    0
}

///.
/// Converts WCHAR to WSTRING
/// # Safety
/// uses raw pointer
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn WCHAR_TO_WSTRING(dest: *mut u16, input: u16) -> i32 {
    *dest = input;
    // Do not rely on the destination being zero-initialized.
    *dest.add(1) = 0;
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn conversions_terminate_results_in_dirty_buffers() {
        // Result buffers are not guaranteed to be zeroed; every writer must
        // terminate its own output instead of relying on zeroed memory.
        let mut dest8 = [0xAA_u8; 81];
        unsafe { CHAR_TO_STRING(dest8.as_mut_ptr(), b'x') };
        assert_eq!(dest8[..2], [b'x', 0]);

        let mut dest16 = [0xAAAA_u16; 81];
        unsafe { WCHAR_TO_WSTRING(dest16.as_mut_ptr(), u16::from('x' as u8)) };
        assert_eq!(dest16[..2], ['x' as u16, 0]);

        dest16.fill(0xAAAA);
        unsafe { STRING_TO_WSTRING_EXT("ab\0".as_ptr(), dest16.as_mut_ptr()) };
        assert_eq!(dest16[..3], ['a' as u16, 'b' as u16, 0]);
    }

    #[test]
    fn string_to_wstring_replaces_invalid_bytes() {
        let mut dest = [0xAAAA_u16; 81];
        unsafe { STRING_TO_WSTRING_EXT(b"a\xFFb\0".as_ptr(), dest.as_mut_ptr()) };

        // converting 'a<0xFF>b' must yield "a<U+FFFD>b": the invalid byte becomes U+FFFD
        assert_eq!(dest[..4], [0x61, 0xFFFD, 0x62, 0]);
    }

    #[test]
    fn wstring_to_string_truncates_at_result_capacity() {
        // 2048 words of 3-UTF-8-byte characters would inflate to 6144 bytes;
        // the result must truncate at the capacity on a character boundary
        let mut src = [0x20AC_u16; 2049];
        src[2048] = 0;
        let mut dest = [0xAA_u8; 8192];
        unsafe { WSTRING_TO_STRING_EXT(src.as_ptr(), dest.as_mut_ptr()) };
        let written = dest.iter().position(|&byte| byte == 0).unwrap();
        assert_eq!(written, 682 * 3);
        assert!(dest[written + 1..].iter().all(|&byte| byte == 0xAA));
    }
}
