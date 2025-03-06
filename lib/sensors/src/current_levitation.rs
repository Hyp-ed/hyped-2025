use crate::SensorValueRange;
use hyped_adc::HypedAdc;

/// current_levitation implements the logic to read current from the ACHS-7121 current sensor using the
/// Hyped ADC trait.
///
/// Data sheet: https://docs.broadcom.com/doc/ACHS-712x-DS
pub struct CurrentLevitation<T: HypedAdc> {
    adc: T,
    calculate_bounds: fn(f32) -> SensorValueRange<f32>,
}

impl<T: HypedAdc> CurrentLevitation<T> {
    /// Create a new instance of the Current Levitation sensor
    pub fn new(adc: T) -> CurrentLevitation<T> {
        Self::new_with_bounds(adc, default_calculate_bounds)
    }

    pub fn new_with_bounds(
        adc: T,
        calculate_bounds: fn(f32) -> SensorValueRange<f32>,
    ) -> CurrentLevitation<T> {
        CurrentLevitation {
            adc,
            calculate_bounds,
        }
    }
    /// The ACHS-7121 has a current range of +- 10 A, sensitivity of 185 mV/A.
    /// Assuming we're supplying 5V to the sensor, our off-set is 2.5V in the output reading - note that this offset is given
    /// by the supply voltage divided by 2, so if you change the supply voltage, you'll have to change the offset that we subtract
    /// in the read function accordingly.
    pub fn read(&mut self) -> SensorValueRange<f32> {
        let adc_reading = self.adc.read_value() as f32;
        let resolution = self.adc.get_resolution() as f32;
        // Map the values we're reading in (currently 0-4096) into our voltage range
        let voltage = ((adc_reading) / (resolution)) * V_REF;
        (self.calculate_bounds)((voltage - OFFSET) / SENSITIVITY)
    }
}

pub fn default_calculate_bounds(value: f32) -> SensorValueRange<f32> {
    if value <= MIN_AMPS || value >= MAX_AMPS {
        SensorValueRange::Critical(value)
    } else if value <= WARN_AMPS_LOW || value >= WARN_AMPS_HIGH {
        SensorValueRange::Warning(value)
    } else {
        SensorValueRange::Safe(value)
    }
}

const OFFSET: f32 = 2.5;
const SENSITIVITY: f32 = 0.185;
const MIN_AMPS: f32 = 10.0;
const MAX_AMPS: f32 = -10.0;
const WARN_AMPS_LOW: f32 = -8.0;
const WARN_AMPS_HIGH: f32 = 8.0;
const V_REF: f32 = 5.0;
