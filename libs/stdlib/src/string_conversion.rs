use crate::string_functions::{CharsDecoder, CharsEncoder, EncodedCharsIter};

/// .
/// Converts WSTRING to STRING
/// Limited by a return type of 80 charachters
///
/// # Safety
///
/// Works on string pointer conversion, inherently unsafe
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn WSTRING_TO_STRING_EXT(src: *const u16, dest: *mut u8) -> i32 {
    let mut dest = dest;
    EncodedCharsIter::decode(src).map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER)).encode(&mut dest);

    0
}

/// .
/// Converts STRING to WSTRING
/// Limited by a return type of 80 charachters
///
/// # Safety
///
/// Works on string pointer conversion, inherently unsafe
///
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn STRING_TO_WSTRING_EXT(src: *const u8, dest: *mut u16) -> i32 {
    let mut dest = dest;
    let mut buffer = [0_u16; 2];
    for char in EncodedCharsIter::decode(src) {
        let slice = char.encode_utf16(&mut buffer);
        for word in slice {
            *dest = *word;
            dest = dest.add(1);
        }
    }
    // Do not rely on the destination being zero-initialized.
    *dest = 0;

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
}
