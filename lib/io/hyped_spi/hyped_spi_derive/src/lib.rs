extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Generics};

#[proc_macro_derive(HypedSpi)]
pub fn hyped_spi_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hyped_spi(&ast)
}

fn impl_hyped_spi(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics: &Generics = &ast.generics;
    let (_impl_generics, _ty_generics, _) = generics.split_for_impl();
    let gen = quote! {};
    gen.into()
}
