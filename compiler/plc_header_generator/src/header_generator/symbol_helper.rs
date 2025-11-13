mod symbol_helper_c;

pub trait SymbolHelper {
    fn get_reference_symbol(&self) -> String;
    fn get_variadic_symbol(&self) -> String;
}
