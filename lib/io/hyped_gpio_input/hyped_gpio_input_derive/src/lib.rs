extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HypedGpioInput)]
pub fn hyped_gpio_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hyped_gpio_input(&ast)
}

fn impl_hyped_gpio_input(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HypedGpioInput for #name {
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
