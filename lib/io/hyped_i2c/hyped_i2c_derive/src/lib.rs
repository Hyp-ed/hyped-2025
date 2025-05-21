extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Generics};

#[proc_macro_derive(HypedI2c)]
pub fn hyped_i2c_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hyped_i2c(&ast)
}

fn impl_hyped_i2c(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics: &Generics = &ast.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics HypedI2c for #name #ty_generics{
            fn read_byte(&mut self, device_address: u8, register_address: u8) -> Option<u8> {
                let mut read = [0];
                let result = self.i2c.lock(|i2c| {
                    i2c.borrow_mut().blocking_write_read(
                        device_address,
                        [register_address].as_ref(),
                        &mut read,
                    )
                });
                match result {
                    Ok(_) => Some(read[0]),
                    Err(_) => None,
                }
            }

            fn read_byte_16(&mut self, device_address: u8, register_address: u16) -> Option<u8> {
                let register_addr_hi = (register_address >> 8) as u8 & 0xFF;
                let register_addr_lo = register_address as u8 & 0xFF;
                let mut read = [0];
                let result = self.i2c.lock(|i2c| {
                    i2c.borrow_mut().blocking_write_read(
                        device_address,
                        [register_addr_hi, register_addr_lo].as_ref(),
                        &mut read,
                    )
                });
                match result {
                    Ok(_) => Some(read[0]),
                    Err(_) => None,
                }
            }


            fn write_byte_to_register(
                &mut self,
                device_address: u8,
                register_address: u8,
                data: u8,
            ) -> Result<(), I2cError> {
                let result = self.i2c.lock(|i2c| {
                    i2c.borrow_mut()
                        .blocking_write(device_address, [register_address, data].as_ref())
                });
                match result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(i2c_error_from_error(e)),
                }
            }

            fn write_byte(&mut self, device_address: u8, data: u8) -> Result<(), I2cError> {
                let result = self.i2c.lock(|i2c| {
                    i2c.borrow_mut().blocking_write(device_address, [data].as_ref())
                });
                match result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(i2c_error_from_error(e)),
                }
            }

            fn write_byte_to_register_16(
                &mut self,
                device_address: u8,
                register_address: u16,
                data: u8,
            ) -> Result<(), I2cError> {
                let register_addr_hi = (register_address >> 8) as u8;
                let register_addr_lo = register_address as u8;
                let result = self.i2c.lock(|i2c| {
                    i2c.borrow_mut()
                        .blocking_write(device_address, [register_addr_hi, register_addr_lo, data].as_ref())
                });
                match result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(i2c_error_from_error(e)),
                }
            }
        }

        impl #impl_generics #name #ty_generics {
            pub fn new(i2c: &'static Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>) -> Self {
                Self { i2c }
            }
        }

        fn i2c_error_from_error(error: i2c::Error) -> I2cError {
            match error {
                i2c::Error::Bus => I2cError::Bus,
                i2c::Error::Arbitration => I2cError::Arbitration,
                i2c::Error::Nack => I2cError::Nack,
                i2c::Error::Timeout => I2cError::Timeout,
                i2c::Error::Crc => I2cError::Crc,
                i2c::Error::Overrun => I2cError::Overrun,
                i2c::Error::ZeroLengthTransfer => I2cError::ZeroLengthTransfer,
            }
        }
    };
    gen.into()
}
