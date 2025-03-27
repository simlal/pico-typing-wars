#![no_std]
#![no_main]

mod button;
mod game;
mod led;

use embassy_time::Timer;
use game::{
    get_current_game_state_or_reset, transition_game_state, update_current_game_state_duration,
    GameState,
};
use led::{waiting_state_leds, Led, LedRole};

use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Raspberry Pi Pico init in main executor...");
    let p = embassy_rp::init(Default::default());

    // Initializing LED peripherals
    let mut leds = [
        Led::new(p.PIN_25, LedRole::Onboard),
        Led::new(p.PIN_5, LedRole::Player1),
        Led::new(p.PIN_8, LedRole::Player2),
    ];
    for led in &leds {
        info!("Initializing {}...", led);
    }

    // Initialize game state singleton in waiting mode
    game::initialize_game().await;
    info!("OK for Game Singleton.");
    update_current_game_state_duration().await;

    loop {
        // Take the action based on game state
        let current_state = get_current_game_state_or_reset().await;

        match current_state {
            GameState::Waiting => {
                info!("We are waiting!");
                waiting_state_leds(&mut leds).await;
                transition_game_state(GameState::Playing).await;
            }
            GameState::Playing => {
                info!("We are playing!");
                let total_rounds = 3;

                Timer::after_secs(1).await;
                transition_game_state(GameState::Waiting).await;
            }
            _ => error!("err"),
        }
    }
}
