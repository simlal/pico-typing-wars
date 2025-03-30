use core::iter;

use crate::common::LevelToStr;

use defmt::{debug, info, Format};
use embassy_rp::gpio::{Input, Level, Pin, Pull};
use embassy_time::{Duration, Instant, Timer};

#[derive(PartialEq, Eq, Format, Clone, Copy)]
pub enum ButtonRole {
    Player1,
    Player2,
}
pub struct Button<'a> {
    input: Input<'a>,
    role: ButtonRole,
}

impl Button<'_> {
    pub fn new<P: Pin>(pin: P, role: ButtonRole) -> Self {
        Self {
            input: Input::new(pin, Pull::Up), // Initialize input with pull up
            role,
        }
    }

    pub async fn wait_for_press(&mut self) {
        self.input.wait_for_low().await;
        info!("{} is LOW!", self);
        Timer::after_millis(100).await;
        self.input.wait_for_high().await;
    }

    pub async fn measure_minimal_debounce(&mut self, ms_test_range: u64, iterations: usize) -> u64 {
        info!(
            "Measuring debounce for {} Button with {} ms max and averaging over {}",
            self.role, ms_test_range, iterations
        );
        let mut total_transitions = 0;
        let mut max_debounce_time = 0;
        for i in 0..iterations {
            // Wait for an initial press
            self.input.wait_for_low().await;
            info!("Button pressed! Measuring minimal debounce time");

            // Debounce
            let mut transitions = 0;
            let mut last_level = Level::Low; // We just checked its low

            let start_time = Instant::now();
            let mut last_transition_time = start_time;
            let mut longest_debounce = Duration::from_millis(0);
            // Fix: Add duration to start_time instead of subtracting
            let end_time = start_time + Duration::from_millis(ms_test_range);

            // Evaluate max transition time
            while Instant::now() < end_time {
                let current_level = self.input.get_level();
                if current_level != last_level {
                    transitions += 1;
                    let now = Instant::now();

                    // No need to debounce if no transitions
                    if transitions > 1 {
                        let bounce_duration = now - last_transition_time;
                        if bounce_duration > longest_debounce {
                            longest_debounce = bounce_duration;
                        }
                    }

                    last_transition_time = now;
                    debug!(
                        "Transition #{} detected from {} to {} at {} ms from test start.",
                        transitions,
                        self.level_to_str(&last_level),
                        self.level_to_str(&current_level),
                        (last_transition_time - start_time).as_millis()
                    );
                    last_level = current_level;
                }

                // Small delay to prevent tight CPU looping
                Timer::after_micros(50).await;
            }

            info!(
                "Detected {} transitions in iteration {}",
                transitions,
                i + 1
            );
            if transitions > 0 {
                info!(
                    "Longest debounce interval: {}ms",
                    longest_debounce.as_millis()
                );
                max_debounce_time = max_debounce_time.max(longest_debounce.as_millis());
            }

            total_transitions += transitions;

            info!(
                "Found {} transitions with longest_debounce time of {} ms for test iteration i={}",
                transitions,
                longest_debounce.as_millis(),
                i
            );

            // Wait for button release before next iteration
            if i < iterations - 1 {
                self.input.wait_for_high().await;
                // Add delay between tests
                Timer::after_millis(500).await;
            }
        }
        // Compute summary
        let avg_transitions = if iterations > 0 {
            total_transitions / iterations as u64
        } else {
            0
        };
        info!(
            "Summary: Avg transitions={}, longest_debounce_time={} ms over {} iterations.",
            avg_transitions, max_debounce_time, iterations
        );
        info!("Returning 20% over maximum debounce time");
        (max_debounce_time + (max_debounce_time / 5)).max(5)
    }
}

// Formatting traits
impl LevelToStr for Button<'_> {}
impl Format for Button<'_> {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Button {{ role: {}, level={} }}",
            self.role,
            self.level_to_str(&self.input.get_level()),
        )
    }
}

//TODO: Button controller
// TODO: Task for reset
