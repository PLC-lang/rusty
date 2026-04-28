use std::ops::{Add, AddAssign};

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DataLayout {
    pub i1: Bytes,
    pub i8: Bytes,
    pub i16: Bytes,
    pub i32: Bytes,
    pub i64: Bytes,
    pub f32: Bytes,
    pub f64: Bytes,
    pub p64: Bytes,
    pub v64: Bytes,
    pub v128: Bytes,
    pub aggregate: Bytes,
}

impl Default for DataLayout {
    fn default() -> Self {
        Self {
            i1: Bytes::from_bits(8),
            i8: Bytes::from_bits(8),
            i16: Bytes::from_bits(16),
            i32: Bytes::from_bits(32),
            i64: Bytes::from_bits(64), //Using 64bit default alignment, if we need to support 32bit
            //this has to be adjusted
            f32: Bytes::from_bits(32),
            f64: Bytes::from_bits(64),
            p64: Bytes::from_bits(64),
            v64: Bytes::from_bits(64),
            v128: Bytes::from_bits(128),
            aggregate: Bytes::from_bits(64),
        }
    }
}

/// An representation of a Byte unit, used to represent sizes, and alignments
#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Default, Serialize, Deserialize)]
pub struct Bytes(u32);

impl Add for Bytes {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Bytes(self.0 + rhs.0)
    }
}

impl AddAssign for Bytes {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Bytes {
    pub fn from_bits(value: u32) -> Self {
        Bytes(value / 8)
    }

    pub fn new(value: u32) -> Self {
        Bytes(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn bits(&self) -> u32 {
        self.value() * 8
    }
}

impl From<MemoryLocation> for Bytes {
    fn from(offset: MemoryLocation) -> Self {
        offset.0
    }
}

/// Represents an offset in byte (in memory)
#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd)]
pub struct MemoryLocation(Bytes);

impl MemoryLocation {
    pub fn new(value: u32) -> Self {
        MemoryLocation(Bytes(value))
    }

    // see https://en.wikipedia.org/wiki/Data_structure_alignment#Computing_padding
    pub fn align_to(self, align: Bytes) -> Self {
        let align = align.value().saturating_sub(1);
        MemoryLocation::new((self.value() + align) & !align)
    }

    pub fn value(&self) -> u32 {
        self.0.value()
    }

    pub fn bits(&self) -> u32 {
        self.0.bits()
    }
}

impl AddAssign<Bytes> for MemoryLocation {
    fn add_assign(&mut self, rhs: Bytes) {
        self.0 += rhs
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::tests::index;

    use super::*;

    #[test]
    fn i8_align() {
        let i8_size = MemoryLocation::new(1);

        //Align to 1 byte
        assert_eq!(i8_size.align_to(Bytes::new(1)), i8_size);
        //Align to 2 bytes
        assert_eq!(i8_size.align_to(Bytes::new(2)), MemoryLocation::new(2));
        //Align to 4 bytes
        assert_eq!(i8_size.align_to(Bytes::new(4)), MemoryLocation::new(4));
        //Align to 8 bytes
        assert_eq!(i8_size.align_to(Bytes::new(8)), MemoryLocation::new(8));
    }

    #[test]
    fn i16_align() {
        let i16_size = MemoryLocation::new(2);

        //Align to 1 byte
        assert_eq!(i16_size.align_to(Bytes::new(1)), i16_size);
        //Align to 2 bytes
        assert_eq!(i16_size.align_to(Bytes::new(2)), i16_size);
        //Align to 4 bytes
        assert_eq!(i16_size.align_to(Bytes::new(4)), MemoryLocation::new(4));
        //Align to 8 bytes
        assert_eq!(i16_size.align_to(Bytes::new(8)), MemoryLocation::new(8));
    }

    #[test]
    fn i32_align() {
        let i32_size = MemoryLocation::new(4);

        //Align to 1 byte
        assert_eq!(i32_size.align_to(Bytes::new(1)), i32_size);
        //Align to 2 bytes
        assert_eq!(i32_size.align_to(Bytes::new(2)), i32_size);
        //Align to 4 bytes
        assert_eq!(i32_size.align_to(Bytes::new(4)), i32_size);
        //Align to 8 bytes
        assert_eq!(i32_size.align_to(Bytes::new(8)), MemoryLocation::new(8));
    }

    #[test]
    fn i64_align() {
        let i64_size = MemoryLocation::new(8);

        //Align to 1 byte
        assert_eq!(i64_size.align_to(Bytes::new(1)), i64_size);
        //Align to 2 bytes
        assert_eq!(i64_size.align_to(Bytes::new(2)), i64_size);
        //Align to 4 bytes
        assert_eq!(i64_size.align_to(Bytes::new(4)), i64_size);
        //Align to 8 bytes
        assert_eq!(i64_size.align_to(Bytes::new(8)), i64_size);
    }

    #[test]
    fn struct_with_default_alignment() {
        //Given the default data layout
        //When a struct with different member sizes is created
        let (_, index) = index(
            "
        TYPE MyStruct : STRUCT
            a : BYTE; //8bit - offset 0 -> 8
            b : DWORD; //32bit - offset 32 -> 64
            c : WORD; //16bit - offset 64 -> 80
            d : LWORD; //64bit - offset 128 -> 192
        END_STRUCT
        END_TYPE
        ",
        );

        let struct_type = index.get_effective_type_by_name("MyStruct").unwrap().get_type_information();
        // The size is 120 bytes and the struct is not aligned
        assert_eq!(struct_type.get_size(&index).unwrap().bits(), 120);
    }
}
