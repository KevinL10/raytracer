#[derive(Debug, Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, t: f64) -> bool {
        self.min <= t && t <= self.max
    }

    pub fn surrounds(&self, t: f64) -> bool {
        self.min < t && t < self.max
    }
}
