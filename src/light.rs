
use crate::primitives::{Color, Direction};

pub trait Light {
    fn get_direction(&self) -> Direction;
    fn get_color(&self) -> Color;
    fn get_intensity(&self) -> f64;
}

pub struct DirectionalLight {
    pub direction: Direction,
    pub color: Color,
    pub intensity: f64
}
impl Light for DirectionalLight {
    fn get_direction(&self) -> Direction {self.direction}
    fn get_color(&self) -> Color {self.color}
    fn get_intensity(&self) -> f64 {self.intensity}
}
