#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::hz;
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use embassy_time::Instant;
use hyped_control::pi_controller::{Pi, PiController, PiGain};
use hyped_control::pid_controller::{Pid, PidController, PidGain};

use {defmt_rtt as _, panic_probe as _};

const MAX_VOLTAGE: f32 = 500.0;
const MAX_CURRENT: f32 = 5.0; // TODOLater check with lev
const TARGET_HEIGHT: f32 = 15.0; // mm
const FILTER_CONSTANT: f32 = 0.2; // TO TUNE. A number between 0 and 1

const GAIN_HEIGHT: PidGain = PidGain {
    kp: 29497.7537305353,
    ki: 262105.664028736,
    kd: 815.114265452965,
    p_reference_gain: 0.873451984,
    d_reference_gain: 0.705728005,
};

const GAIN_CURRENT: PiGain = PiGain {
    kp: 1000.0,
    ki: 5_000_000.0,
};

const GAIN_VOLTAGE: PiGain = PiGain {
    kp: 50.0,
    ki: 10_000_000.0,
};

/*
For the lev control, we need to chain a PID controller with 2 PI controllers and output a PWM signal. The PID takes in
a height and outputs a current, the first PI takes in a current and outputs a voltage, which we the use to calculate
duty cycle of the PWM signal, which is the third PI.

Outside the loop, we initialise the PID/PIs and the board pins for PWM. We also need max_duty to represent
our voltage output as a fraction of the max duty cycle. In the loop, there is a timer which measures the time between
the signal being set and us taking readings from the sensors as part of the control loop. The PID/PI calculations are
then performed, and the duty cycle is set, and timer restarted.
*/

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut pid_height = Pid::new(GAIN_HEIGHT.into());
    let mut pi_current = Pi::new(GAIN_CURRENT.into());
    let mut pi_voltage = Pi::new(GAIN_VOLTAGE.into());

    let p = embassy_stm32::init(Default::default());

    let green_light = PwmPin::new_ch2(p.PB3, OutputType::PushPull); // TODOLater change to actual pin (this is just for testing)

    let mut pwm = SimplePwm::new(
        p.TIM2,
        None,
        Some(green_light),
        None,
        None,
        hz(2000),
        CountingMode::EdgeAlignedUp,
    ); // TODOLater change to actual pin (this is just for testing)

    let max_duty = pwm.get_max_duty() as f32;

    pwm.enable(Channel::Ch2);

    let mut time_start = Instant::now().as_micros() as f32;

    loop {
        let actual_height = 0.7; // TODOLater we'll get that from a sensor

        let actual_current = 1.0; // TODOLater we'll get that from a sensor

        let actual_voltage = 0.8; // TODOLater we'll get that from a sensor

        let dt = (Instant::now().as_micros() as f32) - time_start; // this gets the timeframe between the last change in the pwm signal for the PID

        let target_current =
            (pid_height.update(TARGET_HEIGHT, actual_height, dt, FILTER_CONSTANT)).min(MAX_CURRENT); // takes in height -> outputs current target (within boundaries) and uses low pass filter on derivative term

        let target_voltage =
            (pi_current.update(target_current, actual_current, dt)).min(MAX_VOLTAGE); // takes in current -> outputs voltage (within boundaries) and ignores derivative term from output

        let duty_cycle = pi_voltage.update(target_voltage, actual_voltage, dt); // TODOLater include .min(max_duty) if max_duty given

        let duty_cycle = duty_cycle * max_duty; // the duty cycle ranges from 0 to max_duty, so what fraction of that do we need
                                                // probably TODOLater update how this is calculated

        pwm.set_duty(Channel::Ch2, duty_cycle as u32);

        time_start = Instant::now().as_micros() as f32;

        info!("height = {}", actual_height);
        info!("v_out = {}", target_voltage);
        info!("duty_cycle = {}", duty_cycle);
    }
}
