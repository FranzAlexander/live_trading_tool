use super::Bar;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RangeBar {
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
}

impl Bar for RangeBar {
    fn new() -> Self {
        RangeBar {
            close: 0.0,
            high: 0.0,
            low: 0.0,
            open: 0.0,
        }
    }

    fn update(&mut self, price: f64, range: f64) -> bool {
        if self.high == 0.0 && self.low == 0.0 {
            self.open = price;
            self.high = price;
            self.low = price;
            self.close = price;
        } else {
            if price > self.high {
                self.high = price;
                if self.high - self.low >= range {
                    self.close = self.high;
                    return true; // Bar is complete
                }
            } else if price < self.low {
                self.low = price;
                if self.high - self.low >= range {
                    self.close = self.low;
                    return true; // Bar is complete
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
    }
}
