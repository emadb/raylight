use std::{f32, f64};

pub struct Direction(f64);

impl Direction {
    pub fn new(degrees: u16) -> Self {
        let theta = degrees as f64 * f64::consts::PI / 180.0;
        Direction(theta)
    }

    pub fn value(self) -> f64 {
        self.0
    }

    pub fn cos(&self) -> f32 {
        let value: f32 = self.0 as f32;
        value.cos()
    }
    pub fn sin(&self) -> f32 {
        let value: f32 = self.0 as f32;
        value.sin()
    }
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub direction: Direction
}

impl Player {
    pub fn new(x: u32, y: u32, direction: f64) -> Self {
        Self { x: x as f32 + 0.5, y: y as f32 + 0.5, direction: Direction(direction) }
    }
}