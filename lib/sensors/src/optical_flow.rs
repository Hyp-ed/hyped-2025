use embassy_time::{Duration, Instant, Timer};
use hyped_spi::{HypedSpi, SpiError};

/// Optical flow implements the logic to interact with the PMW3901MB-TXQT: Optical Motion Tracking Chip
///
/// This implementation is directly coming from https://github.com/pimoroni/pmw3901-python/blob/main/pmw3901/__init__.py
/// Data Sheet: https://www.codico.com/de/mpattachment/file/download/id/952/
pub struct OpticalFlow<'a, T: HypedSpi + 'a> {
    spi: &'a mut T,
}

impl<'a, T: HypedSpi> OpticalFlow<'a, T> {
    /// Note: ensure SPI instance is configured properly being passed in
    pub async fn new(spi: &'a mut T) -> Result<Self, OpticalFlowError> {
        let mut optical_flow = Self { spi };

        let power_up_reset_instr = &mut [REG_POWER_UP_RESET | 0x80, POWER_UP_RESET_INSTR];
        optical_flow.write(power_up_reset_instr)?;

        for offset in 0..NUM_UNIQUE_DATA_VALUES {
            let data = &mut [REG_DATA_READY + offset];
            optical_flow.read(data)?;
        }

        optical_flow.secret_sauce().await?;
        defmt::info!("Secret sauce done");

        // ensure device identifies itself correctly
        // let product_id_data = &mut [REG_PRODUCT_ID];
        // defmt::info!("Reading product ID...");
        // perform_transfer(optical_flow.spi, product_id_data)?;
        // defmt::info!("read");
        // match product_id_data.get(0) {
        //     Some(x)) if *x == PMW3901_PRODUCT_ID => (),
        //     _ => return Err(OpticalFlowError::InvalidProductId),
        // }

        // defmt::info!("Product ID check done");

        // let revision_id_data = &mut [REG_REVISION_ID];
        // perform_transfer(optical_flow.spi, revision_id_data)?;
        // match revision_id_data.get(0) {
        //     Some(x)) if VALID_PMW3901_REVISIONS.contains(x) => (),
        //     _ => return Err(OpticalFlowError::InvalidRevisionId),
        // }

        // defmt::info!("Revision ID check done");

        Ok(optical_flow)
    }

    fn write(&mut self, data: &mut [u8]) -> Result<(), OpticalFlowError> {
        match self.spi.transfer_in_place(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(OpticalFlowError::SpiError(e)),
        }
    }

    fn read(&mut self, data: &mut [u8]) -> Result<(), OpticalFlowError> {
        let register = data[0];
        for i in 0..data.len() {
            let _value = self
                .spi
                .transfer_in_place(&mut [register + i as u8, 0])
                .unwrap();
        }
        Ok(())
    }

    /// Get motion data from PMW3901 using burst read.
    pub async fn get_motion(&mut self) -> Result<(i16, i16), &'static str> {
        let start = Instant::now();

        while start.elapsed() < TIMEOUT {
            let mut data = [
                REG_MOTION_BURST, // Command byte to initiate burst read
                0x00,             // Placeholder for the rest of the 12 bytes
                0x00, // Definitely a better way of doing this but for some reason i was getting syntax errors
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
            ];

            self.spi
                .transfer_in_place(&mut data)
                .expect("Failed to read motion data.");

            // Parse the response data
            let response = &data[1..]; // Ignore the command byte
            let mut cursor = response.iter(); // Iterator to parse data sequentially

            let dr = cursor.next().unwrap();
            let _obs = cursor.next().unwrap();
            let x = i16::from_le_bytes([*cursor.next().unwrap(), *cursor.next().unwrap()]);
            let y = i16::from_le_bytes([*cursor.next().unwrap(), *cursor.next().unwrap()]);
            let quality = *cursor.next().unwrap();
            let _raw_sum = cursor.next();
            let _raw_max = cursor.next();
            let _raw_min = cursor.next();
            let shutter_upper = *cursor.next().unwrap();
            let _shutter_lower = cursor.next();

            // Validate the data
            if (dr & 0b1000_0000) != 0 && !(quality < 0x19 && shutter_upper == 0x1F) {
                return Ok((x, y));
            }

            // Wait before retrying
            Timer::after(RETRY_DURATION).await;
        }

