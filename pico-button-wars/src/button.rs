use crate::common::LevelToStr;

use defmt::{debug, info, Format};
use embassy_futures::select::{select, Either};
use embassy_rp::gpio::{Input, Level, Pin, Pull};
use embassy_rp::watchdog::*;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Instant, Ticker, Timer};

// Could be subject to interrupt but OK for now
pub type ButtonMutex = Mutex<ThreadModeRawMutex, Option<Button<'static>>>;

// Debounce time with prior tests from measure_minimal_debounce()
const MINIMAL_DEBOUNCE_TIME: u64 = 50;

#[derive(PartialEq, Eq, Format, Clone, Copy, Debug, Hash)]
pub enum ButtonRole {
    Player1,
    Player2,
}

pub struct Button<'a> {
    input: Input<'a>,
    role: ButtonRole,
    debounce: Duration,
}

impl Button<'_> {
    pub fn new<P: Pin>(pin: P, role: ButtonRole) -> Self {
        Self {
            input: Input::new(pin, Pull::Up), // Initialize input with pull up
            role,
            debounce: Duration::from_millis(MINIMAL_DEBOUNCE_TIME),
        }
    }

    pub fn role(&self) -> ButtonRole {
        self.role
    }

    async fn wait_for_press(&mut self) -> Instant {
        loop {
            self.input.wait_for_falling_edge().await;
            let press_instant = Instant::now();
            Timer::after(self.debounce).await;
            // safety in case debounce not enough
            if self.input.get_level() == Level::Low {
                info!("{} button pressed.", self.role);
                return press_instant;
            }
        }
    }

    async fn wait_for_release(&mut self) -> Instant {
        loop {
            self.input.wait_for_low().await;
            let release_instant = Instant::now();
            Timer::after(self.debounce).await;
            // safety in case debounce not enough
            if self.input.get_level() == Level::High {
                info!("{} button released.", self.role);
                return release_instant;
            }
        }
    }

    pub async fn measure_full_press_release(&mut self) -> Instant {
        self.wait_for_press().await;
        return self.wait_for_release().await;
    }

    pub async fn wait_for_full_press(&mut self) {
        self.wait_for_press().await;
    }

    // Figure out minimal debounce time for button press
    pub async fn measure_minimal_debounce(&mut self, ms_test_range: u64, iterations: usize) -> u64 {
        const MIN_DEBOUNCE_DEFAULT_IN_TEST: u64 = 150;
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
        info!(
            "Returning 10% over maximum debounce time or default {}",
            MIN_DEBOUNCE_DEFAULT_IN_TEST
        );
        (max_debounce_time + (max_debounce_time / 10)).max(MIN_DEBOUNCE_DEFAULT_IN_TEST)
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

#[embassy_executor::task(pool_size = 1)]
pub async fn monitor_double_longpress(
    b1: &'static ButtonMutex,
    b2: &'static ButtonMutex,
    wd: &'static Mutex<ThreadModeRawMutex, Option<Watchdog>>,
) {
    // Less-blocking approach with 50 ms polling on button mutex
    let mut ticker = Ticker::every(Duration::from_millis(50));

    // Track long press state
    let mut b1_pressed_time: Option<Instant> = None;
    let mut b2_pressed_time: Option<Instant> = None;
    let mut reset_countdown_active = false;

    loop {
        // Check both buttons without holding locks for too long
        let b1_pressed = {
            // Only try to lock for a short 10ms time before giving up
            match select(b1.lock(), Timer::after(Duration::from_millis(10))).await {
                Either::First(button_lock) => {
                    if let Some(button) = button_lock.as_ref() {
                        button.input.get_level() == Level::Low
                    } else {
                        false // Could not acquire lock
                    }
                }
                Either::Second(_) => {
                    // Couldn't get lock, maintain previous state
                    b1_pressed_time.is_some()
                }
            }
        };

        let b2_pressed = {
            // Only try to lock for a short time before giving up
            match select(b2.lock(), Timer::after(Duration::from_millis(10))).await {
                Either::First(button_lock) => {
                    if let Some(button) = button_lock.as_ref() {
                        button.input.get_level() == Level::Low
                    } else {
                        false // Could not acquire lock
                    }
                }
                Either::Second(_) => {
                    // Couldn't get lock, maintain previous state
                    b2_pressed_time.is_some()
                }
            }
        };

        // Update press times
        if b1_pressed && b1_pressed_time.is_none() {
            b1_pressed_time = Some(Instant::now());
            debug!("Button 1 pressed");
        }

        if b2_pressed && b2_pressed_time.is_none() {
            b2_pressed_time = Some(Instant::now());
            debug!("Button 2 pressed");
        }

        // Check for button releases
        if !b1_pressed && b1_pressed_time.is_some() {
            let duration = b1_pressed_time.unwrap().elapsed();
            debug!("Button 1 released after {} ms", duration.as_millis());
            b1_pressed_time = None;
            reset_countdown_active = false;
        }

        if !b2_pressed && b2_pressed_time.is_some() {
            let duration = b2_pressed_time.unwrap().elapsed();
            debug!("Button 2 released after {} ms", duration.as_millis());
            b2_pressed_time = None;
            reset_countdown_active = false;
        }

        // Check for longpress condition
        if let (Some(t1), Some(t2)) = (b1_pressed_time, b2_pressed_time) {
            let b1_duration = t1.elapsed();
            let b2_duration = t2.elapsed();

            if !reset_countdown_active
                && b1_duration.as_millis() >= 1000
                && b2_duration.as_millis() >= 1000
            {
                reset_countdown_active = true;
                info!(
                    "Both buttons held for 1+ second. Continuing to monitor for reset threshold..."
                );
            }

            // Check if press duration is enough to trigger reset
            if b1_duration.as_millis() >= 3000 && b2_duration.as_millis() >= 3000 {
                info!(
                    "Long press detected on both buttons (b1={} ms, b2={} ms). Resetting via watchdog...",
                    b1_duration.as_millis(),
                    b2_duration.as_millis()
                );

                // Lock the watchdog to prevent feeding
                let _lock_forever = wd.lock().await;
                loop {
                    Timer::after_secs(10).await; // Keep the lock forever
                }
            }
        }

        ticker.next().await;
    }
}
