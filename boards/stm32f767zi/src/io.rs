use core::cell::RefCell;
use embassy_stm32::adc::{Adc, AnyAdcChannel, Instance};
use embassy_stm32::gpio::{Input, Output};
use embassy_stm32::spi::{self, Spi};
use embassy_stm32::{i2c::I2c, mode::Blocking};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};

use heapless::{String, Vec};
use hyped_adc::HypedAdc;
use hyped_adc_derive::HypedAdc;
use hyped_core::format;
use hyped_gpio::{HypedGpioInputPin, HypedGpioOutputPin};
use hyped_gpio_derive::{HypedGpioInputPin, HypedGpioOutputPin};
use hyped_i2c::{HypedI2c, I2cError};
use hyped_i2c_derive::HypedI2c;
use hyped_spi::{HypedSpi, SpiError};

#[derive(HypedAdc)]
pub struct Stm32f767ziAdc<'d, T: Instance> {
    adc: Adc<'d, T>,
    channel: AnyAdcChannel<T>,
}

#[derive(HypedGpioInputPin)]
pub struct Stm32f767ziGpioInput {
    pin: Input<'static>,
}

#[derive(HypedGpioOutputPin)]
pub struct Stm32f767ziGpioOutput {
    pin: Output<'static>,
}

#[derive(HypedI2c)]
pub struct Stm32f767ziI2c<'d> {
    i2c: &'d Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>,
}

// #[derive(HypedSpi)]
pub struct Stm32f767ziSpi {
    spi: Spi<'static, Blocking>,
}

impl Stm32f767ziSpi {
    pub fn new(spi: Spi<'static, Blocking>) -> Self {
        Self { spi }
    }
}

impl HypedSpi for Stm32f767ziSpi {
    fn read(&mut self, words: &mut [u8]) -> Result<(), SpiError> {
        // only support u8 for now, create a new vec to store the read data
        let mut binding = Vec::<u8, 64>::new();
        let new_words = binding.as_mut_slice();
        defmt::info!("r: {:#04x} {:#04x}", new_words[0], new_words[1]);
        match self.spi.blocking_read(new_words) {
            Ok(_) => {
                // convert new_words to HypedWord and store it in words
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
        // only support u8 for now, convert HypedWord to u8
        let binding = words.iter().map(|word| *word).collect::<Vec<u8, 64>>();
        let new_words = binding.as_slice();
        defmt::info!("w: {:#04x} {:#04x}", new_words[0], new_words[1]);
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
        // only support u8 for now, convert HypedWord to u8
        let mut binding = data.iter().map(|word| *word).collect::<Vec<u8, 64>>();
        let new_words = binding.as_mut_slice();
        defmt::info!("t: {:#04x} {:#04x}", new_words[0], new_words[1]);
        match self.spi.blocking_transfer_in_place(new_words) {
            Ok(_) => {
                // convert new_words to HypedWord and store it in data
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
