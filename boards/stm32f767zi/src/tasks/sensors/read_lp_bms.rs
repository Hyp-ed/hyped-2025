use crate::{board_state::THIS_BOARD, tasks::can::send::CAN_SEND};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Sender};
use embassy_time::{Duration, Timer};
use hyped_communications::{data::CanData, measurements::MeasurementReading, messages::CanMessage};
use hyped_core::{
    config::{MeasurementId, SENSORS_CONFIG},
};
use hyped_sensors::lp_bms::{Bms, BatteryData};
use hyped_can::{HypedCanTx, HypedCanRx};
use defmt::warn;

/// Task to periodically read data from the LP BMS and send it over CAN
#[embassy_executor::task]
pub async fn read_lp_bms<T>(
    bms: &'static mut Bms<'static, T>,
    measurement_id: MeasurementId,
    latest_bms_sender: Sender<'static, CriticalSectionRawMutex, Option<BatteryData>, 1>,
) -> !
where
    T: HypedCanTx + HypedCanRx,
{
    let can_sender = CAN_SEND.sender();

    loop {
        match bms.read_battery_data() {
            Ok(battery_data) => {
                latest_bms_sender.send(Some(battery_data.clone()));

                let board = *THIS_BOARD.get().await;

                // Send all BMS information over CAN
                can_sender
                    .send(CanMessage::MeasurementReading(MeasurementReading::new(
                        CanData::F32(battery_data.voltage),
                        board,
                        measurement_id,
                    )))
                    .await;

                can_sender
                    .send(CanMessage::MeasurementReading(MeasurementReading::new(
                        CanData::F32(battery_data.current),
                        board,
                        measurement_id,
                    )))
                    .await;

                can_sender
                    .send(CanMessage::MeasurementReading(MeasurementReading::new(
                        CanData::U16(battery_data.max_cell_mv),
                        board,
                        measurement_id,
                    )))
                    .await;

                can_sender
                    .send(CanMessage::MeasurementReading(MeasurementReading::new(
                        CanData::U16(battery_data.min_cell_mv),
                        board,
                        measurement_id,
                    )))
                    .await;

                // Send each temperature
                for (i, temp) in battery_data.temperatures_c.iter().enumerate() {
                    can_sender
                        .send(CanMessage::MeasurementReading(MeasurementReading::new(
                            CanData::I16(*temp),
                            board,
                            measurement_id, 
                        )))
                        .await;
                }

                // Send each cell voltage
                for (i, cell_mv) in battery_data.cell_voltages_mv.iter().enumerate() {
                    can_sender
                        .send(CanMessage::MeasurementReading(MeasurementReading::new(
                            CanData::U16(*cell_mv),
                            board,
                            measurement_id,
                        )))
                        .await;
                }
            }
            Err(e) => {
                warn!("Failed to read BMS data: {:?}", e);
                latest_bms_sender.send(None);
            }
        }

        Timer::after(Duration::from_hz(
            SENSORS_CONFIG.sensors.lp_bms.update_frequency as u64,
        ))
        .await;
    }
}