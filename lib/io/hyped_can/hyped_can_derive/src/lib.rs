extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Generics};

#[proc_macro_derive(HypedCan)]
pub fn hyped_can_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_hyped_can(&ast)
}

fn impl_hyped_can(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics: &Generics = &ast.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let gen = quote! {
        impl #impl_generics HypedCan for #name #ty_generics {
            fn read_frame(&mut self) -> Result<HypedEnvelope, CanError> {
                let result = self.can.lock(|can| can.borrow_mut().try_read());
                match result {
                    Ok(envelope) => Ok(HypedEnvelope {
                        frame: CanFrame {
                            can_id: {
                                let id = envelope.frame.id();
                                match id {
                                    embassy_stm32::can::Id::Standard(id) => id.as_raw() as u32, // 11-bit ID
                                    embassy_stm32::can::Id::Extended(id) => id.as_raw(),        // 29-bit ID
                                }
                            },
                            data: {
                                let mut data = [0u8; 8];
                                data.copy_from_slice(envelope.frame.data());
                                data
                            },
                        },
                        ts: envelope.ts,
                    }),
                    Err(embassy_stm32::can::enums::TryReadError::BusError(e)) => Err(match e {
                        embassy_stm32::can::enums::BusError::Stuff => CanError::Stuff,
                        embassy_stm32::can::enums::BusError::Form => CanError::Form,
                        embassy_stm32::can::enums::BusError::Acknowledge => CanError::Acknowledge,
                        embassy_stm32::can::enums::BusError::BitRecessive => CanError::BitRecessive,
                        embassy_stm32::can::enums::BusError::BitDominant => CanError::BitDominant,
                        embassy_stm32::can::enums::BusError::Crc => CanError::Crc,
                        embassy_stm32::can::enums::BusError::Software => CanError::Software,
                        embassy_stm32::can::enums::BusError::BusOff => CanError::BusOff,
                        embassy_stm32::can::enums::BusError::BusPassive => CanError::BusPassive,
                        embassy_stm32::can::enums::BusError::BusWarning => CanError::BusWarning,
                    }),
                    Err(embassy_stm32::can::enums::TryReadError::Empty) => Err(CanError::Empty),
                }
            }

            fn write_frame(&mut self, frame: &CanFrame) -> Result<(), CanError> {
                let frame = embassy_stm32::can::Frame::new(
                    match frame.can_id {
                        id if id <= 0x7FF => embassy_stm32::can::frame::Header::new(
                            embassy_stm32::can::Id::Standard(
                                embassy_stm32::can::StandardId::new(id as u16).unwrap(),
                            ),
                            frame.data.len() as u8,
                            false,
                        ),
                        id => embassy_stm32::can::frame::Header::new(
                            embassy_stm32::can::Id::Extended(
                                embassy_stm32::can::ExtendedId::new(id).unwrap(),
                            ),
                            frame.data.len() as u8,
                            false,
                        ),
                    },
                    &frame.data,
                );
                match frame {
                    Ok(frame) => {
                        let result = self.can.lock(|can| can.borrow_mut().try_write(&frame));
                        match result {
                            Ok(_) => Ok(()),
                            Err(embassy_stm32::can::TryWriteError::Full) => Err(CanError::Full),
                        }
                    }
                    Err(e) => Err(match e {
                        embassy_stm32::can::enums::FrameCreateError::NotEnoughData => {
                            CanError::NotEnoughData
                        }
                        embassy_stm32::can::enums::FrameCreateError::InvalidDataLength => {
                            CanError::InvalidDataLength
                        }
                        embassy_stm32::can::enums::FrameCreateError::InvalidCanId => CanError::InvalidCanId,
                    }),
                }
            }
        }


        impl #impl_generics #name #ty_generics {
            pub fn new(can: &'static Mutex<NoopRawMutex, RefCell<Can<'static>>>) -> Self {
                Self { can }
            }
        }

    };
    gen.into()
}
