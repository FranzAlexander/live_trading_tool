use crate::range::delta_bar::DeltaBar;

pub struct MinData {
    pub end_time: i64,
    pub cumlutive_delta: Option<DeltaBar>,
}

impl MinData {
    pub fn new() -> Self {
        MinData {
            end_time: 0,
            cumlutive_delta: None,
        }
    }
}
