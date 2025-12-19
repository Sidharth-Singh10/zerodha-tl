use crate::models::Mode;

#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub instruments: Vec<u32>,
    pub mode: Mode,
}

impl StreamConfig {
    pub fn new(instruments: Vec<u32>) -> Self {
        Self {
            instruments,
            mode: Mode::LTP,
        }
    }

    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }
}
