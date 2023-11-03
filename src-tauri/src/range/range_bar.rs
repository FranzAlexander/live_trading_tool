use super::Bar;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RangeBar {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub start_time: i64,
    is_first_bar: bool,
}

impl Bar for RangeBar {
    fn new() -> Self {
        RangeBar {
            close: 0.0,
            high: 0.0,
            low: 0.0,
            open: 0.0,
            start_time: 0,
            is_first_bar: true,
        }
    }

    fn update(&mut self, price: f64, range: f64, time: f64) -> bool {
        if self.is_first_bar {
            self.open = price;
            self.high = price;
            self.low = price;
            self.close = price;
            self.start_time = time as i64;
            self.is_first_bar = false; // Set the flag to false after initializing the first bar
        } else {
            if price > self.high || price < self.low {
                self.high = self.high.max(price);
                self.low = self.low.min(price);

                if self.high - self.low >= range {
                    self.close = if price >= self.open {
                        self.high
                    } else {
                        self.low
                    };

                    return true;
                }
            }
        }
        false
    }

    fn reset(&mut self) {
        // When resetting, keep the closing price of the previous bar as the opening of the new bar
        let last_close = self.close;
        let last_end_time = self.start_time;

        self.open = last_close;
        self.high = last_close;
        self.low = last_close;
        self.close = last_close;
        self.start_time = last_end_time; // Start time of the new bar is the end time of the last bar
        self.is_first_bar = false; // After the first reset, all subsequent bars are not the first
    }
}
