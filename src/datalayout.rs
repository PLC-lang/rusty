use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct DataLayout {
    pub i1: Offset,
    pub i8: Offset,
    pub i16: Offset,
    pub i32: Offset,
    pub i64: Offset,
    pub f32: Offset,
    pub f64: Offset,
    pub p64: Offset,
    pub v64: Offset,
    pub v128: Offset,
    pub aggregate: Offset,
}

impl Default for DataLayout {
    fn default() -> Self {
        Self {
            i1: Offset::from_bits(8),
            i8: Offset::from_bits(8),
            i16: Offset::from_bits(16),
            i32: Offset::from_bits(32),
            i64: Offset::from_bits(64), //Using 64bit default alignment, if we need to support 32bit
            //this has to be adjusted
            f32: Offset::from_bits(32),
            f64: Offset::from_bits(64),
            p64: Offset::from_bits(64),
            v64: Offset::from_bits(64),
            v128: Offset::from_bits(128),
            aggregate: Offset::from_bits(64),
        }
    }
}

/// An offset, used to represent sizes, alignments, and offsets
#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd)]
pub struct Offset(u32);

impl Add for Offset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Offset(self.0 + rhs.0)
    }
}

impl AddAssign for Offset {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Offset {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Offset(self.0 - rhs.0)
    }
}

impl Mul for Offset {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Offset(self.0 * rhs.0)
    }
}

impl Div for Offset {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Offset(self.0 / rhs.0)
    }
}

impl Offset {
    pub fn from_bits(value: u32) -> Self {
        Offset(value / 8)
    }

    pub fn new(value: u32) -> Self {
        Offset(value)
    }

    pub fn align_to(self, align: Self) -> Self {
        let align = align.bytes() - 1;
        Offset((self.0 + align) & !align)
    }

    pub fn bytes(&self) -> u32 {
        self.0
    }

    pub fn bits(&self) -> u32 {
        self.bytes() * 8
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::tests::index;

    use super::*;

    #[test]
    fn i8_align() {
        let i8_size = Offset::new(1);

        //Align to 1 byte
        assert_eq!(i8_size.align_to(Offset::new(1)), i8_size);
        //Align to 2 bytes
        assert_eq!(i8_size.align_to(Offset::new(2)), Offset::new(2));
        //Align to 4 bytes
        assert_eq!(i8_size.align_to(Offset::new(4)), Offset::new(4));
        //Align to 8 bytes
        assert_eq!(i8_size.align_to(Offset::new(8)), Offset::new(8));
    }

    #[test]
    fn i16_align() {
        let i16_size = Offset::new(2);

        //Align to 1 byte
        assert_eq!(i16_size.align_to(Offset::new(1)), i16_size);
        //Align to 2 bytes
        assert_eq!(i16_size.align_to(Offset::new(2)), i16_size);
        //Align to 4 bytes
        assert_eq!(i16_size.align_to(Offset::new(4)), Offset::new(4));
        //Align to 8 bytes
        assert_eq!(i16_size.align_to(Offset::new(8)), Offset::new(8));
    }

    #[test]
    fn i32_align() {
        let i32_size = Offset::new(4);

        //Align to 1 byte
        assert_eq!(i32_size.align_to(Offset::new(1)), i32_size);
        //Align to 2 bytes
        assert_eq!(i32_size.align_to(Offset::new(2)), i32_size);
        //Align to 4 bytes
        assert_eq!(i32_size.align_to(Offset::new(4)), i32_size);
        //Align to 8 bytes
        assert_eq!(i32_size.align_to(Offset::new(8)), Offset::new(8));
    }

    #[test]
    fn i64_align() {
        let i64_size = Offset::new(8);

        //Align to 1 byte
        assert_eq!(i64_size.align_to(Offset::new(1)), i64_size);
        //Align to 2 bytes
        assert_eq!(i64_size.align_to(Offset::new(2)), i64_size);
        //Align to 4 bytes
        assert_eq!(i64_size.align_to(Offset::new(4)), i64_size);
        //Align to 8 bytes
        assert_eq!(i64_size.align_to(Offset::new(8)), i64_size);
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

        let struct_type = index
            .get_effective_type_by_name("MyStruct")
            .unwrap()
            .get_type_information();
        // And the struct size takes the alignment into account
        assert_eq!(struct_type.get_size(&index).bits(), 192);
        assert_eq!(struct_type.get_alignment(&index), Offset::new(8)) //Struct alignment is 64 by default
    }
}
