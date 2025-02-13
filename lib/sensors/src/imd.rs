use hyped_can::{CanError, HypedCan, HypedCanFrame};

// CAN Protocol Reference Manual: https://www.sensata.com/sites/default/files/a/sensata-sendyne-sim100mod-insulation-monitoring-device-protocol-manual.pdf
// Datasheet: https://www.sensata.com/sites/default/files/a/sensata-el-sim100-mod-datasheet.pdf
pub struct Imd<'a, T: HypedCan> {
    can: &'a mut T,
    resistance_positive: u16, // not entirely sure if i should include these three fields...
    resistance_negative: u16,
    isolation_status: u8,
}

impl<'a, T: HypedCan> Imd<'a, T> {
    /// Create a new instance of the IMD.
    pub fn new(can: &'a mut T) -> Self {
        Self {
            can,
            resistance_positive: 0,
            resistance_negative: 0,
            isolation_status: 0,
        } // may need to do error handling
    }

    pub fn update_values(&mut self) -> Result<(), ImdError> {
        let frame = HypedCanFrame {
            can_id: CAN_EFF_FLAG | IMD_REQUEST_DATA_CAN_ID,
            data: [REQUEST_ISOLATION_RESISTANCES, 0, 0, 0, 0, 0, 0, 0],
        };

        match HypedCan::write_frame(self.can, &frame) {
            Ok(_) => Ok(()),
            Err(e) => Err(ImdError::CanError(e)),
        }
    }

    pub fn process_message(&mut self, frame: HypedCanFrame) {
        self.isolation_status = frame.data[1] & 3;
        self.resistance_positive = (frame.data[2] as u16) << 8 | (frame.data[3] as u16);
        self.resistance_negative = (frame.data[5] as u16) << 8 | (frame.data[6] as u16);
    }
}

pub enum ImdError {
    CanError(CanError),
}

const CAN_EFF_FLAG: u32 = 0x80000000;
const IMD_REQUEST_DATA_CAN_ID: u32 = 0xA100101;
const IMD_RESPONSE_CAN_ID: u32 = 0xA100100;
const REQUEST_ISOLATION_RESISTANCES: u8 = 0xE1;
const REQUEST_ISOLATION_STATE: u8 = 0xE0;
