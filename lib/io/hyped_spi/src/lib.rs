#![no_std]

/// SPI errors that can occur
/// From: https://docs.embassy.dev/embassy-stm32/git/stm32f103c8/spi/enum.Error.html
#[derive(Debug)]
pub enum SpiError {
    Framing,
    Crc,
    ModeFault,
    Overrun,
}

/// SPI trait used to abstract SPI and allow for mocking
/// Note: SPI has many configurable parameters,
/// but we assume the actual implementation to handle this per sensor.
pub trait HypedSpi {
    /// Read a list of values (bytes) from an SPI device
    /// Note: the length of data read is implicit in the width of words
    fn read(&mut self, words: &mut [u8]) -> Result<(), SpiError>;
    /// Write a list of bytes to an SPI device
    fn write(&mut self, words: &[u8]) -> Result<(), SpiError>;
    /// Perform a Bidirectional transfer (using DMA), i.e. an SPI transaction
    /// A list of bytes is written to the SPI device
    /// and as each byte in that list is sent out, it is replaced by the data
    /// simultaneously read from the SPI device over the MISO line.
    fn transfer_in_place(&mut self, data: &mut [u8]) -> Result<(), SpiError>;
}
