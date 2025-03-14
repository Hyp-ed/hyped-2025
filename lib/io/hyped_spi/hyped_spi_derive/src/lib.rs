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
    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics HypedSpi for #name #ty_generics{
            fn read(&mut self, words: &mut [u8]) -> Result<(), SpiError> {
                match self.spi.blocking_read(words) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(spi_error_from_error(e)),
                }
            }

            fn write(&mut self, words: &[u8]) -> Result<(), SpiError> {
                let binding = words.iter().copied().collect::<Vec<u8, 64>>();
                let new_words = binding.as_slice();
                match self.spi.blocking_write(new_words) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(spi_error_from_error(e)),

                }
            }

            fn transfer_in_place(&mut self, data: &mut [u8]) -> Result<(), SpiError> {
                match self.spi.blocking_transfer_in_place(data) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(spi_error_from_error(e)),

                }
            }
        }

        impl #impl_generics #name #ty_generics {
            pub fn new(spi: Spi<'static, Blocking>) -> Self {
                Self { spi }
            }
        }

        fn spi_error_from_error(e: spi::Error) -> SpiError {
            match e {
                spi::Error::Framing => SpiError::Framing,
                spi::Error::Crc => SpiError::Crc,
                spi::Error::ModeFault => SpiError::ModeFault,
                spi::Error::Overrun => SpiError::Overrun,
            }
        }
    };
    gen.into()
}
