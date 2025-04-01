#![no_std]
#![no_main]

mod button;
mod common;
mod game;
mod led;

use button::{monitor_double_longpress, Button, ButtonMutex, ButtonRole};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::watchdog::*;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::{with_deadline, Duration, Instant, Ticker, Timer};
use game::{
    get_current_game_state_or_reset, transition_game_state, update_current_game_state_duration,
    GameState,
};
use led::{waiting_state_leds, Led, LedRole};
use {defmt_rtt as _, panic_probe as _};

// Static watchdog & buttons periphs to allow for tasks
static WATCHDOG: Mutex<ThreadModeRawMutex, Option<Watchdog>> = Mutex::new(None);
static BUTTON_P1: ButtonMutex = Mutex::new(None);
static BUTTON_P2: ButtonMutex = Mutex::new(None);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Raspberry Pi Pico init in main executor...");
    let p = embassy_rp::init(Default::default());
    // Watchdog for reset
    {
        let mut watchdog_unlocked = WATCHDOG.lock().await;
        *watchdog_unlocked = Some(Watchdog::new(p.WATCHDOG));

        // Making sure we panic if unproperly init
        match *watchdog_unlocked {
            None => crate::panic!("Could not initialize watchdog timer"),
            Some(_) => info!("Initialized 'WATCHDOG'  as static shareable thread-safe ref",),
        }

        if let Some(wd) = watchdog_unlocked.as_mut() {
            let wd_starve_time = Duration::from_secs(3);
            wd.start(wd_starve_time);
            info!(
                "Started watchdog on feed scheduale of {} s",
                wd_starve_time.as_secs()
            );
        }
    }

    // Start watchdog feeding task so we can reset the game with longpress
    spawner
        .spawn(feed_watchdog(&WATCHDOG, Duration::from_millis(500)))
        .unwrap();

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
    {
        let mut button_p1_unlocked = BUTTON_P1.lock().await;
        *button_p1_unlocked = Some(Button::new(p.PIN_10, ButtonRole::Player1));

        let mut button_p2_unlocked = BUTTON_P2.lock().await;
        *button_p2_unlocked = Some(Button::new(p.PIN_11, ButtonRole::Player1));

        // Making sure we panic if unproperly init
        match *button_p1_unlocked {
            None => crate::panic!("Could not initialize player 1 button."),
            Some(_) => info!("Initialized 'BUTTON_P1'  as static shareable thread-safe ref",),
        }
        match *button_p2_unlocked {
            None => crate::panic!("Could not initialize player 2 button."),
            Some(_) => info!("Initialized 'BUTTON_P2'  as static shareable thread-safe ref",),
        }
    }

    // Initialize game state singleton in waiting mode
    game::initialize_game().await;
    info!("OK for Game Singleton.");
    update_current_game_state_duration().await;

    // Spawn the button press task to reset game on long press
    spawner
        .spawn(monitor_double_longpress(&BUTTON_P1, &BUTTON_P2, &WATCHDOG))
        .unwrap();

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
                loop {
                    // if let Some(button_p1) = (BUTTON_P1.lock().await).as_mut() {
                    //     let press_duration = button_p1.measure_full_press_release().await;
                    //     info!("{}", press_duration.as_millis());
                    //
                    info!("1s in playing loop");
                    Timer::after_secs(1).await;
                }
                // info!("{}", press_duration.as_millis());
                // let start = Instant::now();
                //
                // match with_deadline(start + Duration::from_secs(3), button_p1.wait_for_press())
                //     .await
                // {
                //     // Button Released < 1s
                //     Ok(_) => {
                //         info!("Button pressed for: {}ms", start.elapsed().as_millis());
                //         continue;
                //     }
                //     // button held for > 1s
                //     Err(_) => {
                //         info!("Button Held");
                //     }
                // }
            }

            // transition_game_state(GameState::Waiting).await;
            GameState::Finished => {
                info!("Done testing. Going into wait mode.");
                game::transition_game_state(GameState::Waiting).await;
            }
            _ => error!("err"),
        }
    }
}

// TEST DEBOUNCE TIME. Uncomment to run as main
// #[embassy_executor::main]
async fn _test_debounce_time(_spawner: Spawner) {
    info!("Raspberry Pi Pico init in main executor...");
    let p = embassy_rp::init(Default::default());

    // Initializing Buttons peripherals with Pull UP
    let mut button_p1 = Button::new(p.PIN_10, ButtonRole::Player1);
    info!("Initializing {}...", &button_p1);

    // Initialize game state singleton in waiting mode
    game::initialize_game().await;
    loop {
        // Take the action based on game state
        let current_state = get_current_game_state_or_reset().await;

        // NOTE: Main priority compared to button reset + display refresh
        match current_state {
            GameState::Playing => {
                let minimal_debounce_b1 = button_p1.measure_minimal_debounce(100, 10).await;
                info!(
                    "min debounce ms: {} for {}",
                    minimal_debounce_b1, &button_p1
                );

                game::transition_game_state(GameState::Finished).await;
            }
            GameState::Finished => {
                info!("Done testing. Going into wait mode.");
                game::transition_game_state(GameState::Waiting).await;
            }

            _ => error!("err"),
        }
    }
}

#[embassy_executor::task(pool_size = 1)]
pub async fn feed_watchdog(
    wd: &'static Mutex<ThreadModeRawMutex, Option<Watchdog>>,
    feed_schedule: Duration,
) {
    let mut ticker = Ticker::every(feed_schedule);

    loop {
        {
            let mut wd_unlocked = wd.lock().await;
            if let Some(wd) = wd_unlocked.as_mut() {
                wd.feed();
                // info!("watchdog fed")
            }
        } // watchdog lock dropped here
        ticker.next().await;
    }
}
