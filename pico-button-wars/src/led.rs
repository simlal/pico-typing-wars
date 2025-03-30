use defmt::{debug, Format};
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_time::{Duration, Timer};

use crate::common::LevelToStr;

#[derive(PartialEq, Eq, Format, Clone, Copy)]
pub enum LedRole {
    Onboard,
    Player1,
    Player2,
}

// A simple abstraction over an output pin with a role
pub struct Led<'a> {
    output: Output<'a>,
    role: LedRole,
}

impl Led<'_> {
    pub fn new<P: Pin>(pin: P, role: LedRole) -> Self {
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
}

// Formatting traits
impl LevelToStr for Led<'_> {}
impl Format for Led<'_> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Led {{ role: {}, output_level={} }}",
            self.role,
            self.level_to_str(&self.output.get_output_level()),
        )
    }
}

// ****** GameState Leds funcs ****** //

// Non concurrent flashing pattern
pub async fn waiting_state_leds(leds: &'_ mut [Led<'_>; 3]) {
    debug!("Flashing leds inside waiting_state_leds(): {}", leds);

    // First pattern, fast flashes
    let mut duration = Duration::from_millis(200);
    for led in leds.iter_mut() {
        led.flash_pattern(duration, 2).await;
    }

    // Chasing second pattern
    let mut i: usize = 0;
    let mut passes: usize = 0;
    let max_circles: usize = 10;
    duration = Duration::from_millis(100);
    loop {
        leds[0].flash_pattern(duration, 1).await;
        leds[1].flash_pattern(duration, 1).await;
        leds[2].flash_pattern(duration, 1).await;

        // PERF: Need better pattern or algo
        i += 1;
        passes += 1;
        // Ramp up at each quarter
        if i > 2 {
            duration = duration.checked_sub(duration / 2).unwrap();
            i = 0;
        }

        if passes == max_circles {
            break;
        }
    }
    Timer::after_millis(500).await;
}
