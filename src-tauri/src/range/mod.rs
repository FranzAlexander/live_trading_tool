pub trait Bar {
    fn update(&mut self, value: f64, range: f64) -> bool;
    fn reset(&mut self);
}
