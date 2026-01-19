//! Defines shift operations

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate left operation on bytes
pub fn ROL__BYTE(input: u8, n: u32) -> u8 {
    input.rotate_left(n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate left operation on word
pub fn ROL__WORD(input: u16, n: u32) -> u16 {
    input.rotate_left(n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate left operation on dword
pub fn ROL__DWORD(input: u32, n: u32) -> u32 {
    input.rotate_left(n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate left operation on lword
pub fn ROL__LWORD(input: u64, n: u32) -> u64 {
    input.rotate_left(n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate right operation on bytes
pub fn ROR__BYTE(input: u8, n: u32) -> u8 {
    input.rotate_right(n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate right operation on word
pub fn ROR__WORD(input: u16, n: u32) -> u16 {
    input.rotate_right(n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate right operation on dword
pub fn ROR__DWORD(input: u32, n: u32) -> u32 {
    input.rotate_right(n)
}

#[allow(non_snake_case)]
#[no_mangle]
/// Rotate right operation on lword
pub fn ROR__LWORD(input: u64, n: u32) -> u64 {
    input.rotate_right(n)
}
