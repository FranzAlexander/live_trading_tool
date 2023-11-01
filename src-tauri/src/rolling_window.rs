use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollingWindow<T> {
    window: Vec<T>,
    start: usize,
    end: usize,
}

impl<T> RollingWindow<T> {
    pub fn new(capacity: usize) -> Self {
        RollingWindow {
            window: Vec::with_capacity(capacity),
            start: 0,
            end: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.window.len() < self.window.capacity() {
            self.window.push(value);
            self.end += 1;
        } else {
            // Wrap around the index before using it
            if self.end >= self.window.capacity() {
                self.end = 0;
            }
            self.window[self.end] = value;

            self.end += 1;

            // Wrap around again if needed
            if self.end >= self.window.capacity() {
                self.end = 0;
            }

            if self.end == self.start {
                self.start += 1;
                if self.start >= self.window.capacity() {
                    self.start = 0;
                }
            }
        }
    }

    pub fn prev_value(&self) -> Option<&T> {
        if self.window.is_empty() {
            None
        } else if self.end == 0 {
            self.window.last()
        } else {
            self.window.get(self.end - 1)
        }
    }

    // Method to get a mutable reference to the current value
    pub fn current_value_mut(&mut self) -> Option<&mut T> {
        if self.window.is_empty() {
            None
        } else if self.end == 0 {
            self.window.last_mut()
        } else {
            self.window.get_mut(self.end - 1)
        }
    }
}

impl<T: Clone> RollingWindow<T> {
    pub fn ordered_elements(&self) -> Vec<T> {
        let mut ordered = Vec::with_capacity(self.window.len());

        if !self.window.is_empty() {
            let mut idx = self.start;
            loop {
                ordered.push(self.window[idx].clone());
                idx += 1;
                if idx == self.window.capacity() {
                    idx = 0;
                }
                if idx == self.end {
                    break;
                }
            }
        }

        ordered
    }
}
