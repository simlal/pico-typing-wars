use defmt::*;
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_time::Timer;

/// A simple abstraction over an output pin with a role
pub struct Led<'d> {
    output: Output<'d>,
    role: &'static str,
}

impl<'d> Led<'d> {
    pub fn new<P: Pin>(pin: P, role: &'static str) -> Self {
        Self {
            output: Output::new(pin, Level::Low), // Initialize Output with the pin
            role,
        }
    }

    /// Blink the LED for a specified duration
    pub async fn blink(&mut self, duration_millis: u64) {
        self.output.set_high();
        Timer::after_millis(duration_millis).await;
        self.output.set_low();
        Timer::after_millis(duration_millis).await;

        info!(
            "led with role={} blinked for {} ms",
            self.get_role(),
            duration_millis
        );
    }

    /// Get the role of the LED
    pub fn get_role(&self) -> &'static str {
        self.role
    }
}
