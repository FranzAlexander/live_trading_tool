use serde::{Deserialize, Serialize};

use super::Bar;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaBar {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub cumulative_delta: f64,
}

impl Bar for DeltaBar {
    fn new() -> Self {
        DeltaBar {
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            cumulative_delta: 0.0,
        }
    }
    fn update(&mut self, delta: f64, range: f64) -> bool {
        if self.open == 0.0 {
            // Initialize the bar if it's the first update.
            self.open = delta;
            self.high = delta;
            self.low = delta;
            self.close = delta;
            self.cumulative_delta = delta;
        } else {
            // Update the cumulative delta and close value.
            self.cumulative_delta += delta;
            self.close = self.cumulative_delta;
    
            // Update the high and low values.
            if self.cumulative_delta > self.high {
                self.high = self.cumulative_delta;
            }
            if self.cumulative_delta < self.low {
                self.low = self.cumulative_delta;
            }
    
            // Check if the bar needs to be closed.
            if (self.high - self.low).abs() >= range {
                if self.close == self.high || self.close == self.low {
                    // Bar closes at high or low, respecting Rule 3.
                    return true;
                }
            }
        }
        false
    }
    

    fn reset(&mut self) {
        self.open = 0.0;
        self.high = 0.0;
        self.low = 0.0;
        self.close = 0.0;
        self.cumulative_delta = 0.0;
    }
}
