use defmt::{error, info, warn, Format};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Instant, Timer};

#[derive(PartialEq, Eq, Format, Clone, Copy)]
pub enum GameState {
    Waiting,
    Playing,
    ComputingResult,
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
    // FIX: Make it default to waiting and initilaize leds ?
    fn new(current_state: GameState) -> Game {
        Game {
            state: current_state,
            state_start: Instant::now(), // Makes sense when first creating the game
            state_duration: Duration::from_secs(0),
        }
    }

    fn update_state_duration(&mut self) {
        let new_duration = Instant::now().duration_since(self.state_start);
        self.state_duration = new_duration;

        // log it
        info!(
            "Current GameState={}, started={} ms from boot with current-duration={} ms",
            self.state,
            self.state_start.as_millis(),
            self.state_duration.as_millis()
        );
    }

    fn transition(&mut self, next_state: GameState) {
        if next_state == self.state {
            info!("Already in the {} state, no transition needed.", self.state);
            return;
        }

        info!(
            "Transitioning game from {} to {} state...",
            self.state, next_state
        );

        self.update_state_duration();

        self.state = next_state;
        self.state_start = Instant::now();
        self.update_state_duration();
        match next_state {
            GameState::Waiting => {
                self.state = GameState::Waiting;
                self.state_start = Instant::now();
            }
            GameState::Playing => {
                // TODO: Implement round logic
                self.state = GameState::Playing;
                self.state_start = Instant::now();
            }
            GameState::ComputingResult => {
                // TODO: Implement result logic
                self.state = GameState::ComputingResult;
                self.state_start = Instant::now();
            }
            GameState::Finished => {
                // TODO: PLAY A LED ROUTINE + return to waiting
                self.state = GameState::Finished;
                self.state_start = Instant::now();

                // Timer::after_secs(1).await;
                info!("Finished game!");
                self.transition(GameState::Waiting);
            }
        }
    }
}

// Game singleton with mutex to share accross tasks
pub static GAME: Mutex<CriticalSectionRawMutex, Option<Game>> = Mutex::new(None);

// Helper function to initialize the global game instance
pub async fn initialize_game() {
    let mut game_lock = GAME.lock().await;
    *game_lock = Some(Game::new(GameState::Waiting));
}

// Helper function to transition game state from any task
pub async fn transition_game_state(next_state: GameState) {
    let mut game_lock = GAME.lock().await;
    if let Some(game) = game_lock.as_mut() {
        game.transition(next_state);
    // releases game_lock
    } else {
        error!("GAME singleton not initialized properly!")
    }
}

// Helper to update and log current game state duration
pub async fn update_current_game_state_duration() {
    let mut game_lock = GAME.lock().await;
    if let Some(game) = game_lock.as_mut() {
        game.update_state_duration();
    // releases game_lock
    } else {
        warn!("Cannot get hold of GAME singleton")
    }
}
