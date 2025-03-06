use hyped_can::{CanError, HypedCanFrame, HypedCanTx};

// CAN Protocol Reference Manual: https://www.sensata.com/sites/default/files/a/sensata-sendyne-sim100mod-insulation-monitoring-device-protocol-manual.pdf
// Datasheet: https://www.sensata.com/sites/default/files/a/sensata-el-sim100-mod-datasheet.pdf
pub struct Imd<'a, T: HypedCanTx> {
    can_tx: &'a mut T,
    resistance_positive: u16,
    resistance_negative: u16,
    isolation_status: u8,
}

impl<'a, T: HypedCanTx> Imd<'a, T> {
    /// Create a new instance of the IMD.
    pub fn new(can_tx: &'a mut T) -> Self {
        Self {
            can_tx,
            resistance_positive: 0,
            resistance_negative: 0,
            isolation_status: 0,
        }
    }

    pub fn update_values(&mut self) -> Result<(), ImdError> {
        let frame = HypedCanFrame {
            can_id: CAN_EFF_FLAG | IMD_REQUEST_DATA_CAN_ID,
            data: [REQUEST_ISOLATION_RESISTANCES, 0, 0, 0, 0, 0, 0, 0],
        };

        match HypedCanTx::write_frame(self.can_tx, &frame) {
            Ok(_) => Ok(()),
            Err(e) => Err(ImdError::CanError(e)),
        }
    }

    pub fn process_message(&mut self, frame: HypedCanFrame) {
        if frame.can_id == IMD_RETURN_DATA_CAN_ID {
            self.isolation_status = frame.data[1] & 3;
            self.resistance_positive = (frame.data[2] as u16) << 8 | (frame.data[3] as u16);
            self.resistance_negative = (frame.data[5] as u16) << 8 | (frame.data[6] as u16);
        }
    }

    pub fn get_resistance_positive(&self) -> u16 {
        self.resistance_positive
    }

    pub fn get_resistance_negative(&self) -> u16 {
        self.resistance_negative
    }

    pub fn get_isolation_status(&self) -> u8 {
        self.isolation_status
    }
}

pub enum ImdError {
    CanError(CanError),
}

const CAN_EFF_FLAG: u32 = 0x80000000;
const IMD_REQUEST_DATA_CAN_ID: u32 = 0xA100101;
const IMD_RETURN_DATA_CAN_ID: u32 = 0xA100100;
const REQUEST_ISOLATION_RESISTANCES: u8 = 0xE1;
