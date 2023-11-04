use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaBar {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub cumulative_delta: f64,
    pub start_time: i64,
    pub is_new_bar: bool,
}

impl DeltaBar {
    pub fn new(start_time: i64, inital_delta: f64) -> Self {
        DeltaBar {
            open: inital_delta,
            high: inital_delta,
            low: inital_delta,
            close: inital_delta,
            cumulative_delta: inital_delta,
            start_time,
            is_new_bar: true,
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.cumulative_delta += delta;
        self.close = self.cumulative_delta;

        // Update high and low based on the cumulative delta.
        self.high = self.high.max(self.cumulative_delta);
        self.low = self.low.min(self.cumulative_delta);
    }
}
