use core::cell::RefCell;

use embassy_stm32::adc::{Adc, AnyAdcChannel, Instance};
use embassy_stm32::gpio::{Input, Output};
use embassy_stm32::spi::{self, Spi, Word};
use embassy_stm32::{i2c::I2c, mode::Blocking};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use heapless::Vec;

use hyped_adc::HypedAdc;
use hyped_adc_derive::HypedAdc;
use hyped_gpio::{HypedGpioInputPin, HypedGpioOutputPin};
use hyped_gpio_derive::{HypedGpioInputPin, HypedGpioOutputPin};
use hyped_i2c::{HypedI2c, I2cError};
use hyped_i2c_derive::HypedI2c;
use hyped_spi::{HypedSpi, HypedWord, SpiError};
use hyped_spi_derive::HypedSpi;

#[derive(HypedAdc)]
pub struct Stm32l476rgAdc<'d, T: Instance> {
    adc: Adc<'d, T>,
    channel: AnyAdcChannel<T>,
}

#[derive(HypedGpioInputPin)]
pub struct Stm32l476rgGpioInput {
    pin: Input<'static>,
}

#[derive(HypedGpioOutputPin)]
pub struct Stm32l476rgGpioOutput {
    pin: Output<'static>,
}

#[derive(HypedI2c)]
pub struct Stm32l476rgI2c<'d> {
    i2c: &'d Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>,
}

// #[derive(HypedSpi)]
pub struct Stm32l476rgSpi {
    spi: Spi<'static, Blocking>,
}

impl HypedSpi for Stm32l476rgSpi {
    fn read(&mut self, words: &mut [HypedWord]) -> Result<(), SpiError> {
        // only support u8 for now, create a new vec to store the read data
        let mut binding = Vec::<u8, 64>::new();
        let new_words = binding.as_mut_slice();
        match self.spi.blocking_read(new_words) {
            Ok(_) => {
                // convert new_words to HypedWord and store it in words
                for (i, word) in new_words.iter().enumerate() {
                    words[i] = HypedWord::U8(*word);
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

    fn write(&mut self, words: &[HypedWord]) -> Result<(), SpiError> {
        // only support u8 for now, convert HypedWord to u8
        let mut binding = words
            .iter()
            .map(|word| match word {
                HypedWord::U8(val) => *val,
                _ => panic!("Only support u8 for now"),
            })
            .collect::<Vec<u8, 64>>();
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

    fn transfer_in_place(&mut self, data: &mut [HypedWord]) -> Result<(), SpiError> {
        // only support u8 for now, convert HypedWord to u8
        let mut binding = data
            .iter()
            .map(|word| match word {
                HypedWord::U8(val) => *val,
                _ => panic!("Only support u8 for now"),
            })
            .collect::<Vec<u8, 64>>();
        let new_words = binding.as_mut_slice();

        match self.spi.blocking_transfer_in_place(new_words) {
            Ok(_) => {
                // convert new_words to HypedWord and store it in data
                for (i, word) in new_words.iter().enumerate() {
                    data[i] = HypedWord::U8(*word);
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
