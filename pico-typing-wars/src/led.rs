use defmt::Format;
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
    }

    pub fn role(&self) -> &str {
        self.role
    }
}

// Helper func to defmt Level enum
fn level_to_str(level: &Level) -> &str {
    match level {
        Level::Low => "Low",
        Level::High => "High",
    }
}

// Formatting traits

impl<'a> defmt::Format for Led<'a> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Led {{ role: {}, output_level={} }}",
            self.role(),
            level_to_str(&self.output.get_output_level()),
        )
    }
}
