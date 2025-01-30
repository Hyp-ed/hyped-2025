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
                        frame: HypedCanFrame {
                            can_id: {
                                let id = envelope.frame.id();
                                match id {
                                    Id::Standard(id) => id.as_raw() as u32, // 11-bit ID
                                    Id::Extended(id) => id.as_raw(),        // 29-bit ID
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
                    Err(TryReadError::BusError(e)) => Err(match e {
                        BusError::Stuff => CanError::Stuff,
                        BusError::Form => CanError::Form,
                        BusError::Acknowledge => CanError::Acknowledge,
                        BusError::BitRecessive => CanError::BitRecessive,
                        BusError::BitDominant => CanError::BitDominant,
                        BusError::Crc => CanError::Crc,
                        BusError::Software => CanError::Software,
                        BusError::BusOff => CanError::BusOff,
                        BusError::BusPassive => CanError::BusPassive,
                        BusError::BusWarning => CanError::BusWarning,
                    }),
                    Err(TryReadError::Empty) => Err(CanError::Empty),
                }
            }

            fn write_frame(&mut self, frame: &HypedCanFrame) -> Result<(), CanError> {
                let frame = Frame::new(
                    match frame.can_id {
                        id if id <= 0x7FF => frame::Header::new(
                            Id::Standard(
                                StandardId::new(id as u16).unwrap(),
                            ),
                            frame.data.len() as u8,
                            false,
                        ),
                        id => frame::Header::new(
                            Id::Extended(
                                ExtendedId::new(id).unwrap(),
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
                            Err(TryWriteError::Full) => Err(CanError::Full),
                        }
                    }
                    Err(e) => Err(match e {
                        FrameCreateError::NotEnoughData => {
                            CanError::NotEnoughData
                        }
                        FrameCreateError::InvalidDataLength => {
                            CanError::InvalidDataLength
                        }
                        FrameCreateError::InvalidCanId => CanError::InvalidCanId,
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
