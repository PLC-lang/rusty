pub mod data_type_generator;
pub mod date_time_util;
pub mod expression_generator;
pub mod llvm;
pub mod pou_generator;
pub mod statement_generator;
pub mod variable_generator;

// see:
// - https://llvm.org/docs/LangRef.html#data-layout
// - https://llvm.org/doxygen/NVPTXBaseInfo_8h_source.html
pub const ADDRESS_SPACE_GENERIC: u16 = 0;
pub const ADDRESS_SPACE_GLOBAL: u16 = 1;
pub const ADDRESS_SPACE_CONST: u16 = 4;
