extern crate std;
use hyped_io::spi::Word::{self, U8};
use hyped_io::spi::{HypedSpi, SpiError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::thread::sleep;

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

const TIMEOUT: Duration = Duration::from_secs(5);


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


    pub fn get_motion(&mut self) -> Result<(i16, i16), &'static str>{
        // Get motion data from PMW3901 using burst read.    

        let start = Instant::now();

        while start.elapsed() < TIMEOUT {
            let mut data = [
                REG_MOTION_BURST,        // Command byte to initiate burst read
                Word::U8(0x00),          // Placeholder for the rest of the 12 bytes
                Word::U8(0x00),          // Definitely a better way of doing this but for some reason i was getting syntaz errors
                Word::U8(0x00),         
                Word::U8(0x00),          
                Word::U8(0x00),
                Word::U8(0x00), 
                Word::U8(0x00), 
                Word::U8(0x00), 
                Word::U8(0x00), 
                Word::U8(0x00), 
                Word::U8(0x00), 
                Word::U8(0x00),       
            ];         
                
            
            // Perform the SPI transfer
            self.spi.transfer_in_place(&mut data).map_err(|_| "SPI read failed")?;
    
            // Parse the response data
            let response = &data[1..]; // Ignore the command byte
            let mut cursor = response.iter(); // Iterator to parse data sequentially
    
            let dr = match cursor.next() {
                Some(Word::U8(x)) => *x,
                _ => return Err("Failed to parse dr"),
            };
            let _obs = match cursor.next() {
                Some(Word::U8(x)) => *x,
                _ => return Err("Failed to parse obs"),
            };
            let x = match (cursor.next(), cursor.next()) {
                (Some(Word::U8(lsb)), Some(Word::U8(msb))) => {
                    i16::from_le_bytes([*lsb, *msb])
                }
                _ => return Err("Failed to parse x"),
            };
            let y = match (cursor.next(), cursor.next()) {
                (Some(Word::U8(lsb)), Some(Word::U8(msb))) => {
                    i16::from_le_bytes([*lsb, *msb])
                }
                _ => return Err("Failed to parse y"),
            };
            let quality = match cursor.next() {
                Some(Word::U8(x)) => *x,
                _ => return Err("Failed to parse quality"),
            };
            let _raw_sum = match cursor.next() {
                Some(Word::U8(x)) => *x,
                _ => return Err("Failed to parse raw_sum"),
            };
            let _raw_max = match cursor.next() {
                Some(Word::U8(x)) => *x,
                _ => return Err("Failed to parse raw_max"),
            };
            let _raw_min = match cursor.next() {
                Some(Word::U8(x)) => *x,
                _ => return Err("Failed to parse raw_min"),
            };
            let shutter_upper = match cursor.next() {
                Some(Word::U8(x)) => *x,
                _ => return Err("Failed to parse shutter_upper"),
            };
            let _shutter_lower = match cursor.next() {
                Some(Word::U8(x)) => *x,
                _ => return Err("Failed to parse shutter_lower"),
            };
    
            // Validate the data
            if (dr & 0b1000_0000) != 0 && !(quality < 0x19 && shutter_upper == 0x1F) {
                return Ok((x, y)); // Return delta x and delta y if valid
            }
    
            // Wait before retrying
            sleep(Duration::from_millis(10));
        }
    
        Err("Timed out waiting for motion data")
    }
}
