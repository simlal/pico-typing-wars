#![no_std]
#![no_main]

mod game;
mod led;

use game::{Game, GameState};
use led::Led;

use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Initializing pico...");
    let p = embassy_rp::init(Default::default());
    let mut onboard_led = Led::new(p.PIN_25, "onboard");
    let mut player_1_led = Led::new(p.PIN_5, "player_1_led");
    let mut player_2_led = Led::new(p.PIN_8, "player_2_led");

    // Test transition and defmt traits
    let mut game = Game::new(GameState::Waiting);
    info!("{}", game);
    game.transition();
    info!("{}", game);

    // NOTE: test new flashing pattern
    // TODO: Refactor to task
    loop {
        //info!("Turning onboard led pin output to high...");
        onboard_led.flash_pattern(100, 2).await;

        // players leds test
        player_1_led.flash_pattern(100, 4).await;
        player_2_led.flash_pattern(100, 4).await;
    }
}
