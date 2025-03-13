#![no_std]
#![no_main]

mod game;
mod led;

use embassy_time::Timer;
use game::{transition_game_state, update_current_game_state_duration, GameState};
use led::{waiting_state_leds, Led};

use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Raspberry Pi Pico init in main executor...");
    let p = embassy_rp::init(Default::default());

    // Initializing LED peripherals
    let leds = [
        Led::new(p.PIN_25, "onboard"),
        Led::new(p.PIN_5, "player_1_led"),
        Led::new(p.PIN_8, "player_2_led"),
    ];
    for led in &leds {
        info!("Initializing {}...", led);
    }

    // Initialize LEDS mutex
    led::initialize_leds(leds).await;
    info!("OK for LEDS.");

    // Initialize game state singleton
    game::initialize_game().await;
    info!("OK for Game Singleton.");

    update_current_game_state_duration().await;
    unwrap!(spawner.spawn(waiting_state_leds(&led::LEDS)));

    // Testing game transition
    transition_game_state(GameState::Playing).await;
    Timer::after_secs(2).await;
    update_current_game_state_duration().await;
}
