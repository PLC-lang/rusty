extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Validators)]
pub fn derive_validators_fn(input: TokenStream) -> TokenStream {
    // parse the input stream into rust AST
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    // build the impl which is lowered, analyzed and then generated
    let generated = quote! {
        impl Validators for #name {
            fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
                self.diagnostics.push(diagnostic);
            }

            fn take_diagnostics(&mut self) -> Vec<Diagnostic> {
                std::mem::take(&mut self.diagnostics)
            }
        }
    };
    // re-export generated rust code as token stream
    TokenStream::from(generated)
}
