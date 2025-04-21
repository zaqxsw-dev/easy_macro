extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

/// A custom derive macro to implement a builder pattern for structs.
///
/// This macro generates setter methods for each field in the struct, allowing
/// for a fluent API to set values for the fields.
#[proc_macro_derive(EasyBuilder)]
pub fn easybuilder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let setters = if let syn::Data::Struct(data) = input.data {
        data.fields
            .iter()
            .map(|field| {
                let field_name = &field.ident;
                let setter_name = syn::Ident::new(
                    &format!("set_{}", field_name.as_ref().unwrap()),
                    field_name.span(),
                );
                let field_type = &field.ty;

                quote! {
                    impl #name {
                        pub fn #setter_name(&mut self, value: #field_type) -> &mut Self {
                            self.#field_name = value;
                            self
                        }
                    }
                }
            })
            .collect()
    } else {
        Vec::new()
    };

    let expanded = quote! {
        #(#setters)*
    };

    TokenStream::from(expanded)
}
