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
            fn read_value(&mut self) -> u16 {
                self.adc.blocking_read(&mut self.channel)
            }

            fn get_resolution(&self) -> u16 {
                /// STM32 boards have a resolution of 12 bits
                4096
            }

            fn get_reference_voltage(&self) -> f32 {
                self.v_ref
            }

            fn get_voltage(&mut self) -> f32 {
                let adc_reading = self.read_value() as f32;
                let resolution = self.get_resolution() as f32;
                let v_ref = self.get_reference_voltage();
                (adc_reading / resolution) * v_ref
            }
        }

        impl #impl_generics #name #ty_generics {
            /// Create a new instance of our ADC implementation for the STM32L476RG
            pub fn new(adc: Adc<'d, T>, channel: AnyAdcChannel<T>, v_ref: f32) -> Self {
                Self { adc, channel, v_ref }
            }
        }
    };
    gen.into()
}
