#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RangeBar {
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
}

impl RangeBar {
    pub fn new() -> Self {
        RangeBar {
            close: 0.0,
            high: 0.0,
            low: 0.0,
            open: 0.0,
        }
    }
}
