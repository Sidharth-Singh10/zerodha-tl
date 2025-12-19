#[derive(Debug, Default)]
pub struct Tick {
    pub instrument_token: u32,
    pub mode: Mode,
    pub ltp: f64,
    pub volume: i32,
    // Add other fields as needed: open, high, low, close...
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    LTP,
    Quote,
    Full,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::LTP
    }
}
