use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct DataLayout {
    pub i1: Byte,
    pub i8: Byte,
    pub i16: Byte,
    pub i32: Byte,
    pub i64: Byte,
    pub f32: Byte,
    pub f64: Byte,
    pub p64: Byte,
    pub v64: Byte,
    pub v128: Byte,
    pub aggregate: Byte,
}

impl Default for DataLayout {
    fn default() -> Self {
        Self {
            i1: Byte::from_bits(8),
            i8: Byte::from_bits(8),
            i16: Byte::from_bits(16),
            i32: Byte::from_bits(32),
            i64: Byte::from_bits(64), //Using 64bit default alignment, if we need to support 32bit
            //this has to be adjusted
            f32: Byte::from_bits(32),
            f64: Byte::from_bits(64),
            p64: Byte::from_bits(64),
            v64: Byte::from_bits(64),
            v128: Byte::from_bits(128),
            aggregate: Byte::from_bits(64),
        }
    }
}

/// A Byte unit, used to represent sizes, alignments, and offsets
#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd)]
pub struct Byte(u32);

impl Add for Byte {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Byte(self.0 + rhs.0)
    }
}

impl AddAssign for Byte {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Byte {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Byte(self.0 - rhs.0)
    }
}

impl Mul for Byte {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Byte(self.0 * rhs.0)
    }
}

impl Div for Byte {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Byte(self.0 / rhs.0)
    }
}

impl Byte {
    pub fn from_bits(value: u32) -> Self {
        Byte(value / 8)
    }

    pub fn new(value: u32) -> Self {
        Byte(value)
    }

    pub fn align_to(self, align: Self) -> Self {
        let align = align.bytes() - 1;
        Byte((self.0 + align) & !align)
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
        let i8_size = Byte::new(1);

        //Align to 1 byte
        assert_eq!(i8_size.align_to(Byte::new(1)), i8_size);
        //Align to 2 bytes
        assert_eq!(i8_size.align_to(Byte::new(2)), Byte::new(2));
        //Align to 4 bytes
        assert_eq!(i8_size.align_to(Byte::new(4)), Byte::new(4));
        //Align to 8 bytes
        assert_eq!(i8_size.align_to(Byte::new(8)), Byte::new(8));
    }

    #[test]
    fn i16_align() {
        let i16_size = Byte::new(2);

        //Align to 1 byte
        assert_eq!(i16_size.align_to(Byte::new(1)), i16_size);
        //Align to 2 bytes
        assert_eq!(i16_size.align_to(Byte::new(2)), i16_size);
        //Align to 4 bytes
        assert_eq!(i16_size.align_to(Byte::new(4)), Byte::new(4));
        //Align to 8 bytes
        assert_eq!(i16_size.align_to(Byte::new(8)), Byte::new(8));
    }

    #[test]
    fn i32_align() {
        let i32_size = Byte::new(4);

        //Align to 1 byte
        assert_eq!(i32_size.align_to(Byte::new(1)), i32_size);
        //Align to 2 bytes
        assert_eq!(i32_size.align_to(Byte::new(2)), i32_size);
        //Align to 4 bytes
        assert_eq!(i32_size.align_to(Byte::new(4)), i32_size);
        //Align to 8 bytes
        assert_eq!(i32_size.align_to(Byte::new(8)), Byte::new(8));
    }

    #[test]
    fn i64_align() {
        let i64_size = Byte::new(8);

        //Align to 1 byte
        assert_eq!(i64_size.align_to(Byte::new(1)), i64_size);
        //Align to 2 bytes
        assert_eq!(i64_size.align_to(Byte::new(2)), i64_size);
        //Align to 4 bytes
        assert_eq!(i64_size.align_to(Byte::new(4)), i64_size);
        //Align to 8 bytes
        assert_eq!(i64_size.align_to(Byte::new(8)), i64_size);
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
        assert_eq!(struct_type.get_alignment(&index), Byte::new(8)) //Struct alignment is 64 by default
    }
}
