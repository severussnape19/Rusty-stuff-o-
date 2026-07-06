use core::f32;

pub struct Interval {
    pub min: f32,
    pub max: f32
}

impl Interval {
    pub fn new() -> Self { Self { min: f32::INFINITY, max: f32::NEG_INFINITY } }
    pub fn from(min: f32, max: f32) -> Self { Self { min, max } }
    pub fn contains(&self, val: f32) -> bool { val >= self.min && val <= self.max }
    pub fn surrounds(&self, val: f32) -> bool { val > self.min && val < self.max }
    pub const EMPTY: Interval = Interval { min: f32::INFINITY, max: f32::NEG_INFINITY };
    pub const UNIVERSE: Interval = Interval { min: f32::NEG_INFINITY, max: f32::INFINITY };

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
