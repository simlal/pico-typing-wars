use defmt::{debug, error, info, warn, Format};

use embassy_rp::watchdog::Watchdog;
use embassy_sync::{
    blocking_mutex::raw::{CriticalSectionRawMutex, ThreadModeRawMutex},
    mutex::Mutex,
};
use embassy_time::{Duration, Instant, Timer};

type GameMutex = Mutex<CriticalSectionRawMutex, Option<Game>>;
static GAME: GameMutex = Mutex::new(None);

#[derive(PartialEq, Eq, Format, Clone, Copy)]
pub enum GameState {
    Waiting,
    Playing,
    ComputingResults,
    Finished,
}

// Singleton game instance
#[derive(Format)]
struct Game {
    state: GameState,
    state_start: Instant,
    state_duration: Duration,
}
impl Game {
    fn new() -> Game {
        Game {
            state: GameState::Waiting,
            state_start: Instant::now(), // Makes sense when first creating the game
            state_duration: Duration::from_secs(0),
        }
    }

    fn update_state_duration(&mut self) {
        let new_duration = Instant::now()
            .checked_duration_since(self.state_start)
            .unwrap_or(Duration::from_secs(0));
        self.state_duration = new_duration;

        // log it
        debug!(
            "Current GameState={}, started={} ms from boot with current-duration={} ms",
            self.state,
            self.state_start.as_millis(),
            self.state_duration.as_millis()
        );
    }

    fn transition(&mut self, next_state: GameState) {
        if next_state == self.state {
            info!("Already in the {} state, no transition needed.", self.state);
            self.update_state_duration();
            return;
        }

        self.update_state_duration();
        info!(
            "Current state duration before transition={}->{}: {} ms",
            self.state,
            next_state,
            self.state_duration.as_millis()
        );
        // Change the GAME's state object
        match next_state {
            GameState::Waiting => {
                self.state = GameState::Waiting;
                self.state_start = Instant::now();
            }
            GameState::Playing => {
                self.state = GameState::Playing;
                self.state_start = Instant::now();
            }
            GameState::ComputingResults => {
                self.state = GameState::ComputingResults;
                self.state_start = Instant::now();
            }
            GameState::Finished => {
                self.state = GameState::Finished;
                self.state_start = Instant::now();
            }
        }
        info!("Transition finished: {}", self)
    }
}

// *** Game singleton with mutex to share accross tasks *** //

// Helper function to initialize the global game instance
pub async fn initialize_game() {
    let mut game_lock = GAME.lock().await;
    *game_lock = Some(Game::new());

    // Making sure we panic at start of program
    match *game_lock {
        None => panic!("Could not initialize_game"),
        Some(_) => info!("GAME mutex init."),
    }
}

// Helper function to transition game state from any task
pub async fn transition_game_state(next_state: GameState) {
    let mut game_lock = GAME.lock().await;
    if let Some(game) = game_lock.as_mut() {
        game.transition(next_state);
    // releases game_lock
    } else {
        error!(
            "Attempted to transition to {} but GAME singleton not initialized properly!",
            next_state
        )
    }
}

// Helper to update and log current game state duration
pub async fn update_current_game_state_duration() {
    let mut game_lock = GAME.lock().await;
    if let Some(game) = game_lock.as_mut() {
        game.update_state_duration();
    } else {
        warn!("Attempted to update GAME duration but GAME singleton not initialized");
    }
}

// NOTE: FUNCTIONAL STYLE example for getting the mutex!
// pub async fn get_current_game_state() -> Option<GameState> {
//     let game_lock = GAME.lock().await;
//     game_lock.as_ref().map(|game| game.state).or_else(|| {
//         warn!("Attempted to get game state but GAME singleton not initialized");
//         None
//     })
// }
pub async fn get_current_game_state_or_reset(
    wd: &'static Mutex<ThreadModeRawMutex, Option<Watchdog>>,
) -> GameState {
    let game_lock = GAME.lock().await;
    match game_lock.as_ref() {
        Some(game) => game.state,
        None => {
            async {
                warn!(
                    "Attempted to get game state but GAME singleton not initialized. Resetting..."
                );
                // Lock the watchdog to prevent feeding
                let _lock_forever = wd.lock().await;
                loop {
                    Timer::after_secs(10).await; // Keep the lock forever
                }
            }
            .await;
            // HACK: Should not be reached, but fallback
            GameState::Waiting
        }
    }
}