        Err("Timed out waiting for motion data")
    }

    async fn bulk_write(&mut self, data: &[u8]) -> Result<(), OpticalFlowError> {
        for i in (0..data.len()).step_by(2) {
            let register = data[i];
            let value = data[i + 1];
            match register {
                WAIT => {
                    Timer::after(Duration::from_millis(value as u64)).await;
                }
                register => {
                    self.spi
                        .write(&[register, value])
                        .map_err(|e| OpticalFlowError::SpiError(e))?;
                }
            }
        }

        Ok(())
    }

    /// Perform bulk writes as per the Python implementation, but who knows wth this function does
    async fn secret_sauce(&mut self) -> Result<(), OpticalFlowError> {
        self.bulk_write(&[0x7F, 0x00, 0x55, 0x01, 0x50, 0x07, 0x7F, 0x0E, 0x43, 0x10])
            .await?;

        // Read from register 0x67
        let mut read_data = [0x67, 0x00];
        self.read(&mut read_data)
            .expect("Failed to read register 0x67");
        let result = read_data[1];

        // Perform conditional writes based on the read result
        let value_to_write = if result & 0b1000_0000 != 0 {
            0x04
        } else {
            0x02
        };

        self.write(&mut [0x48, value_to_write])?;

        // Perform the second bulk write
        self.bulk_write(&[0x7F, 0x00, 0x51, 0x7B, 0x50, 0x00, 0x55, 0x00, 0x7F, 0x0E])
            .await?;

        // Perform the conditional register adjustments
        let mut reg_73_data = [0x73, 0x00];
        self.read(&mut reg_73_data)
            .expect("Failed to read register 0x73");
        if reg_73_data[1] == 0x00 {
            let mut reg_70 = [0x70, 0x00];
            let mut reg_71 = [0x71, 0x00];

            self.read(&mut reg_70)
                .expect("Failed to read register 0x70");
            self.read(&mut reg_71)
                .expect("Failed to read register 0x71");

            let mut c1 = reg_70[1];
            let mut c2 = reg_71[1];

            if c1 <= 28 {
                c1 += 14;
            } else if c1 > 28 {
                c1 += 11;
            }

            c1 = c1.min(0x3F);
            c1 = c1.max(0x00);
            c2 = (c2 * 45) / 100;

            self.bulk_write(&[0x7F, 0x00, 0x61, 0xAD, 0x51, 0x70, 0x7F, 0x0E])
                .await?;

            self.write(&mut [0x70, c1])?;
            self.write(&mut [0x71, c2])?;
        }

        // Perform the third bulk write
        self.bulk_write(&[
            0x7F, 0x00, 0x61, 0xAD, 0x7F, 0x03, 0x40, 0x00, 0x7F, 0x05, 0x41, 0xB3, 0x43, 0xF1,
            0x45, 0x14, 0x5B, 0x32, 0x5F, 0x34, 0x7B, 0x08, 0x7F, 0x06, 0x44, 0x1B, 0x40, 0xBF,
            0x4E, 0x3F, 0x7F, 0x08, 0x65, 0x20, 0x6A, 0x18, 0x7F, 0x09, 0x4F, 0xAF, 0x5F, 0x40,
            0x48, 0x80, 0x49, 0x80, 0x57, 0x77, 0x60, 0x78, 0x61, 0x78, 0x62, 0x08, 0x63, 0x50,
            0x7F, 0x0A, 0x45, 0x60, 0x7F, 0x00, 0x4D, 0x11, 0x55, 0x80, 0x74, 0x21, 0x75, 0x1F,
            0x4A, 0x78, 0x4B, 0x78, 0x44, 0x08, 0x45, 0x50, 0x64, 0xFF, 0x65, 0x1F, 0x7F, 0x14,
            0x65, 0x67, 0x66, 0x08, 0x63, 0x70, 0x7F, 0x15, 0x48, 0x48, 0x7F, 0x07, 0x41, 0x0D,
            0x43, 0x14, 0x4B, 0x0E, 0x45, 0x0F, 0x44, 0x42, 0x4C, 0x80, 0x7F, 0x10, 0x5B, 0x02,
            0x7F, 0x07, 0x40, 0x41, 0x70, 0x00, WAIT, 0x0A, 0x32, 0x44, 0x7F, 0x07, 0x40, 0x40,
            0x7F, 0x06, 0x62, 0xF0, 0x63, 0x00, 0x7F, 0x0D, 0x48, 0xC0, 0x6F, 0xD5, 0x7F, 0x00,
            0x5B, 0xA0, 0x4E, 0xA8, 0x5A, 0x50, 0x40, 0x80, WAIT, 0xF0, 0x7F, 0x14, 0x6F, 0x1C,
            0x7F, 0x00,
        ])
        .await?;

        Ok(())
    }
}

/// Represents the possible errors that can occur when reading the optical flow sensor
#[derive(Debug)]
pub enum OpticalFlowError {
    SpiError(SpiError),
    InvalidProductId,
    InvalidRevisionId,
}

// Register Addresses:
const REG_PRODUCT_ID: u8 = 0x00;
const REG_REVISION_ID: u8 = 0x01;
const REG_DATA_READY: u8 = 0x02;
const REG_POWER_UP_RESET: u8 = 0x3A;
const REG_MOTION_BURST: u8 = 0x16;
const REG_ORIENTATION: u8 = 0x5B;
const REG_RESOLUTION: u8 = 0x4E;
const REG_RAWDATA_GRAB: u8 = 0x58;
const REG_RAWDATA_GRAB_STATUS: u8 = 0x59;

const TIMEOUT: Duration = Duration::from_secs(5);
const RETRY_DURATION: Duration = Duration::from_millis(10);

// Register Configurations:
const POWER_UP_RESET_INSTR: u8 = 0x5A;
const PMW3901_PRODUCT_ID: u8 = 0x49;
const VALID_PMW3901_REVISIONS: [u8; 2] = [0x01, 0x00];

// Sensor Constants:
const NUM_UNIQUE_DATA_VALUES: u8 = 5;
const WAIT: u8 = 0xFF;
