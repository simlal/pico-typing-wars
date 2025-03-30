use embassy_rp::gpio::Level;

pub trait LevelToStr {
    fn level_to_str(&self, level: &Level) -> &str {
        match level {
            Level::Low => "Low",
            Level::High => "High",
        }
    }
}
