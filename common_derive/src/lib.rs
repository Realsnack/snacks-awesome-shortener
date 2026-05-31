use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(TypeString)]
pub fn derive_type_to_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl TypeString for #name {
            fn type_as_string(&self) -> String {
                ::std::any::type_name_of_val(&self).split("::").last().unwrap().to_string()
            }
        }
    };
    TokenStream::from(expanded)
}
