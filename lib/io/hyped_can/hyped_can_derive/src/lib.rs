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
        impl #impl_generics HypedCan for #name #ty_generics{

            async fn read_frame(&mut self) -> Result<Envelope, CanError> {
                let result = self.can.lock(|can| {
                    can.borrow_mut().read()
                })
                match result {
                    Ok(_) => Ok(result),
                    Err(e) => Err(match e {
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
                }
            }

            fn try_read_frame(&mut self) -> Result<Envelope, CanError> {
                let result = self.can.lock(|can| {
                    can.borrow().try_read()
                })
                match result {
                    Ok(_) => Ok(result),
                    Err(e) => Err(match e {
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
                }
            }

            async fn write_frame(&mut self, frame: &CanFrame) {

                self.can.lock(|can| {
                    can.borrow_mut().write(frame)
                });
            }

            fn try_write_frame(&mut self, frame: &CanFrame) -> Result<(), CanError> {

                let result = self.can.lock(|can| {
                    can.borrow_mut().try_write(frame)
                });
                match result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(match e {
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
                }
            }
        }

        impl #impl_generics #name #ty_generics {
            pub fn new(can: &'static Mutex<NoopRawMutex, RefCell<Can<'static, Blocking>>>) -> Self {
                Self { can }
            }
        }

    };
    gen.into()
}
