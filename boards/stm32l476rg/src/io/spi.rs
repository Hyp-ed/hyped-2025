use embassy_stm32::{spi::Spi, mode::Blocking};
use hyped_io::spi::{HypedSpi, SpiError};
//figure out reading, writing, transfer in place from board

pub struct Stm32l476rgSpi<'d> {
    spi: Spi<'d, Blocking>,
}

impl<'d> HypedSpi for Stm32l476rgSpi<'d> {
    

   
}

impl<'d> Stm32l476rgSpic<'d> {
    /// Create a new instance of our SPI implementation for the STM32L476RG
    pub fn new(spi: Spi<'d, Blocking>) -> Self {
        Self { spi }
    }
}
