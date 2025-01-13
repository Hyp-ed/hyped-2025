use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn control_pneumatics<P: GpioOutputPin>(brake_gpio: P, lateral_suspension_gpio: P) -> ! {
    let p = embassy_stm32::init(Default::default());
    let pneumatics = Pneumatics::new(brake_gpio, lateral_suspension_gpio);
    let mut pneumatics_control = PneumaticsControl::new(pneumatics);

    loop {
        Timer::after(Duration::from_millis(100)).await;
    }
}
