use cgmath::{InnerSpace, Zero, MetricSpace};
use crate::primitives::{Point, Color, Direction};

pub trait Light {
    fn get_direction(&self, hit_point: Point) -> Direction;
    fn get_color(&self, hit_point: Point) -> Color;
    fn get_intensity(&self, hit_point: Point) -> f64;
    fn get_distance_to(&self, other: Point) -> f64;
}

pub struct DirectionalLight {
    pub direction: Direction,
    pub color: Color,
    pub intensity: f64
}
impl Light for DirectionalLight {
    fn get_direction(&self, _: Point) -> Direction {self.direction}
    fn get_color(&self, _: Point) -> Color {self.color}
    fn get_intensity(&self, _: Point) -> f64 {self.intensity}
    fn get_distance_to(&self, _: Point) -> f64 {std::f64::MAX}
}

pub struct PointLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f64
}
impl Light for PointLight {
    fn get_direction(&self, hit_point: Point) -> Direction {
        (hit_point - self.position).normalize()
    }
    fn get_color(&self, _: Point) -> Color {self.color}
    fn get_intensity(&self, hit_point: Point) -> f64 {
        let r2 = (hit_point - self.position).distance(Point::zero());
        self.intensity / (4.0 * std::f64::consts::PI * r2)
    }
    fn get_distance_to(&self, other: Point) -> f64 {
        self.position.distance(other)
    }
}
