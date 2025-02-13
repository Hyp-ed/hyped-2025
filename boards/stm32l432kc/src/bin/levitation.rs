#![no_std]
#![no_main]

use core::f32;
use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::hz;
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::timer::Channel;
use embassy_time::Instant;

use {defmt_rtt as _, panic_probe as _};

trait PidController {
    fn new(config: PidGain) -> Self;
    fn update(&mut self, set_point: f32, actual: f32, dt: f32, filter_constant: f32) -> f32;
}

#[derive(Debug, Clone)]
struct PidGain {
    kp: f32,
    ki: f32,
    kd: f32,
    p_reference_gain: f32,
    d_reference_gain: f32,
}
/// `Pid` is a structure that implements the [`PidController`] trait.
#[derive(Debug, Clone)]
pub struct Pid {
    config: PidGain,
    i_term: f32,
    pre_error: f32,
    current_filter: f32,
    previous_filter: f32,
}

impl PidController for Pid {
    /// Updates the `Pid` controller with the specified set point, actual value, and time delta.
    /// Implements a low pass filter onto the derivative term.
    /// Returns the controller output.
    fn new(config: PidGain) -> Self {
        Self {
            config,
            i_term: 0.0,
            pre_error: core::f32::NAN,
            current_filter: 0.0,
            previous_filter: 0.0,
        }
    }
    fn update(&mut self, set_point: f32, actual: f32, dt: f32, filter_constant: f32) -> f32 {
        let p_error = (set_point * self.config.p_reference_gain) - actual;
        let i_error = set_point - actual;
        let d_error = (set_point * self.config.d_reference_gain) - actual;
        self.i_term += i_error * dt;
        let d_term = if self.pre_error.is_nan() {
            0.0
        } else {
            let error_change = d_error - self.pre_error;
            self.current_filter =
                (filter_constant * self.previous_filter) + ((1.0 - filter_constant) * error_change);
            self.previous_filter = self.current_filter;
            self.current_filter / dt
        };
        let output =
            self.config.kp * p_error + self.config.ki * self.i_term + self.config.kd * d_term;
        self.pre_error = d_error;
        output // TOMaybeDO could restrict output by min value here instead of using .min()
    }
}

trait PiController {
    fn new(config: PiGain) -> Self;
    fn update(&mut self, set_point: f32, actual: f32, dt: f32) -> f32;
}

#[derive(Debug, Clone)]
struct PiGain {
    kp: f32,
    ki: f32,
}

#[derive(Debug, Clone)]
pub struct Pi {
    config: PiGain,
    i_term: f32,
    pre_error: f32,
}

impl PiController for Pi {
    /// Creates a new `Pi` with the specified configuration.
    fn new(config: PiGain) -> Self {
        Self {
            config,
            i_term: 0.0,
            pre_error: f32::NAN,
        }
    }
    /// Updates the `Pi` controller, ignoring D.
    /// Returns the controller output.
    fn update(&mut self, set_point: f32, actual: f32, dt: f32) -> f32 {
        let error = set_point - actual;
        self.i_term += error * dt;
        let output = self.config.kp * error + self.config.ki * self.i_term; // removed the derivative term
        self.pre_error = error;
        output // TOMaybeDO could restrict output by min value here instead of using .min()
    }
}

const MAX_VOLTAGE: f32 = 500.0;
const MAX_CURRENT: f32 = 5.0; // TODOLater check with lev
const TARGET_HEIGHT: f32 = 15.0; // mm
const LOW_PASS_FILTER_CONSTANT_HEIGHT: f32 = 0.2; // TO TUNE. A number between 0 and 1

const GAIN_HEIGHT: PidGain = PidGain {
    kp: 29497.7537305353,
    ki: 262105.664028736,
    kd: 815.114265452965,
    p_reference_gain: 0.873451984, 
    d_reference_gain: 0.705728005, 
};

const GAIN_CURRENT: PiGain = PiGain {
    kp: 1000,
    ki: 5_000_000,
};

const GAIN_VOLTAGE: PiGain = PiGain {
    kp: 50,
    ki: 10_000_000,
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

        let target_current = (pid_height.update(
            TARGET_HEIGHT,
            actual_height,
            dt,
            LOW_PASS_FILTER_CONSTANT_HEIGHT,
        ))
        .min(MAX_CURRENT); // takes in height -> outputs current target (within boundaries) and uses low pass filter on derivative term

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
