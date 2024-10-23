use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn control_pneumatics() -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut pneumatics_control = PneumaticsControl::new();

    loop {
        Timer::after(Duration::from_millis(100)).await;
    }
}
