extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Generics};

#[proc_macro_derive(HypedUart)]
pub fn hyped_uart_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hyped_uart(&ast)
}

fn impl_hyped_uart(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics: &Generics = &ast.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics HypedUart for #name #ty_generics {

        }

        impl #impl_generics #name #ty_generics {
            pub fn new() -> Self {
                Self {}
            }
        }
    };
    gen.into()
}
