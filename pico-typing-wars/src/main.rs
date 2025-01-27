#![no_std]
#![no_main]

mod led;

use defmt::*;
use embassy_executor::Spawner;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Initializing pico...");
    let p = embassy_rp::init(Default::default());
    let mut onboard_led = led::Led::new(p.PIN_25, "onboard");
    let mut player_1_led = led::Led::new(p.PIN_5, "player_1_led");
    let mut player_2_led = led::Led::new(p.PIN_8, "player_2_led");

    loop {
        //info!("Turning onboard led pin output to high...");
        onboard_led.blink(1000).await;

        // players leds test
        player_1_led.blink(250).await;
        player_2_led.blink(250).await;
    }
}
