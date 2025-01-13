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
const RETRY_DURATION: Duration = Duration::from_millis(10);


// Register Configurations:
const POWER_UP_RESET_INSTR: Word = U8(0x5A);
const PMW3901_PRODUCT_ID: u8 = 0x49;
const VALID_PMW3901_REVISIONS: [u8; 2] = [0x01, 0x00];

// Sensor Constants:
const NUM_UNIQUE_DATA_VALUES: u8 = 5;
const WAIT: Word = U8(0xFF);


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
                Word::U8(0x00),          // Definitely a better way of doing this but for some reason i was getting syntax errors
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
                return Ok((x, y)); 
            }
    
            // Wait before retrying
            sleep(RETRY_DURATION);
        }
    
        Err("Timed out waiting for motion data")
    }

    pub fn bulk_write(&mut self, data: &[Word]) -> Result<(), OpticalFlowError> {
        let mut i = 0;

        while i < data.len() {
            match data[i] {
                WAIT => {
                    // Handle WAIT instruction
                    if i + 1 >= data.len() {
                        return Err(OpticalFlowError::SpiError(SpiError::ModeFault));
                    }

                    // Get the delay value from the next element
                    if let Word::U8(delay_ms) = data[i + 1] {
                        sleep(Duration::from_millis(delay_ms as u64)); // Perform the delay
                    } else {
                        return Err(OpticalFlowError::SpiError(SpiError::ModeFault));
                    }

                    i += 2; // Skip WAIT and delay value
                }
                Word::U8(register) => {
                    // Handle register-value pair
                    if i + 1 >= data.len() {
                        return Err(OpticalFlowError::SpiError(SpiError::ModeFault));
                    }

                    // Get the value to write to the register
                    if let Word::U8(value) = data[i + 1] {
                        let mut words = [Word::U8(register), Word::U8(value)];
                        self.spi
                            .write(&words)
                            .map_err(|e| OpticalFlowError::SpiError(e))?;
                    } else {
                        return Err(OpticalFlowError::SpiError(SpiError::ModeFault));
                    }

                    i += 2; // Move to the next pair
                }
                _ => return Err(OpticalFlowError::SpiError(SpiError::ModeFault)), // Unexpected Word type
            }
        }

        Ok(())
    }


    pub fn secret_sauce(&mut self) -> Result<(), OpticalFlowError> {
        // Perform bulk writes as per the Python implementation, but who knows wth this function does
        self.bulk_write(&[
            U8(0x7F), U8(0x00),
            U8(0x55), U8(0x01),
            U8(0x50), U8(0x07),
            U8(0x7F), U8(0x0E),
            U8(0x43), U8(0x10),
        ])?;

        // Read from register 0x67
        let mut read_data = [U8(0x67), U8(0x00)];
        self.spi.transfer_in_place(&mut read_data).map_err(|e| OpticalFlowError::SpiError(e))?;
        let result = if let U8(value) = read_data[1] {
            value
        } else {
            return Err(OpticalFlowError::SpiError(SpiError::ModeFault));
        };

        // Perform conditional writes based on the read result
        let value_to_write = if result & 0b1000_0000 != 0 {
            0x04
        } else {
            0x02
        };

        self.bulk_write(&[
            U8(0x48), U8(value_to_write),
        ])?;

        // Perform the second bulk write
        self.bulk_write(&[
            U8(0x7F), U8(0x00),
            U8(0x51), U8(0x7B),
            U8(0x50), U8(0x00),
            U8(0x55), U8(0x00),
            U8(0x7F), U8(0x0E),
        ])?;

        // Perform the conditional register adjustments
        let mut reg_73_data = [U8(0x73), U8(0x00)];
        self.spi.transfer_in_place(&mut reg_73_data).map_err(|e| OpticalFlowError::SpiError(e))?;
        if let U8(0x00) = reg_73_data[1] {
            let mut reg_70_data = [U8(0x70), U8(0x00)];
            let mut reg_71_data = [U8(0x71), U8(0x00)];
            self.spi.transfer_in_place(&mut reg_70_data).map_err(|e| OpticalFlowError::SpiError(e))?;
            self.spi.transfer_in_place(&mut reg_71_data).map_err(|e| OpticalFlowError::SpiError(e))?;

            let mut c1 = if let U8(value) = reg_70_data[1] {
                value
            } else {
                return Err(OpticalFlowError::SpiError(SpiError::ModeFault));
            };
            let mut c2 = if let U8(value) = reg_71_data[1] {
                value
            } else {
                return Err(OpticalFlowError::SpiError(SpiError::ModeFault));
            };

            if c1 <= 28 {
                c1 += 14;
            } else if c1 > 28 {
                c1 += 11;
            }

            c1 = c1.min(0x3F);
            c2 = (c2 * 45) / 100;

            self.bulk_write(&[
                U8(0x7F), U8(0x00),
                U8(0x61), U8(0xAD),
                U8(0x51), U8(0x70),
                U8(0x7F), U8(0x0E),
                U8(0x70), U8(c1),
                U8(0x71), U8(c2),
            ])?;
        }

        // Perform the third bulk write
        self.bulk_write(&[
            U8(0x7F), U8(0x00),
            U8(0x61), U8(0xAD),
            U8(0x7F), U8(0x03),
            U8(0x40), U8(0x00),
            U8(0x7F), U8(0x05),

            U8(0x41), U8(0xB3),
            U8(0x43), U8(0xF1),
            U8(0x45), U8(0x14),
            U8(0x5B), U8(0x32),
            U8(0x5F), U8(0x34),
            U8(0x7B), U8(0x08),
            U8(0x7F), U8(0x06),
            U8(0x44), U8(0x1B),
            U8(0x40), U8(0xBF),
            U8(0x4E), U8(0x3F),
            U8(0x7F), U8(0x08),
            U8(0x65), U8(0x20),
            U8(0x6A), U8(0x18),

            U8(0x7F), U8(0x09),
            U8(0x4F), U8(0xAF),
            U8(0x5F), U8(0x40),
            U8(0x48), U8(0x80),
            U8(0x49), U8(0x80),

            U8(0x57), U8(0x77),
            U8(0x60), U8(0x78),
            U8(0x61), U8(0x78),
            U8(0x62), U8(0x08),
            U8(0x63), U8(0x50),
            U8(0x7F), U8(0x0A),
            U8(0x45), U8(0x60),
            U8(0x7F), U8(0x00),
            U8(0x4D), U8(0x11),

            U8(0x55), U8(0x80),
            U8(0x74), U8(0x21),
            U8(0x75), U8(0x1F),
            U8(0x4A), U8(0x78),
            U8(0x4B), U8(0x78),

            U8(0x44), U8(0x08),
            U8(0x45), U8(0x50),
            U8(0x64), U8(0xFF),
            U8(0x65), U8(0x1F),
            U8(0x7F), U8(0x14),
            U8(0x65), U8(0x67),
            U8(0x66), U8(0x08),
            U8(0x63), U8(0x70),
            U8(0x7F), U8(0x15),
            U8(0x48), U8(0x48),
            U8(0x7F), U8(0x07),
            U8(0x41), U8(0x0D),
            U8(0x43), U8(0x14),

            U8(0x4B), U8(0x0E),
            U8(0x45), U8(0x0F),
            U8(0x44), U8(0x42),
            U8(0x4C), U8(0x80),
            U8(0x7F), U8(0x10),

            U8(0x5B), U8(0x02),
            U8(0x7F), U8(0x07),
            U8(0x40), U8(0x41),
            U8(0x70), U8(0x00),
            WAIT, U8(0x0A),  // Sleep for 10ms

            U8(0x32), U8(0x44),
            U8(0x7F), U8(0x07),
            U8(0x40), U8(0x40),
            U8(0x7F), U8(0x06),
            U8(0x62), U8(0xF0),
            U8(0x63), U8(0x00),
            U8(0x7F), U8(0x0D),
            U8(0x48), U8(0xC0),
            U8(0x6F), U8(0xD5),
            U8(0x7F), U8(0x00),

            U8(0x5B), U8(0xA0),
            U8(0x4E), U8(0xA8),
            U8(0x5A), U8(0x50),
            U8(0x40), U8(0x80),
            WAIT, U8(0xF0),

            U8(0x7F), U8(0x14),  // Enable LED_N pulsing
            U8(0x6F), U8(0x1C),
            U8(0x7F), U8(0x00),

        ])?;

        Ok(())
    }


}
