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
                let mut binding = Vec::<u8, 64>::new();
                let new_words = binding.as_mut_slice();
                match self.spi.blocking_read(new_words) {
                    Ok(_) => {
                        for (i, word) in new_words.iter().enumerate() {
                            words[i] = *word;
                        }
                        Ok(())
                    }
                    Err(e) => Err(match e {
                        spi::Error::Framing => SpiError::Framing,
                        spi::Error::Crc => SpiError::Crc,
                        spi::Error::ModeFault => SpiError::ModeFault,
                        spi::Error::Overrun => SpiError::Overrun,
                    }),
                }
            }

            fn write(&mut self, words: &[u8]) -> Result<(), SpiError> {
                let binding = words.iter().map(|word| *word).collect::<Vec<u8, 64>>();
                let new_words = binding.as_slice();
                match self.spi.blocking_write(new_words) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(match e {
                        spi::Error::Framing => SpiError::Framing,
                        spi::Error::Crc => SpiError::Crc,
                        spi::Error::ModeFault => SpiError::ModeFault,
                        spi::Error::Overrun => SpiError::Overrun,
                    }),
                }
            }

            fn transfer_in_place(&mut self, data: &mut [u8]) -> Result<(), SpiError> {
                let mut binding = data.iter().map(|word| *word).collect::<Vec<u8, 64>>();
                let new_words = binding.as_mut_slice();
                match self.spi.blocking_transfer_in_place(new_words) {
                    Ok(_) => {
                        for (i, word) in new_words.iter().enumerate() {
                            data[i] = *word;
                        }
                        Ok(())
                    }
                    Err(e) => Err(match e {
                        spi::Error::Framing => SpiError::Framing,
                        spi::Error::Crc => SpiError::Crc,
                        spi::Error::ModeFault => SpiError::ModeFault,
                        spi::Error::Overrun => SpiError::Overrun,
                    }),
                }
            }
        }

        impl #impl_generics #name #ty_generics {
            pub fn new(spi: Spi<'static, Blocking>) -> Self {
                Self { spi }
            }
        }

    };
    gen.into()
}
