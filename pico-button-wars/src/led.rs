use defmt::{debug, error, info, Format};
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_time::{Duration, Instant, Timer};

use crate::{
    button::ButtonRole,
    common::{LevelToStr, SimpleRngU64},
};

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

    pub fn turn_on(&mut self) {
        self.output.set_high();
    }

    pub fn turn_off(&mut self) {
        self.output.set_low();
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

pub async fn highlight_round_winner(
    leds: &'_ mut [Led<'_>; 3],
    winner_button: ButtonRole,
    current_score: usize,
) {
    for _ in 0..3 {
        for led in leds.iter_mut().rev() {
            led.flash_pattern(Duration::from_millis(50), 1).await;
        }
    }
    Timer::after_millis(500).await;

    // Flash the winner
    let winner_led_role = match winner_button {
        ButtonRole::Player1 => LedRole::Player1,
        ButtonRole::Player2 => LedRole::Player2,
    };

    if let Some(winner_led) = leds.iter_mut().find(|led| led.role == winner_led_role) {
        debug!(
            "Blinking winner {} for current_score: {}",
            winner_led, current_score
        );
        for _ in 0..current_score {
            winner_led.turn_on();
            Timer::after_millis(500).await;

            winner_led.turn_off();
            Timer::after_millis(500).await;
        }
    }
}

// Turns on, then off for a random time with 'OFF' instant return for calculation of fastest player
pub async fn round_playing_leds_routine_on_off(
    leds: &'_ mut [Led<'_>; 3],
    current_round: usize,
) -> Instant {
    // Signal that round 'i' is about to start then quick blinky
    for _ in 0..current_round + 1 {
        for led in leds.iter_mut() {
            led.turn_on();
        }
        Timer::after_millis(750).await;
        for led in leds.iter_mut() {
            led.turn_off();
        }
        Timer::after_millis(750).await;
    }
    Timer::after_millis(500).await;
    for _ in 0..4 {
        for led in leds.iter_mut() {
            led.turn_on();
        }
        Timer::after_millis(150).await;
        for led in leds.iter_mut() {
            led.turn_off();
        }
        Timer::after_millis(150).await;
    }

    // Generate random time in ms between 2000-5000 ms for led signal to press button
    let mut rng = SimpleRngU64::new();
    let leds_duration = rng.generate_from_range(2000, 5000);
    info!(
        "Rng time for LED ON until shutoff for current game round: {} ms. ",
        leds_duration
    );
    for led in leds.iter_mut() {
        led.turn_on();
    }
    Timer::after_millis(leds_duration).await;

    for led in leds.iter_mut() {
        led.turn_off();
    }
    info!("GO!");
    Instant::now()
}
