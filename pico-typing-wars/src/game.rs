use defmt::Format;

use embassy_time::Instant;

#[derive(PartialEq, Eq, Format)]
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
}

impl Game {
    pub fn new(current_state: GameState) -> Game {
        Game {
            state: current_state,
            state_start: Instant::now(), // Makes sense when first creating the game
        }
    }

    // NOTE: only getters for debugging for now not sure to leave here
    pub fn get_state(&self) -> &GameState {
        &self.state
    }
    pub fn get_state_start(&self) -> &Instant {
        &self.state_start
    }

    pub fn transition(&mut self) {
        // Repeat now() inside all match arms to respect real-time setting
        match &self.state {
            GameState::Waiting => {
                self.state = GameState::Playing;
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
