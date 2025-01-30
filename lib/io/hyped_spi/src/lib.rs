#![no_std]

use core::ops::Add;

/// SPI errors that can occur
/// From: https://docs.embassy.dev/embassy-stm32/git/stm32f103c8/spi/enum.Error.html
#[derive(Debug)]
pub enum SpiError {
    Framing,
    Crc,
    ModeFault,
    Overrun,
}

/// Keeping this generic over either size for compatibility
/// For example: some sensors may need to a byte written to them
/// and return two bytes in a single transaction
#[derive(PartialEq)] // Derive PartialEq for Word
pub enum HypedWord {
    U8(u8),
    U16(u16),
}

impl Add for HypedWord {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (HypedWord::U8(x), HypedWord::U8(y)) => HypedWord::U8(x + y),
            (HypedWord::U16(x), HypedWord::U16(y)) => HypedWord::U16(x + y),
            (HypedWord::U16(x), HypedWord::U8(y)) => HypedWord::U16(x + y as u16),
            (HypedWord::U8(x), HypedWord::U16(y)) => HypedWord::U16(x as u16 + y),
        }
    }
}

/// SPI trait used to abstract SPI and allow for mocking
/// Note: SPI has many configurable parameters,
/// but we assume the actual implementation to handle this per sensor.
pub trait HypedSpi {
    /// Read a list of values (bytes) from an SPI device
    /// Note: the length of data read is implicit in the width of words
    fn read(&mut self, words: &mut [HypedWord]) -> Result<(), SpiError>;
    /// Write a list of bytes to an SPI device
    fn write(&mut self, words: &[HypedWord]) -> Result<(), SpiError>;
    /// Perform a Bidirectional transfer (using DMA), i.e. an SPI transaction
    /// A list of bytes is written to the SPI device
    /// and as each byte in that list is sent out, it is replaced by the data
    /// simultaneously read from the SPI device over the MISO line.
    fn transfer_in_place(&mut self, data: &mut [HypedWord]) -> Result<(), SpiError>;
}
