extern crate proc_macro;

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

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

#[proc_macro_derive(GetAstId)]
pub fn derive_get_ast_id(input: TokenStream) -> TokenStream {
    // XXX: I think it is possible to make this derive macro more flexible (returning an AstId/requiring the id field)
    // really limits the use-cases. It might be possible to also make a getter for i.e. location by using attributes.
    let syn_item = parse_macro_input!(input as DeriveInput);
    // data is of type syn::Data
    // See https://doc.servo.org/syn/enum.Data.html
    let Data::Enum(enum_item) = syn_item.data else {
        // type not derivable -> return a compiler error
        return Error::new(Span::call_site().into(), "GetAstId is only implemented for the AstStatement-enum. Each variant must provide an id field")
        .to_compile_error()
        .into();
    };

    // get all the variant idents
    let variants = enum_item.variants.clone().into_iter().map(|v| v.ident);

    // also get a tokenstream of how the id field should be accessed for each variant-type.
    // XXX: this could also be hardcoded, but I fear I won't remember this if we ever need to extend/change it
    let fields = enum_item.variants.into_iter().map(|v| match v.fields {
        syn::Fields::Named(_) => quote!({id, ..} => *id,),
        syn::Fields::Unnamed(_) => {
            unimplemented!("currently no variants of this kind in AstStatement")
        }
        syn::Fields::Unit => unimplemented!("currently no variants of this kind in AstStatement"),
    });

    // XXX: providing an iterator with the name (which stays the same for all variants...) for each variant was the only
    // way I got this to compile (for now)
    let name = &syn_item.ident;
    let mut names = vec![];
    for _ in 0..variants.len() {
        names.push(name);
    }

    // paste the get_id implementation
    quote! {
    impl #name {
        pub fn get_id(&self) -> AstId {
            match self {
                // all enum variants must be named variants and have an ID field..
                #(#names::#variants #fields)*
            }
        }
    }}
    .into()
}
