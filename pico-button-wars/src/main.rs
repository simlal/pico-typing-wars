#![no_std]
#![no_main]

mod button;
mod common;
mod game;
mod led;

use button::{Button, ButtonRole};
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

    // Initializing LED peripherals with output level as Low
    let mut leds = [
        Led::new(p.PIN_25, LedRole::Onboard),
        Led::new(p.PIN_5, LedRole::Player1),
        Led::new(p.PIN_8, LedRole::Player2),
    ];
    for led in &leds {
        info!("Initializing {}...", led);
    }

    // Initializing Buttons peripherals with Pull UP
    let mut button_p1 = Button::new(p.PIN_10, ButtonRole::Player1);
    let mut button_p2 = Button::new(p.PIN_11, ButtonRole::Player2);
    info!("Initializing {}...", &button_p1);
    info!("Initializing {}...", &button_p2);

    // Initialize game state singleton in waiting mode
    game::initialize_game().await;
    info!("OK for Game Singleton.");
    update_current_game_state_duration().await;

    // TODO: Spawn the DISPLAY task with 1 Hz refresh rate (NO Guarantee)

    loop {
        // Take the action based on game state
        let current_state = get_current_game_state_or_reset().await;

        // NOTE: Main priority compared to button reset + display refresh
        match current_state {
            GameState::Waiting => {
                info!("We are waiting!");
                // waiting_state_leds(&mut leds).await;
                transition_game_state(GameState::Playing).await;
            }
            GameState::Playing => {
                info!("We are playing!");
                // let total_rounds = 3;

                Timer::after_secs(1).await;

                // Test buttons!
                let minimal_debounce_b1 = button_p1.measure_minimal_debounce(100, 10).await;
                info!("min debounce ms: {}", minimal_debounce_b1);

                // button_p1.wait_for_press().await;
                // button_p2.wait_for_press().await;

                // transition_game_state(GameState::Waiting).await;
            }
            _ => error!("err"),
        }
    }
}
