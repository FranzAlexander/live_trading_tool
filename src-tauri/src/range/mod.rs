use std::collections::VecDeque;

use self::{delta_bar::DeltaBar, range_bar::RangeBar};

pub mod delta_bar;
pub mod range_bar;

pub trait Bar {
    fn new() -> Self;
    fn update(&mut self, value: f64, range: f64, time: f64) -> bool;
    fn reset(&mut self);
}

pub struct RangeData {
    pub range: f64,
    pub current_bar: Option<RangeBar>,
    pub range_bars: VecDeque<RangeBar>,
    pub delta_bars: VecDeque<DeltaBar>,
    pub count: usize,
}

impl RangeData {
    pub fn new(range: f64) -> Self {
        RangeData {
            range,
            current_bar: None,
            range_bars: VecDeque::with_capacity(400),
            delta_bars: VecDeque::with_capacity(400),
            count: 0,
        }
    }

    pub fn update(&mut self, price: f64, size: f64, side: &str, time: f64) {
        let delta = match side {
            "b" => size,
            "s" => -size,
            _ => 0.0,
        };

        if let Some(bar) = &mut self.current_bar {
            bar.update(price);
            if bar.is_complete(self.range) {
                let new_start_time = if (time as i64) <= bar.start_time {
                    bar.start_time + 1
                } else {
                    time as i64
                };
                self.range_bars.push_back(bar.clone());
                self.current_bar = Some(RangeBar::new(price, new_start_time));
            }
        } else {
            self.current_bar = Some(RangeBar::new(price, time as i64))
        };

        // Update or create a new RangeBar if necessary
        // match self.range_bars.back_mut() {
        //     Some(last_bar) => {
        //         if !last_bar.update(price, self.range, time) {
        //             // Check if the price has moved sufficiently away from the last bar's close
        //             if (price - last_bar.close).abs() >= self.range {
        //                 // Efficient capacity management
        //                 if self.range_bars.len() >= self.range_bars.capacity() {
        //                     self.range_bars.pop_front();
        //                 }
        //                 self.range_bars.push_back(self.create_new_bar(price, time));
        //             }
        //         }
        //     }
        //     None => {
        //         // If there are no bars, create the first one
        //         self.range_bars.push_back(self.create_new_bar(price, time));
        //     }
        // }

        // // Update or create a new RangeBar if necessary
        // if self.range_bars.is_empty()
        //     || !self
        //         .range_bars
        //         .back_mut()
        //         .unwrap()
        //         .update(price, self.range)
        // {
        //     if self.range_bars.len() == self.range_bars.capacity() {
        //         self.range_bars.pop_front();
        //     }
        //     let mut new_bar = RangeBar::new();
        //     new_bar.update(price, self.range); // Initialize the new bar with the current price
        //     self.range_bars.push_back(new_bar);
        // }

        // Update or create a new DeltaBar if necessary
        if self.delta_bars.is_empty()
            || !self
                .delta_bars
                .back_mut()
                .unwrap()
                .update(delta, self.range, time)
        {
            if self.delta_bars.len() == self.delta_bars.capacity() {
                self.delta_bars.pop_front();
            }
            let mut new_bar = DeltaBar::new();
            new_bar.update(delta, self.range, time); // Initialize the new bar with the current delta
            self.delta_bars.push_back(new_bar);
        }
    }

    pub fn get_bars(&self) -> (VecDeque<RangeBar>, VecDeque<DeltaBar>) {
        (self.range_bars.clone(), self.delta_bars.clone())
    }

    // Function to create and initialize a new bar
    // fn create_new_bar(&self, price: f64, time: f64) -> RangeBar {
    //     let mut new_bar = RangeBar::new();
    //     new_bar.update(price, self.range, time); // Initialize the new bar with the current price
    //     new_bar
    // }
}
