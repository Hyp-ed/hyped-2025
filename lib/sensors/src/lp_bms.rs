/// Driver for the TinyBMS s516 30A Battery Management System using CAN.
/// Used to monitor battery status and health.

use defmt::Format;
use crate::SensorValueRange;
use hyped_can::{HypedCanTx, HypedCanRx, HypedCanFrame, CanError};

/// Represents the parsed status from the BMS.
#[derive(Debug, PartialEq, Clone, Format)]
pub struct BatteryData {
    pub voltage: f32,
    pub current: f32,
    pub max_cell_mv: u16,
    pub min_cell_mv: u16,
    pub temperatures_c: [i16; 3], // internal, external1, external2
    pub cell_voltages_mv: heapless::Vec<u16, 32>,
}


pub struct Bms<'a, T: HypedCanTx + HypedCanRx + 'a> {
    can: &'a mut T,
}

impl<'a, T: HypedCanTx + HypedCanRx> Bms<'a, T> {
    const NODE_ID: u8 = 0x01;
    const REQUEST_ID: u32 = 0x400 | Self::NODE_ID as u32;
    const RESPONSE_ID: u32 = 0x500 | Self::NODE_ID as u32;

    fn send_simple_request(&mut self, cmd: u8) -> Result<(), CanError> {
        let frame = HypedCanFrame::new(Self::REQUEST_ID, [cmd, 0, 0, 0, 0, 0, 0, 0]);
        self.can.write_frame(&frame)
    }

    fn read_response(&mut self, expected_cmd: u8) -> Result<[u8; 8], CanError> {
        loop {
            let envelope = self.can.read_frame()?;
            if envelope.frame.can_id == Self::RESPONSE_ID && envelope.frame.data[1] == expected_cmd {
                return Ok(envelope.frame.data);
            }
        }
    }

    pub fn read_voltage(&mut self) -> Result<f32, CanError> {
        self.send_simple_request(0x14)?;
        let data = self.read_response(0x14)?;
        Ok(f32::from_le_bytes([data[2], data[3], data[4], data[5]]))
    }

    pub fn read_current(&mut self) -> Result<f32, CanError> {
        self.send_simple_request(0x15)?;
        let data = self.read_response(0x15)?;
        Ok(f32::from_le_bytes([data[2], data[3], data[4], data[5]]))
    }

    pub fn read_max_cell_voltage(&mut self) -> Result<u16, CanError> {
        self.send_simple_request(0x16)?;
        let data = self.read_response(0x16)?;
        Ok(u16::from_le_bytes([data[2], data[3]]))
    }

    pub fn read_min_cell_voltage(&mut self) -> Result<u16, CanError> {
        self.send_simple_request(0x17)?;
        let data = self.read_response(0x17)?;
        Ok(u16::from_le_bytes([data[2], data[3]]))
    }

    pub fn read_temperatures(&mut self) -> Result<[i16; 3], CanError> {
        self.send_simple_request(0x1B)?;
        let mut temps = [0i16; 3];
        for i in 0..3 {
            let data = self.read_response(0x1B)?;
            temps[i] = i16::from_le_bytes([data[2], data[3]]);
        }
        Ok(temps)
    }

    pub fn read_cell_voltages(&mut self) -> Result<heapless::Vec<u16, 32>, CanError> {
        self.send_simple_request(0x1C)?;
        let mut voltages = heapless::Vec::<u16, 32>::new();
        loop {
            match self.can.read_frame() {
                Ok(envelope) if envelope.frame.can_id == Self::RESPONSE_ID && envelope.frame.data[1] == 0x1C => {
                    let data = envelope.frame.data;
                    let val = u16::from_le_bytes([data[2], data[3]]);
                    voltages.push(val).map_err(|_| CanError::BufferOverflow)?;
                }
                Ok(_) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(voltages)
    }
}

impl<'a, T: HypedCanTx + HypedCanRx> Bms<'a, T> {
    pub fn new(can: &'a mut T) -> Self {
        Bms { can }
    }

    pub fn read_battery_data(&mut self) -> Result<BatteryData, CanError> {
        let voltage = self.read_voltage()?;
        let current = self.read_current()?;
        let max_cell_mv = self.read_max_cell_voltage()?;
        let min_cell_mv = self.read_min_cell_voltage()?;
        let temperatures_c = self.read_temperatures()?;
        let cell_voltages_mv = self.read_cell_voltages()?;

        Ok(BatteryData {
            voltage,
            current,
            max_cell_mv,
            min_cell_mv,
            temperatures_c,
            cell_voltages_mv,
        })
    }
}