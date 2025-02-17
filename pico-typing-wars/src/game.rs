use defmt::{info, Format};

use embassy_time::{Duration, Instant};

#[derive(PartialEq, Eq, Format, Clone, Copy)]
pub enum GameState {
    Waiting,
    Playing,
    ComputingResult,
    Finished,
}

#[derive(Format)]
pub struct Game {
    state: GameState,
    state_start: Instant,
    state_duration: Duration,
}
impl Game {
    pub fn new(current_state: GameState) -> Game {
        Game {
            state: current_state,
            state_start: Instant::now(), // Makes sense when first creating the game
            state_duration: Duration::from_secs(0),
        }
    }

    fn update_state_duration(&mut self) {
        let new_duration = Instant::now().duration_since(self.state_start);
        self.state_duration = new_duration;
    }

    fn log_current_state_duration(&self) {
        info!(
            "Current GameState={}, started={} ms (from boot) and current-duration of {} ms",
            self.state,
            self.state_start.as_millis(),
            self.state_duration.as_millis()
        );
    }

    pub fn transition(&mut self, next_state: GameState) {
        if next_state == self.state {
            info!("Already in the {} state, no transition needed.", self.state);
            return;
        }

        info!(
            "Transitioning game from {} to {} state...",
            self.state, next_state
        );

        self.update_state_duration();
        self.log_current_state_duration();

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
                self.state = GameState::ComputingResult;
                self.state_start = Instant::now();
            }
            GameState::ComputingResult => {
                // TODO: Implement result logic
                self.state = GameState::Finished;
                self.state_start = Instant::now();
            }
            GameState::Finished => {
                // TODO: RESET method
                self.state = GameState::Waiting;
                self.state_start = Instant::now();
            }
        }
    }
}
