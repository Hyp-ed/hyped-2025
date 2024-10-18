/// SPI errors that can occur
/// From: https://docs.embassy.dev/embassy-stm32/git/stm32f103c8/spi/enum.Error.html
pub enum SpiError {
    Framing,
    Crc,
    ModeFault,
    Overrun,
}

/// A word is either u8 or u16
pub enum WordSize {
    U8(u8),
    U16(u16),
}

/// SPI trait used to abstract SPI and allow for mocking
pub trait HypedSpi {
    /// Read data into `words` from the SPI sensor
    fn read(&mut self, words: &mut [WordSize]) -> Result<(), SpiError>;
    /// Write data from `words` to the SPI sensor
    fn write(&mut self, words: &[WordSize]) -> Result<(), SpiError>;
}
