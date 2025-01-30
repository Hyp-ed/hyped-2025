extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HypedGpioInputPin)]
pub fn hyped_gpio_input_pin_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hyped_gpio_input_pin(&ast)
}

fn impl_hyped_gpio_input_pin(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HypedGpioInputPin for #name {
            fn is_high(&mut self) -> bool {
                self.pin.is_high()
            }
        }

        impl #name {
            /// Create a new instance of our GPIO implementation for the STM32L476RG
            pub fn new(pin: Input<'static>) -> Self {
                Self { pin }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(HypedGpioOutputPin)]
pub fn hyped_gpio_output_pin_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hyped_gpio_output_pin(&ast)
}

fn impl_hyped_gpio_output_pin(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HypedGpioOutputPin for #name {
            fn set_high(&mut self) {
                self.pin.set_high()
            }

            fn set_low(&mut self) {
                self.pin.set_low()
            }
        }

        impl #name {
            /// Create a new instance of our GPIO implementation for the STM32L476RG
            pub fn new(pin: Output<'static>) -> Self {
                Self { pin }
            }
        }
    };
    gen.into()
}
