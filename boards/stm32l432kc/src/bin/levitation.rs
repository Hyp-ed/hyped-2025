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


/*
For the lev control, we need to chain 2 PIDs together and output a PWM signal. The first one takes in a height and 
outputs a current, the second one takes in a current and outputs a voltage, which we the use to calculate duty cycle of
the PWM signal. Outside the loop, we initialise the pids and the board pins for PWM. We also need max_duty to represent
our voltage output as a fraction of the max duty cycle. In the loop, there is a timer which measures the time between 
the signal being set and us taking readings from the sensors as part of the PID calculations. The PID calculations are 
then performed, and the duty cycle is set, and timer restarted.
*/

#[embassy_executor::main] 
async fn main(_spawner: Spawner) {

    let mut pid_height = Pid::new(GAIN_HEIGHT.into());
    let mut pid_current = Pid::new(GAIN_CURRENT.into());

    let p = embassy_stm32::init(Default::default());

    let green_light = PwmPin::new_ch2(p.PB3, OutputType::PushPull); // TODOLater change to actual pin (this is just for testing)

    let mut pwm = SimplePwm::new(p.TIM2, None, Some(green_light), None, None, hz(2000), CountingMode::EdgeAlignedUp); // TODOLater change to actual pin (this is just for testing)
    
    let max_duty = pwm.get_max_duty() as f32;
    
    pwm.enable(Channel::Ch2);

    let mut time_start = Instant::now().as_micros() as f32;

    loop {
        
        let actual_height = 0.7; // TODOLater we'll get that from a sensor

        let actual_current = 1.0; // TODOLater we'll get that from a sensor        

        let dt = (Instant::now().as_micros() as f32) - time_start; // this gets the timeframe between the last change in the pwm signal for the PID

        let target_current = (pid_height.update(TARGET_HEIGHT, actual_height, dt)).min(MAX_CURRENT); // takes in height -> outputs current target (within boundaries)

        let required_voltage = (pid_current.update(target_current, actual_current, dt)).min(MAX_VOLTAGE); // takes in current -> outputs voltage (within boundaries)

        let duty_cycle = max_duty * (required_voltage / MAX_VOLTAGE); // the duty cycle ranges from 0 to max_duty, so what fraction of that do we need
                                                                      // probably TODOLater update how this is calculated

        pwm.set_duty(Channel::Ch2, duty_cycle as u32);

        time_start = Instant::now().as_micros() as f32;

        info!("height = {}", actual_height);
        info!("v_out = {}", required_voltage);
    }
}
