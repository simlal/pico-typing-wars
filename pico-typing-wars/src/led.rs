use defmt::*;
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_time::{Duration, Timer};

/// A simple abstraction over an output pin with a role
pub struct Led<'a> {
    output: Output<'a>,
    role: &'a str,
}

impl<'a> Led<'a> {
    pub fn new<P: Pin>(pin: P, role: &'a str) -> Self {
        Self {
            output: Output::new(pin, Level::Low), // Initialize Output with the pin
            role,
        }
    }

    /// Blink the LED for a specified duration
    pub async fn flash_pattern(&mut self, blink_duration: Duration, repeats: usize) {
        for _ in 0..repeats {
            // Make sure we are off before flashing
            if self.output.is_set_high() {
                self.output.set_low();
            }
            self.output.set_high();
            Timer::after(blink_duration).await;
            self.output.set_low();
            Timer::after(blink_duration).await;
        }
        debug!(
            "led={} flashed {} times with a blink-duration={} ms",
            self.get_role(),
            repeats,
            blink_duration
        );
    }

    pub fn get_role(&self) -> &str {
        self.role
    }
}
