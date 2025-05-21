#![no_std]
#![no_main]

use core::cell::RefCell;
use defmt_rtt as _;
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
use hyped_boards_stm32f767zi::{
    board_state::{CURRENT_STATE, EMERGENCY, THIS_BOARD},
    default_can_config,
    tasks::{
        can::{
            board_heartbeat::{heartbeat_listener, send_heartbeat},
            receive::can_receiver,
            send::can_sender,
        },
        sensors::read_temperature::read_temperature,
        state_machine::state_updater,
    },
};
use hyped_communications::boards::Board;
use hyped_core::config::MeasurementId;
use hyped_sensors::SensorValueRange::{self, Critical, Safe, Warning};
use hyped_state_machine::states::State;
use panic_probe as _;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    CAN1_RX0 => Rx0InterruptHandler<CAN1>;
    CAN1_RX1 => Rx1InterruptHandler<CAN1>;
    CAN1_SCE => SceInterruptHandler<CAN1>;
    CAN1_TX => TxInterruptHandler<CAN1>;
});

type I2c1Bus = Mutex<NoopRawMutex, RefCell<I2c<'static, Blocking>>>;

/// Used to keep the latest temperature sensor value.
pub static LATEST_TEMPERATURE_READING: Watch<
    CriticalSectionRawMutex,
    Option<SensorValueRange<f32>>,
    1,
> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    THIS_BOARD
        .init(Board::TemperatureTester)
        .expect("Failed to initialize board");

    let p = embassy_stm32::init(Default::default());
    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), Default::default());

    // Initialize the I2C bus and store it in a static cell so that it can be accessed from the tasks.
    static I2C_BUS: StaticCell<I2c1Bus> = StaticCell::new();
    let i2c_bus = I2C_BUS.init(Mutex::new(RefCell::new(i2c)));

    let mut can = Can::new(p.CAN1, p.PD0, p.PD1, Irqs);
    default_can_config!(can);
    can.enable().await;
    let (can_tx, can_rx) = can.split();
    spawner.must_spawn(can_receiver(can_rx));
    spawner.must_spawn(can_sender(can_tx));

    spawner.must_spawn(emergency_handler());
    spawner.must_spawn(send_heartbeat(Board::Telemetry));
    spawner.must_spawn(heartbeat_listener(Board::Telemetry));
    spawner.must_spawn(state_updater());

    spawner.must_spawn(read_temperature(
        i2c_bus,
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

#[embassy_executor::task]
async fn emergency_handler() {
    let current_state_sender = CURRENT_STATE.sender();

    loop {
        // All main loops should have logic to handle an emergency signal...
        if EMERGENCY.receiver().unwrap().get().await {
            defmt::error!("Emergency signal received! Cleaning up...");
            // ... and take appropriate action
            current_state_sender.send(State::Emergency);
            // Wait for the emergency signal to be sent
            Timer::after(Duration::from_secs(1)).await;
            panic!("Terminating due to emergency signal!");
        }
    }
}
