use std::collections::VecDeque;

use self::{delta_bar::DeltaBar, range_bar::RangeBar};

pub mod delta_bar;
pub mod range_bar;

pub trait Bar {
    fn new() -> Self;
    fn update(&mut self, value: f64, range: f64, time: f64) -> bool;
    fn reset(&mut self);
}

#[derive(serde::Serialize, Clone)]
pub struct RangePayload {
    pub range_bar: RangeBar,
    pub delta_bar: DeltaBar,
}

pub struct RangeData {
    pub range: f64,
    pub current_bar: Option<RangeBar>,
    pub current_delta_bar: Option<DeltaBar>,
    pub range_bars: VecDeque<RangeBar>,
    pub delta_bars: VecDeque<DeltaBar>,
    pub count: usize,
}

impl RangeData {
    pub fn new(range: f64) -> Self {
        RangeData {
            range,
            current_bar: None,
            current_delta_bar: None,
            range_bars: VecDeque::with_capacity(400),
            delta_bars: VecDeque::with_capacity(400),
            count: 0,
        }
    }

    pub fn update(
        &mut self,
        price: f64,
        size: f64,
        side: &str,
        time: f64,
    ) -> (Option<RangeBar>, Option<DeltaBar>) {
        let delta = if side == "b" { size } else { -size };

        if let (Some(bar), Some(delta_bar)) = (&mut self.current_bar, &mut self.current_delta_bar) {
            bar.update(price, size);
            delta_bar.update(delta);
            if bar.is_complete(self.range) {
                let new_start_time = if (time as i64) <= bar.start_time {
                    bar.start_time + 1
                } else {
                    time as i64
                };
                self.range_bars.push_back(bar.clone());
                self.delta_bars.push_back(delta_bar.clone());
                self.current_bar = Some(RangeBar::new(price, size, new_start_time));
                self.current_delta_bar = Some(DeltaBar::new(
                    new_start_time,
                    delta_bar.cumulative_delta + delta,
                ));
                return (
                    Some(self.range_bars.back().unwrap().clone()),
                    Some(self.delta_bars.back().unwrap().clone()),
                );
            }
        } else {
            self.current_bar = Some(RangeBar::new(price, size, time as i64));
            self.current_delta_bar = Some(DeltaBar::new(time as i64, delta));
        };

        (None, None)
    }

    pub fn get_bars(&self) -> (VecDeque<RangeBar>, VecDeque<DeltaBar>) {
        (self.range_bars.clone(), self.delta_bars.clone())
    }
}
