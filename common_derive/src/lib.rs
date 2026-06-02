use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Path, parse_macro_input};

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

#[proc_macro_derive(ProtoMessage, attributes(proto))]
pub fn derive_proto_serialization(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let mut proto_type: Option<Path> = None;

    for attr in &input.attrs {
        if attr.path().is_ident("proto") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("type") {
                    let value = meta.value()?;

                    proto_type = Some(value.parse()?);

                    Ok(())
                } else {
                    Err(meta.error("unsupported proto attribute"))
                }
            })
            .unwrap();
        }
    }

    let proto_type = proto_type.expect("missing #[proto(type = ...)] attribute");

    let expanded = quote! {
        impl ProtoMessage for #name {
            type Proto = #proto_type;

            fn to_proto(&self) -> Self::Proto {
                todo!()
            }
        }

        impl From<#proto_type> for #name {
            fn from(value: #proto_type) -> Self {
                todo!()
            }
        }
    };

    TokenStream::from(expanded)
}
