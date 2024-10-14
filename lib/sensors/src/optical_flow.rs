use crate::spi::HypedSpi;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use byteorder::{LittleEndian, ReadBytesExt};
use std::thread::sleep;
use serde::{Serialize, Deserialize};
use serde_json::json;


// implement get_motion function first
// add readByteFromRegister (?) function to HypedSPI trait which takes register address and gives data

const REG_MOTION_BURST: u8 = 0x16;
const TIMEOUT: Duration = Duration::from_secs(5);


pub struct PMW3901<SPI> {
    spi: SPI,

}

impl<SPI> PMW3901<SPI>
where
    SPI: HypedSpi,
{

    
    pub fn new(spi: SPI) -> Self {
        PMW3901 { spi }
    }

    // **ATTEMPTED PYTHON IMPLEMENTATION OF get_motion**
    pub fn get_motion(&mut self) -> Result<(i16, i16), &'static str>{
        let start = Instant::now();


        while start.elapsed() < TIMEOUT {
            // Send burst read command and read 12 bytes
            let mut data = [0u8; 13]; // Includes the command byte
            data[0] = REG_MOTION_BURST; // first element in data is the register
            self.spi.read(&mut data).map_err(|_| "SPI read failed")?; //read data using HypedSpi trait

            // Parse the response data
            let mut cursor = &data[1..]; //slice data and use cursor to go through the data
            let _ = cursor.read_u8().unwrap(); // read each byte from cursor and assign it to variables
            let dr = cursor.read_u8().unwrap();
            let obs = cursor.read_u8().unwrap();
            let x = cursor.read_i16::<LittleEndian>().unwrap(); // x and y value is what we need and little endian is used to make sure bytes are ordered ith LSB first
            let y = cursor.read_i16::<LittleEndian>().unwrap(); // LittleEndian takes (advances) 2 bytes from cursor
            let quality = cursor.read_u8().unwrap();
            let raw_sum = cursor.read_u8().unwrap();
            let raw_max = cursor.read_u8().unwrap();
            let raw_min = cursor.read_u8().unwrap();
            let shutter_upper = cursor.read_u8().unwrap();
            let shutter_lower = cursor.read_u8().unwrap();

            if (dr & 0b1000_0000) != 0 && (quality >= 0x19 || shutter_upper != 0x1F) {
                return Ok((x, y)); // Return delta x and y if valid
            }

            // Delay before retrying
            sleep(Duration::from_millis(10));

        }

        Err("Timed out waiting for motion data")
    }

}


// the # implements the Serialize trait from the serde crate for the strcuts. this is apparently needed to convert strcut to json
#[derive(Serialize)]
struct Measurement {
    header: Header,
    payload: Payload,
}

#[derive(Serialize)]
struct Header {
    timestamp: u64,
    priority: u8,
}

#[derive(Serialize)]
struct Payload {
    x: i16,
    y: i16,
}


// Make main function with flexible error handling
pub fn main() -> Result<(), Box<dyn std::error::Error>> {


    let spi = HypedSPI::new() // spi implementation?
    let mut sensor = PMW3901::new(spi);


    // While loop implementation from python code
    loop {

        // Get motion data
        let (x, y) = sensor.get_motion()?;

        // Prepare measurement data
        let measurement = Measurement {
            header: Header {
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)? // Get time in secs from epoch as in the python code
                    .as_secs(),
                priority: 1,
            },
            payload: Payload { x, y },
        };

        // Serialize measurement data to JSON
        let measurement_data = serde_json::to_string(&measurement)?;

        // Print payload 
        //println!("{}", measurement_data);


        // delay
        sleep(Duration::from_millis(50));

    }
}    
