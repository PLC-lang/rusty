#[derive(Copy, Clone, Debug)]
pub struct DataLayout {
    pub i1: Align,
    pub i8: Align,
    pub i16: Align,
    pub i32: Align,
    pub i64: Align,
    pub f32: Align,
    pub f64: Align,
    pub p64: Align,
    pub v64: Align,
    pub v128: Align,
    pub aggregate: Align,
}

impl Default for DataLayout {
    fn default() -> Self {
        Self {
            i1: Align::from_bits(8),
            i8: Align::from_bits(8),
            i16: Align::from_bits(16),
            i32: Align::from_bits(32),
            i64: Align::from_bits(64), //Using 64bit default alignment, if we need to support 32bit
                                       //this has to be adjusted
            f32: Align::from_bits(32),
            f64: Align::from_bits(64),
            p64: Align::from_bits(64),
            v64: Align::from_bits(64),
            v128: Align::from_bits(128),
            aggregate: Align::from_bits(64),
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd)]
pub struct Align(u32);

impl Align {
    fn from_bits<T: TryInto<u32>>(value: T) -> Self {
        let align = value.try_into().ok().unwrap() / 8;
        Self::from_bytes(align)
    }

    fn from_bytes<T: TryInto<u32>>(value: T) -> Self {
        let align = value.try_into().ok().unwrap();
        if !align.is_power_of_two() {
            panic!("Align {align} is not a power of two")
        }
        Align(align)
    }

    pub fn bytes(&self) -> u32 {
        dbg!(self.0)
    }

    pub fn bits(&self) -> u32 {
        dbg!(self.0 * 8)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Size {
    value: u32,
}

impl Size {
    pub fn from_bits(value: u32) -> Self {
        Size::from_bytes(value / 8)
    }

    pub fn from_bytes(value: u32) -> Self {
        Size { value }
    }

    pub fn align_to(self, align: Align) -> Self {
        let align = align.bytes() - 1;
        Size {
            value: (self.value + align) & !align,
        }
    }

    pub fn bytes(&self) -> u32 {
        self.value
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
        let i8_size = Size::from_bytes(1);

        //Align to 1 byte
        assert_eq!(i8_size.align_to(Align::from_bytes(1)), i8_size);
        //Align to 2 bytes
        assert_eq!(i8_size.align_to(Align::from_bytes(2)), Size::from_bytes(2));
        //Align to 4 bytes
        assert_eq!(i8_size.align_to(Align::from_bytes(4)), Size::from_bytes(4));
        //Align to 8 bytes
        assert_eq!(i8_size.align_to(Align::from_bytes(8)), Size::from_bytes(8));
    }

    #[test]
    fn i16_align() {
        let i16_size = Size::from_bytes(2);

        //Align to 1 byte
        assert_eq!(i16_size.align_to(Align::from_bytes(1)), i16_size);
        //Align to 2 bytes
        assert_eq!(i16_size.align_to(Align::from_bytes(2)), i16_size);
        //Align to 4 bytes
        assert_eq!(i16_size.align_to(Align::from_bytes(4)), Size::from_bytes(4));
        //Align to 8 bytes
        assert_eq!(i16_size.align_to(Align::from_bytes(8)), Size::from_bytes(8));
    }

    #[test]
    fn i32_align() {
        let i32_size = Size::from_bytes(4);

        //Align to 1 byte
        assert_eq!(i32_size.align_to(Align::from_bytes(1)), i32_size);
        //Align to 2 bytes
        assert_eq!(i32_size.align_to(Align::from_bytes(2)), i32_size);
        //Align to 4 bytes
        assert_eq!(i32_size.align_to(Align::from_bytes(4)), i32_size);
        //Align to 8 bytes
        assert_eq!(i32_size.align_to(Align::from_bytes(8)), Size::from_bytes(8));
    }

    #[test]
    fn i64_align() {
        let i64_size = Size::from_bytes(8);

        //Align to 1 byte
        assert_eq!(i64_size.align_to(Align::from_bytes(1)), i64_size);
        //Align to 2 bytes
        assert_eq!(i64_size.align_to(Align::from_bytes(2)), i64_size);
        //Align to 4 bytes
        assert_eq!(i64_size.align_to(Align::from_bytes(4)), i64_size);
        //Align to 8 bytes
        assert_eq!(i64_size.align_to(Align::from_bytes(8)), i64_size);
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
            .get_effective_type("MyStruct")
            .unwrap()
            .get_type_information();
        // And the struct size takes the alignment into account
        assert_eq!(struct_type.get_size(&index).bits(), 192);
        assert_eq!(struct_type.get_alignment(&index), Align::from_bytes(8)) //Struct alignment is 64 by default
    }
}
