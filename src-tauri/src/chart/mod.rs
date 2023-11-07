use crate::range::delta_bar::DeltaBar;

pub struct MinData {
    pub name: String,
    pub end_time: i64,
    pub cumlutive_delta: Option<DeltaBar>,
}

impl MinData {
    pub fn new(name: String) -> Self {
        MinData {
            name,
            end_time: 0,
            cumlutive_delta: None,
        }
    }

    pub fn update(&mut self, price: f64, size: f64, side: &str, time: i64) -> Option<DeltaBar> {
        let delta = if side == "b" { size } else { -size };

        if let Some(current_delta) = &mut self.cumlutive_delta {
            current_delta.update(delta);
            if time > self.end_time {
                let delta_bar = current_delta.clone();
                let round_num: i64 = if self.name == "ohlc-1" { 60 } else { 300 };
                self.end_time = time;
                self.cumlutive_delta = Some(DeltaBar::new(
                    self.end_time - round_num,
                    current_delta.cumulative_delta + delta,
                ));

                return Some(delta_bar);
            }
        } else {
            let round_num: i64 = if self.name == "ohlc-1" { 60 } else { 300 };
            self.end_time = time;
            self.cumlutive_delta = Some(DeltaBar::new(self.end_time - round_num, delta));
        }

        None
    }
}
