#![no_std]
#![no_main]

use defmt::info;
use advanced_pid::{prelude::*, Pid, PidGain};
use embassy_executor::Spawner;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use embassy_stm32::time::hz;
use embassy_time::Instant;

use {defmt_rtt as _, panic_probe as _};

const MAX_VOLTAGE: f32 = 500.0; // TODOLater
const MAX_CURRENT: f32 = 500.0; // TODOLater
const TARGET_HEIGHT: f32 = 10.0; // TODOLater to be determined by levitation

const GAIN_HEIGHT: PidGain = PidGain{ 
    // TODOLater to be determined by levitation
    kp: 1.0,
    ki: 0.05,
    kd: 0.005,
};

const GAIN_CURRENT: PidGain = PidGain{ 
    // TODOLater determined by levitation
    kp: 1.1,
    ki: 0.12,
    kd: 0.05,
};


#[embassy_executor::main] 
async fn main(_spawner: Spawner) {

    let mut pid_height = Pid::new(GAIN_HEIGHT.into());
    let mut pid_current = Pid::new(GAIN_CURRENT.into());

    let p = embassy_stm32::init(Default::default());

    let green_light = PwmPin::new_ch2(p.PB3, OutputType::PushPull);

    let mut pwm = SimplePwm::new(p.TIM2, None, Some(green_light), None, None, hz(2000), CountingMode::EdgeAlignedUp);
    pwm.enable(Channel::Ch2);

    let max_duty = pwm.get_max_duty() as f32;

    let mut time_start = Instant::now().as_micros() as f32;

    loop {
        
        let actual_height = 0.7; // TODOLater we'll get that from a sensor
        
        let dt = (Instant::now().as_micros() as f32) - time_start;

        let output_height_pid = pid_height.update(TARGET_HEIGHT, actual_height, dt);

        let target_current = (actual_height + output_height_pid).min(MAX_CURRENT);

        let actual_current = 1.0; // TODOLater we'll get that from a sensor

        let output_current_pid = pid_current.update(target_current, actual_current, dt);

        let required_voltage = (actual_current + output_height_pid).min(MAX_VOLTAGE);

        let duty_cycle = max_duty * (required_voltage / MAX_VOLTAGE);

        pwm.set_duty(Channel::Ch2, duty_cycle as u32);    

        time_start = Instant::now().as_micros() as f32;

        info!("height = {}", actual_height);
        info!("v_out = {}", required_voltage);
    }
}
