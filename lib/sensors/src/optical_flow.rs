use defmt::warn;
use embassy_time::{Duration, Instant, Timer};
use hyped_gpio::HypedGpioOutputPin;
use hyped_spi::{HypedSpi, SpiError};

/// Optical flow implements the logic to interact with the PMW3901MB-TXQT: Optical Motion Tracking Chip
///
/// This implementation is directly coming from https://github.com/pimoroni/pmw3901-python/blob/main/pmw3901/__init__.py
/// Data Sheet: https://www.codico.com/de/mpattachment/file/download/id/952/
pub struct OpticalFlow<'a, T: HypedSpi + 'a, C: HypedGpioOutputPin> {
    spi: &'a mut T,
    cs: C,
}

impl<'a, T: HypedSpi, C: HypedGpioOutputPin> OpticalFlow<'a, T, C> {
    /// Note: ensure SPI instance is configured properly being passed in
    pub async fn new(spi: &'a mut T, cs: C) -> Result<Self, OpticalFlowError> {
        let mut optical_flow = Self { spi, cs };

        optical_flow.cs.set_low();
        Timer::after(Duration::from_millis(5)).await;
        optical_flow.cs.set_high();

        let power_up_reset_instr = &mut [REG_POWER_UP_RESET, POWER_UP_RESET_INSTR];
        Timer::after(Duration::from_millis(2)).await;
        optical_flow.write(power_up_reset_instr)?;

        for offset in 0..NUM_UNIQUE_DATA_VALUES {
            optical_flow.read(REG_DATA_READY + offset)?;
        }

        optical_flow.secret_sauce().await?;
        defmt::info!("Secret sauce done");

        // let (product_id, revision_id) = optical_flow.get_id()?;
        // if product_id != PMW3901_PRODUCT_ID {
        //     warn!("Invalid product id: {}", product_id);
        // }

        // if !VALID_PMW3901_REVISIONS.contains(&revision_id) {
        //     warn!("Invalid revision id: {}", revision_id);
        // }

        Ok(optical_flow)
    }

    fn get_id(&mut self) -> Result<(u8, u8), OpticalFlowError> {
        let product_id = self.read(REG_PRODUCT_ID)?;
        let revision_id = self.read(REG_REVISION_ID)?;
        Ok((product_id, revision_id))
    }

    fn write(&mut self, data: &mut [u8; 2]) -> Result<(), OpticalFlowError> {
        self.cs.set_low();
        let result = match self.spi.transfer_in_place(
            // OR 0x80 to the register
            &mut [data[0] | 0x80, data[1]],
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(OpticalFlowError::SpiError(e)),
        };
        self.cs.set_high();
        result
    }

    fn read(&mut self, register: u8) -> Result<u8, OpticalFlowError> {
        let data = &mut [register, 0];
        self.cs.set_low();
        self.spi.transfer_in_place(data).unwrap();
        self.cs.set_high();
        Ok(data[1])
    }

    /// Get motion data from PMW3901 using burst read.
    pub async fn get_motion(&mut self) -> Result<(i16, i16), &'static str> {
        let start = Instant::now();

        while start.elapsed() < TIMEOUT {
            self.cs.set_low();
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
            self.cs.set_high();

            // Parse the response data
            let response = &data[1..]; // Ignore the command byte

            defmt::info!("Response: {:?}", response);

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
                    let _ = self.write(&mut [register, value]);
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
        let result = self.read(0x67).expect("Failed to read register 0x67");

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
        let reg_73_data = self.read(0x73).expect("Failed to read register 0x73");

        if reg_73_data == 0x00 {
            let mut c1 = self.read(0x70).expect("Failed to read register 0x70");
            let mut c2 = self.read(0x71).expect("Failed to read register 0x71");

            if c1 <= 28 {
                c1 += 14;
            } else if c1 > 28 {
                c1 += 11;
            }

            c1 = c1.min(0x3F);
            c1 = c1.max(0x00);
            c2 = (c2 * 45) / 100; // TODO: maybe need floor division

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
