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

    0
}

/// .
/// Converts WCHAR to CHAR
///
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn WCHAR_TO_CHAR(input: u16) -> u8 {
    let u16_arr = [input];
    let mut res_iter =
        char::decode_utf16(u16_arr.into_iter()).map(|r| r.unwrap_or(std::char::REPLACEMENT_CHARACTER));
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
