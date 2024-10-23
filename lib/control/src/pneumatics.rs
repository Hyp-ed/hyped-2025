use hyped_core::types::DigitalSignal;

/// A solenoid is either on or off
#[derive(Debug, PartialEq)]
pub enum SolenoidSignal {
    Off,
    On,
}

/// Encodes the inverse behaviour of the pink and orange solenoids
#[derive(Debug, PartialEq)]
pub enum PinkOrangeState {
    PinkOnOrangeOff,
    PinkOffOrangeOn,
}

/// Represents the state of the pneumatics system.
///
/// The two blue solenoids are controlled together, the pink and orange
/// should be the inverse of each other (never the same). The other
/// colours are controlled independently from each other.
pub struct PneumaticsState {
    yellow: SolenoidSignal,
    pink_orange: PinkOrangeState,
    green: SolenoidSignal,
    blue: SolenoidSignal,
}

/// A struct that controls the state of the pneumatics system
pub struct PneumaticsControl {
    state: PneumaticsState,
}

pub struct PneumaticsOutput {
    pub yellow: DigitalSignal,
    pub pink: DigitalSignal,
    pub orange: DigitalSignal,
    pub green: DigitalSignal,
    pub blue_1: DigitalSignal,
    pub blue_2: DigitalSignal,
}

impl PneumaticsControl {
    pub fn new() -> PneumaticsControl {
        PneumaticsControl {
            // TODO: check that this initial state is correct
            state: PneumaticsState {
                yellow: SolenoidSignal::Off,
                pink_orange: PinkOrangeState::PinkOffOrangeOn,
                blue: SolenoidSignal::Off,
                green: SolenoidSignal::Off,
            },
        }
    }

    /// Get the state of the solenoids
    pub fn get_state(&self) -> &PneumaticsState {
        &self.state
    }

    /// Get the GPIO pin values corresponding to the current state
    pub fn get_gpio_out(&self) -> PneumaticsOutput {
        PneumaticsOutput {
            yellow: match self.state.yellow {
                SolenoidSignal::Off => DigitalSignal::Low,
                SolenoidSignal::On => DigitalSignal::High,
            },
            pink: match self.state.pink_orange {
                PinkOrangeState::PinkOnOrangeOff => DigitalSignal::High,
                PinkOrangeState::PinkOffOrangeOn => DigitalSignal::Low,
            },
            orange: match self.state.pink_orange {
                PinkOrangeState::PinkOnOrangeOff => DigitalSignal::Low,
                PinkOrangeState::PinkOffOrangeOn => DigitalSignal::High,
            },
            green: match self.state.green {
                SolenoidSignal::Off => DigitalSignal::Low,
                SolenoidSignal::On => DigitalSignal::High,
            },
            blue_1: match self.state.blue {
                SolenoidSignal::Off => DigitalSignal::Low,
                SolenoidSignal::On => DigitalSignal::High,
            },
            blue_2: match self.state.blue {
                SolenoidSignal::Off => DigitalSignal::Low,
                SolenoidSignal::On => DigitalSignal::High,
            },
        }
    }

    /// Set the state of the yellow solenoid
    pub fn set_yellow(&mut self, signal: SolenoidSignal) {
        self.state.yellow = signal;
    }

    /// Set the state of the pink and orange solenoids
    pub fn set_pink_orange(&mut self, state: PinkOrangeState) {
        self.state.pink_orange = state;
    }

    /// Set the state of the green solenoid
    pub fn set_green(&mut self, signal: SolenoidSignal) {
        self.state.green = signal;
    }

    /// Set the state of the blue solenoids
    pub fn set_blue(&mut self, signal: SolenoidSignal) {
        self.state.blue = signal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pneumatics_control() {
        let mut control = PneumaticsControl::new();

        control.set_yellow(SolenoidSignal::On);
        control.set_pink_orange(PinkOrangeState::PinkOnOrangeOff);
        control.set_green(SolenoidSignal::On);
        control.set_blue(SolenoidSignal::On);

        assert_eq!(control.get_state().yellow, SolenoidSignal::On);
        assert_eq!(
            control.get_state().pink_orange,
            PinkOrangeState::PinkOnOrangeOff
        );
        assert_eq!(control.get_state().green, SolenoidSignal::On);
        assert_eq!(control.get_state().blue, SolenoidSignal::On);

        let gpio_out = control.get_gpio_out();
        assert_eq!(gpio_out.yellow, DigitalSignal::High);
        assert_eq!(gpio_out.pink, DigitalSignal::High);
        assert_eq!(gpio_out.orange, DigitalSignal::Low);
        assert_eq!(gpio_out.green, DigitalSignal::High);
        assert_eq!(gpio_out.blue_1, DigitalSignal::High);
        assert_eq!(gpio_out.blue_2, DigitalSignal::High);
    }
}
