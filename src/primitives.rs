
use cgmath::Vector3;

use crate::ray::Ray;

pub type Point = Vector3<f64>;
pub type Direction = Vector3<f64>;
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub trait Renderable {
    fn color(&self) -> &Color;
    fn surface_normal(&self, _: &Point) -> Vector3<f64>;
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}
