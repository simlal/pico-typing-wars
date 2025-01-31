#![no_std]
#![no_main]

mod game;
mod led;

use embassy_time::{Duration, Timer};
use game::{Game, GameState};
use led::Led;

use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

// Non concurrent flashing pattern
//#[embassy_executor::task]
async fn waiting_leds_task(leds: &mut [Led<'_>; 3]) {
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

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Initializing pico...");
    let p = embassy_rp::init(Default::default());
    // NOTE: Initialize directly in leds arr?
    let mut onboard_led = Led::new(p.PIN_25, "onboard");
    info!("{}", onboard_led);

    let mut player_1_led = Led::new(p.PIN_5, "player_1_led");
    let mut player_2_led = Led::new(p.PIN_8, "player_2_led");
    let mut leds = [onboard_led, player_1_led, player_2_led];

    // INFO: Test transition and defmt traits
    let mut game = Game::new(GameState::Waiting);
    info!("{}", game);
    game.transition();
    info!("{}", game);

    loop {
        // TODO: Convert to spawner-task
        waiting_leds_task(&mut leds).await;
    }
}
