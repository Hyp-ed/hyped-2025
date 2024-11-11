#![no_std]
#![no_main]

use advanced_pid::{prelude::*, Pid, PidGain};
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Pull};
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
pub async fn control_loop(mut pid_height: Pid, mut pid_current: Pid) -> ! {
    loop {
        let target_height = 0.14; // to be determined by levitation

        let mut actual_height = 0.7; // we'll get that from a sensor

        let mut dt = 1.0; // time read from timer

        let output_height_pid = pid_height.update(target_height, actual_height, dt);

        let target_current = actual_height + output_height_pid;

        let mut actual_current = 1.0; // we'll get that from a sensor

        let output_current_pid = pid_current.update(target_current, actual_current, dt);

        let required_voltage = actual_current + output_current_pid;

        //pwm_set_pwm()
        defmt::info!("{}", required_voltage);

        Timer::after(Duration::from_millis(10)).await;    
    }
    
    
    
}

#[embassy_executor::main] 
async fn main(spawner: Spawner) {
    
    let gain_height = PidGain{ 
        // to be determined by levitation
        kp: 1.0,
        ki: 0.05,
        kd: 0.005,
    };

    let gain_current = PidGain{ 
        // to be determined by levitation
        kp: 1.1,
        ki: 0.12,
        kd: 0.05,
    };

    let mut pid_height = Pid::new(gain_height.into());
    let mut pid_current = Pid::new(gain_current.into());

    spawner.spawn(control_loop(pid_height, pid_current));
}
