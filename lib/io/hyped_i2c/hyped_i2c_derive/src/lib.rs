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
            /// Read a byte from a register on a device
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

            /// Read a byte from a register with a 16-bit address on a device
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


            /// Write a byte to a register on a device
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
                    Err(e) => Err(match e {
                        embassy_stm32::i2c::Error::Bus => I2cError::Bus,
                        embassy_stm32::i2c::Error::Arbitration => I2cError::Arbitration,
                        embassy_stm32::i2c::Error::Nack => I2cError::Nack,
                        embassy_stm32::i2c::Error::Timeout => I2cError::Timeout,
                        embassy_stm32::i2c::Error::Crc => I2cError::Crc,
                        embassy_stm32::i2c::Error::Overrun => I2cError::Overrun,
                        embassy_stm32::i2c::Error::ZeroLengthTransfer => I2cError::ZeroLengthTransfer,
                    }),
                }
            }

            // Write a byte to a register with a 16-bit address on a device
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
            Err(e) => Err(match e {
                embassy_stm32::i2c::Error::Bus => I2cError::Bus,
                embassy_stm32::i2c::Error::Arbitration => I2cError::Arbitration,
                embassy_stm32::i2c::Error::Nack => I2cError::Nack,
                embassy_stm32::i2c::Error::Timeout => I2cError::Timeout,
                embassy_stm32::i2c::Error::Crc => I2cError::Crc,
                embassy_stm32::i2c::Error::Overrun => I2cError::Overrun,
                embassy_stm32::i2c::Error::ZeroLengthTransfer => I2cError::ZeroLengthTransfer,
            }),
        }
    }

        }

        impl #impl_generics #name #ty_generics {
            /// Create a new instance of our I2C implementation for the STM32L476RG
            pub fn new(i2c: Mutex<CriticalSectionRawMutex, RefCell<I2c<'d, Blocking>>>) -> Self {
                Self { i2c }
            }
        }

    };
    gen.into()
}
