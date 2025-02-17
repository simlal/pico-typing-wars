#![no_std]
#![no_main]

mod game;
mod led;

use embassy_time::{Duration, Timer};
use game::{Game, GameState};
use led::{waiting_state_leds, Led};

use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Initializing pico...");
    let p = embassy_rp::init(Default::default());
    // NOTE: Initialize directly in leds arr?
    let onboard_led = Led::new(p.PIN_25, "onboard");
    info!("{}", onboard_led);

    let player_1_led = Led::new(p.PIN_5, "player_1_led");
    let player_2_led = Led::new(p.PIN_8, "player_2_led");
    let mut leds = [onboard_led, player_1_led, player_2_led];

    // INFO: Test transition and defmt traits
    let mut game = Game::new(GameState::Waiting);
    info!("{}", game);
    // test spawning the waiting task
    spawner.spawn(waiting_state_leds(&mut leds));

    game.transition(GameState::Playing);
    info!("{}", game);

    loop {
        //match game()
        //unwrap!(spawner.spawn(waiting_state_leds(&mut leds)))
    }
}
