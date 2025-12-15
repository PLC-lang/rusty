use crate::header_generator::{header_generator_c::GeneratedHeaderForC, symbol_helper::SymbolHelper};

impl SymbolHelper for GeneratedHeaderForC {
    fn get_reference_symbol(&self) -> String {
        String::from(C_REFERENCE_SYMBOL)
    }

    fn get_variadic_symbol(&self) -> String {
        String::from(C_VARIADIC_SYMBOL)
    }
}

// ------------------- //
// -- "C" Constants -- //

/// The constant value for the "c" reference symbol
const C_REFERENCE_SYMBOL: &str = "*";

/// The constant value for the "c" variadic symbol
const C_VARIADIC_SYMBOL: &str = "...";

// ------------------- //
