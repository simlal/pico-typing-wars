#![no_std]
#![no_main]

mod game;
mod led;

use embassy_time::Timer;
use game::{initialize_game, transition_game_state, update_current_game_state_duration, GameState};
use led::{waiting_state_leds, Led};

use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Initializing pico...");
    let p = embassy_rp::init(Default::default());

    // Initializing leds
    let onboard_led = Led::new(p.PIN_25, "onboard");
    let player_1_led = Led::new(p.PIN_5, "player_1_led");
    let player_2_led = Led::new(p.PIN_8, "player_2_led");
    let leds = [onboard_led, player_1_led, player_2_led];

    for led in leds {
        info!("Initialized {}", led);
    }

    // Initialize game state singleton
    initialize_game().await;
    update_current_game_state_duration().await;
    // TODO: Spawn the LEDs waiting_state_leds task when in waiting game state

    // Testing game transition
    transition_game_state(GameState::Playing).await;
    Timer::after_secs(2).await;
    update_current_game_state_duration().await;

    //
}
