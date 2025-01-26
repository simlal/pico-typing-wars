#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led_onboard = Output::new(p.PIN_25, Level::Low);
    let mut led_player_1 = Output::new(p.PIN_5, Level::Low);

    loop {
        info!("Turning onboard led pin output to high...");
        led_onboard.set_high();
        info!("led on!");
        Timer::after_secs(1).await;

        info!("Turning onboard led pin output to low...");
        led_onboard.set_low();
        info!("led off!");
        Timer::after_secs(1).await;

        // player1 test
        led_player_1.set_high();
        info!("player1 quick-on/off");
        Timer::after_millis(500).await;
        led_player_1.set_low();
        Timer::after_millis(500).await;
    }
}
