extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Generics};

#[proc_macro_derive(HypedAdc)]
pub fn hyped_adc_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hyped_adc(&ast)
}

fn impl_hyped_adc(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics: &Generics = &ast.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics HypedAdc for #name #ty_generics {
            /// Read a value from the ADC channel
            fn read_value(&mut self) -> u16 {
                self.adc.blocking_read(&mut self.channel)
            }
        }

        impl #impl_generics #name #ty_generics {
            /// Create a new instance of our ADC implementation for the STM32L476RG
            pub fn new(adc: Adc<'d, T>, channel: AnyAdcChannel<T>) -> Self {
                Self { adc, channel }
            }
        }
    };
    gen.into()
}
