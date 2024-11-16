use hyped_io::spi::Word::{self, U8};
use hyped_io::spi::{HypedSpi, SpiError};

/// Optical flow implements the logic to interact with the PMW3901MB-TXQT: Optical Motion Tracking Chip
///
/// This implementation is directly coming from https://github.com/pimoroni/pmw3901-python/blob/main/pmw3901/__init__.py
/// Data Sheet: https://www.codico.com/de/mpattachment/file/download/id/952/

// Register Addresses:
const REG_PRODUCT_ID: Word = U8(0x00);
const REG_REVISION_ID: Word = U8(0x01);
const REG_DATA_READY: Word = U8(0x02);
const REG_POWER_UP_RESET: Word = U8(0x3A);
const REG_MOTION_BURST: Word = U8(0x16);
const REG_ORIENTATION: Word = U8(0x5B);
const REG_RESOLUTION: Word = U8(0x4E);
const REG_RAWDATA_GRAB: Word = U8(0x58);
const REG_RAWDATA_GRAB_STATUS: Word = U8(0x59);

// Register Configurations:
const POWER_UP_RESET_INSTR: Word = U8(0x5A);
const PMW3901_PRODUCT_ID: u8 = 0x49;
const VALID_PMW3901_REVISIONS: [u8; 2] = [0x01, 0x00];

// Sensor Constants:
const NUM_UNIQUE_DATA_VALUES: u8 = 5;

/// Represents the possible errors that can occur when reading the optical flow sensor
#[derive(Debug)]
pub enum OpticalFlowError {
    SpiError(SpiError),
    InvalidProductId,
    InvalidRevisionId,
}

pub struct OpticalFlow<'a, T: HypedSpi + 'a> {
    spi: &'a mut T,
}

impl<'a, T: HypedSpi> OpticalFlow<'a, T> {
    /// Note: ensure SPI instance is configured properly being passed in
    pub fn new(spi: &'a mut T) -> Result<Self, OpticalFlowError> {
        fn perform_transfer<'b, T: HypedSpi>(
            spi: &'b mut T,
            data: &mut [Word],
        ) -> Result<(), OpticalFlowError> {
            match spi.transfer_in_place(data) {
                Ok(_) => Ok(()),
                Err(e) => Err(OpticalFlowError::SpiError(e)),
            }
        }
        // perform secret_sauce_ (yes you read that right...)
        let power_up_reset_instr = &mut [REG_POWER_UP_RESET, POWER_UP_RESET_INSTR];
        perform_transfer(spi, power_up_reset_instr)?;
        // TODO(ishmis): test whether the below reads are even necessary (multiple implementations have this)
        for offset in 0..NUM_UNIQUE_DATA_VALUES {
            let data = &mut [REG_DATA_READY + Word::U8(offset)];
            perform_transfer(spi, data)?;
        }
        // TODO: do secret sauce!!
        // ensure device identifies itself correctly
        let product_id_data = &mut [REG_PRODUCT_ID];
        perform_transfer(spi, product_id_data)?;
        match product_id_data.get(0) {
            Some(U8(x)) if *x == PMW3901_PRODUCT_ID => (),
            _ => return Err(OpticalFlowError::InvalidProductId),
        }
        let revision_id_data = &mut [REG_REVISION_ID];
        perform_transfer(spi, revision_id_data)?;
        match revision_id_data.get(0) {
            Some(U8(x)) if VALID_PMW3901_REVISIONS.contains(x) => (),
            _ => return Err(OpticalFlowError::InvalidRevisionId),
        }
        Ok(Self { spi })
    }
}
