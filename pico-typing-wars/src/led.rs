use defmt::{debug, Format};
use embassy_rp::gpio::{Level, Output, Pin};
use embassy_time::{Duration, Timer};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

// Globally accessible LEDs
pub type LedsMutex = Mutex<CriticalSectionRawMutex, Option<[Led; 3]>>;
pub static LEDS: LedsMutex = Mutex::new(None);

// A simple abstraction over an output pin with a role
pub struct Led {
    output: Output<'static>,
    role: &'static str,
}

impl Led {
    pub fn new<P: Pin>(pin: P, role: &'static str) -> Self {
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
impl Format for Led {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Led {{ role: {}, output_level={} }}",
            self.role(),
            level_to_str(&self.output.get_output_level()),
        )
    }
}

// ****** GameState Leds Tasks ****** //

// Non concurrent flashing pattern
#[embassy_executor::task(pool_size = 1)]
pub async fn waiting_state_leds(leds: &'static LedsMutex) {
    loop {
        // Lock the mutex to get exclusive access to LEDs
        let mut leds_lock = leds.lock().await;

        // Early continue if LED array is not initialized
        let Some(led_array) = leds_lock.as_mut() else {
            drop(leds_lock); // Explicitly drop the lock
            Timer::after_millis(100).await;
            continue;
        };

        debug!("Flashing leds inside waiting_state_leds(): {}", led_array);

        // First pattern, fast flashes
        let mut duration = Duration::from_millis(200);
        for led in led_array.iter_mut() {
            led.flash_pattern(duration, 2).await;
        }

        // Chasing second pattern
        let mut i: usize = 0;
        let mut passes: usize = 0;
        let max_circles: usize = 10;
        duration = Duration::from_millis(100);
        loop {
            led_array[0].flash_pattern(duration, 1).await;
            led_array[1].flash_pattern(duration, 1).await;
            led_array[2].flash_pattern(duration, 1).await;

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

        // Release the leds before new flashing routine
        drop(leds_lock);
        Timer::after_millis(500).await;
    }
}

// ****** Initialize the global LED array ****** //
pub async fn initialize_leds(leds: [Led; 3]) {
    let mut led_lock = LEDS.lock().await;
    *led_lock = Some(leds);
}
