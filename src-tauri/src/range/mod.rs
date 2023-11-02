use std::collections::VecDeque;

use self::{delta_bar::DeltaBar, range_bar::RangeBar};

pub mod delta_bar;
pub mod range_bar;

pub trait Bar {
    fn new() -> Self;
    fn update(&mut self, value: f64, range: f64) -> bool;
    fn reset(&mut self);
}

pub struct RangeData {
    pub range: f64,
    pub range_bars: VecDeque<RangeBar>,
    pub delta_bars: VecDeque<DeltaBar>,
    pub count: usize,
}

impl RangeData {
    pub fn new(range: f64) -> Self {
        RangeData {
            range,
            range_bars: VecDeque::with_capacity(400),
            delta_bars: VecDeque::with_capacity(400),
            count: 0,
        }
    }

    pub fn update(&mut self, price: f64, size: f64, side: &str) {
        let delta = match side {
            "b" => size,
            "s" => -size,
            _ => 0.0,
        };

        if self.range_bars.is_empty()
            || self
                .range_bars
                .back_mut()
                .unwrap()
                .update(price, self.range)
        {
            if self.range_bars.len() == self.range_bars.capacity() {
                self.range_bars.pop_front();
            }
            self.range_bars.push_back(RangeBar::new());
        }

        if self.delta_bars.is_empty()
            || self
                .delta_bars
                .back_mut()
                .unwrap()
                .update(delta, self.range)
        {
            if self.delta_bars.len() == self.delta_bars.capacity() {
                self.delta_bars.pop_front();
            }
            self.delta_bars.push_back(DeltaBar::new());
        }
    }

    pub fn get_bars(&self) -> (VecDeque<RangeBar>, VecDeque<DeltaBar>) {
        (self.range_bars.clone(), self.delta_bars.clone())
    }
}
