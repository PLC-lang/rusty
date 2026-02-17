mod symbol_helper_c;

pub trait SymbolHelper {
    /// Returns the reference symbol for the defined language
    fn get_reference_symbol(&self) -> String;

    /// Returns the variadic symbol for the defined language
    fn get_variadic_symbol(&self) -> String;
}
