
use crate::primitives::{Color, Direction};

pub struct Light {
    pub direction: Direction,
    pub color: Color,
    pub intensity: f64
}
