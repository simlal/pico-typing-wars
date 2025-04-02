#![no_std]
#![no_main]

mod button;
mod common;
mod game;
mod led;

use button::{monitor_double_longpress, Button, ButtonMutex, ButtonRole};
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_rp::watchdog::*;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::{Duration, Instant, Ticker, Timer};
use heapless::{Entry, FnvIndexMap};

use {defmt_rtt as _, panic_probe as _};

use game::{
    get_current_game_state_or_reset, transition_game_state, update_current_game_state_duration,
    GameState,
};
use led::{
    highlight_round_winner, round_playing_leds_routine_on_off, waiting_state_leds, Led, LedRole,
};

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
        *button_p2_unlocked = Some(Button::new(p.PIN_11, ButtonRole::Player2));

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
    const TOTAL_ROUNDS: usize = 5;
    let mut round_winner_times: [(Option<ButtonRole>, u64); TOTAL_ROUNDS] =
        [(None::<ButtonRole>, u64::MIN); TOTAL_ROUNDS];

    let mut players_scores = FnvIndexMap::<ButtonRole, usize, 2>::new();
    players_scores.insert(ButtonRole::Player1, 0).unwrap();
    players_scores.insert(ButtonRole::Player2, 0).unwrap();

    loop {
        // Take the action based on game state
        let current_state = get_current_game_state_or_reset(&WATCHDOG).await;

        // NOTE: Main priority compared to button reset + display refresh
        match current_state {
            GameState::Waiting => {
                info!("We are waiting! Resetting scores before next game");
                // Resetting scores in case we are coming in from a previous game
                for (role, time) in round_winner_times.iter_mut() {
                    *role = None;
                    *time = 0;
                }

                if let Entry::Occupied(mut o) = players_scores.entry(ButtonRole::Player1) {
                    *o.get_mut() = 0;
                }
                if let Entry::Occupied(mut o) = players_scores.entry(ButtonRole::Player2) {
                    *o.get_mut() = 0;
                }

                transition_game_state(GameState::Playing).await;
            }
            GameState::Playing => {
                info!("We are playing!");
                'rounds: for (i, round) in round_winner_times.iter_mut().enumerate() {
                    info!("Players get ready for round #{}", i);

                    // Insure we have both button mutex
                    let mut b1_unlocked = BUTTON_P1.lock().await;
                    let mut b2_unlocked = BUTTON_P2.lock().await;
                    if let (Some(b1_ref), Some(b2_ref)) =
                        (b1_unlocked.as_mut(), b2_unlocked.as_mut())
                    {
                        // Randomized time w/ light ON then OFF + pick first to full press w/ time
                        let target_time_press =
                            round_playing_leds_routine_on_off(&mut leds, i).await;
                        let winner_timepress = select(
                            b1_ref.measure_full_press_release(),
                            b2_ref.measure_full_press_release(),
                        )
                        .await;

                        // Use the button to match the winner led and add it to scores container
                        let winner = match winner_timepress {
                            Either::First(p1_release) => {
                                info!("B1 was faster!");
                                let p1_score = (p1_release - target_time_press).as_millis();
                                (b1_ref.role(), p1_score)
                            }
                            Either::Second(p2_release) => {
                                info!("B2 was faster!");
                                let p2_score = (p2_release - target_time_press).as_millis();
                                (b2_ref.role(), p2_score)
                            }
                        };
                        // Update the player scores
                        if let Entry::Occupied(mut o) = players_scores.entry(winner.0) {
                            *o.get_mut() += 1;
                        }

                        // Save score and highlight round winner
                        highlight_round_winner(
                            &mut leds,
                            winner.0,
                            *players_scores.get(&winner.0).unwrap(),
                        )
                        .await;
                        *round = (Some(winner.0), winner.1);
                        info!(
                            "DINGINGINGING! Congratulations for {} with a response time of {} ms",
                            winner.0, winner.1
                        );
                        // If we have a winner (best of 5), transition to Computing Results
                        info!("Current scores: ");
                        for (player, score) in &players_scores {
                            info!("{}: {}", player, score);
                            if *score == 3 {
                                transition_game_state(GameState::ComputingResults).await;
                                break 'rounds;
                            }
                        }
                    }
                    info!(
                        "Target window for ressetting game with long button double press of 2s..."
                    );
                    drop(b1_unlocked);
                    drop(b2_unlocked);
                    Timer::after_secs(2).await; // Just before starting next round
                }
            }

            GameState::ComputingResults => {
                info!("Computing results for current game...");
                game::transition_game_state(GameState::Finished).await;
            }
            GameState::Finished => {
                info!("Finished the game. Going back into waiting mode.");
                game::transition_game_state(GameState::Waiting).await;
            }
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
        let current_state = get_current_game_state_or_reset(&WATCHDOG).await;

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
