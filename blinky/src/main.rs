//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

//use cortex_m_rt::entry;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    loop {
        info!("Turning onboard led pin output to high...");
        led.set_high();
        info!("led on!");
        Timer::after_secs(1).await;

        info!("Turning onboard led pin output to low...");
        led.set_low();
        info!("led off!");
        Timer::after_secs(1).await;
    }
}
