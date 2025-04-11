#![no_std]
#![no_main]

use core::cell::RefCell;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    can::{
        filter::Mask32, Can, Fifo, Rx0InterruptHandler, Rx1InterruptHandler, SceInterruptHandler,
        TxInterruptHandler,
    },
    i2c::I2c,
    mode::Blocking,
    peripherals::CAN1,
    time::Hertz,
};
use embassy_sync::{
    blocking_mutex::{
        raw::{CriticalSectionRawMutex, NoopRawMutex},
        Mutex,
    },
    watch::Watch,
};
use embassy_time::{Duration, Timer};
use hyped_boards_stm32f767zi::tasks::{
    can::{receive::can_receiver, send::can_sender},
    sensors::read_temperature::read_temperature,
    state_machine::state_updater,
};
use hyped_communications::boards::Board;
use hyped_core::config::MeasurementId;
use hyped_sensors::SensorValueRange::{self, Critical, Safe, Warning};
use hyped_state_machine::states::State;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// The current state of the state machine.
pub static CURRENT_STATE: Watch<CriticalSectionRawMutex, State, 1> = Watch::new();

/// Used to keep the latest temperature sensor value.
pub static LATEST_TEMPERATURE_READING: Watch<
    CriticalSectionRawMutex,
    Option<SensorValueRange<f32>>,
    1,
> = Watch::new();

static BOARD: Board = Board::TemperatureTester;
pub static EMERGENCY: Watch<CriticalSectionRawMutex, bool, 1> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());

    // Initialize the I2C bus and store it in a static cell so that it can be accessed from the tasks.
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));

    let mut can = Can::new(p.CAN1, p.PD0, p.PD1, Irqs);
    can.modify_filters()
        .enable_bank(0, Fifo::Fifo0, Mask32::accept_all());
    can.modify_config().set_bitrate(500_000);
    can.enable().await;
    let (can_tx, can_rx) = can.split();
    spawner.must_spawn(can_receiver(can_rx, EMERGENCY.sender()));
    spawner.must_spawn(can_sender(can_tx));

    Timer::after(Duration::from_secs(2)).await;

    spawner.must_spawn(state_updater(CURRENT_STATE.sender()));

    spawner.must_spawn(read_temperature(
        i2c_bus,
        BOARD,
        MeasurementId::Thermistor1,
        LATEST_TEMPERATURE_READING.sender(),
    ));

    let mut temp_reading_receiver = LATEST_TEMPERATURE_READING.receiver().unwrap();

    loop {
        if let Some(reading) = temp_reading_receiver.changed().await {
            match reading {
                Safe(temp) => {
                    defmt::info!("Temperature: {}°C (safe)", temp);
                }
                Warning(temp) => {
                    defmt::warn!("Temperature: {}°C (warning)", temp);
                }
                Critical(temp) => {
                    defmt::error!("Temperature: {}°C (critical)", temp);
                }
            }
        }
    }
}
