#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RangeBar {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub start_time: i64,
    pub is_new_bar: bool,
}

impl RangeBar {
    pub fn new(price: f64, size: f64, start_time: i64) -> Self {
        RangeBar {
            open: price,
            high: price,
            low: price,
            close: price,
            volume: size,
            start_time,
            is_new_bar: true,
        }
    }

    pub fn update(&mut self, price: f64, size: f64) {
        self.high = self.high.max(price);
        self.low = self.low.min(price);
        self.close = price;
        self.volume += size;
    }

    pub fn is_complete(&self, range: f64) -> bool {
        (self.high - self.low) > range
    }
}
