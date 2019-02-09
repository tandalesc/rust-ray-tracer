
use cgmath::{Vector3};
use image::{Rgba, Pixel};

use crate::ray::Ray;

pub type Point = Vector3<f64>;
pub type Direction = Vector3<f64>;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Color {
    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels((gamma_encode(self.r) * 255.0) as u8,
                            (gamma_encode(self.g) * 255.0) as u8,
                            (gamma_encode(self.b) * 255.0) as u8,
                            0)
    }
    pub fn from_rgba(rgba: Rgba<u8>) -> Self {
        Color {
            r: gamma_decode((rgba.data[0] as f32) / 255.0),
            g: gamma_decode((rgba.data[1] as f32) / 255.0),
            b: gamma_decode((rgba.data[2] as f32) / 255.0)
        }
    }
}
fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0/2.2)
}
fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(2.2)
}

pub trait Renderable {
    fn color(&self) -> &Color;
    fn surface_normal(&self, _: &Point) -> Direction;
}
pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}
pub trait Object: Renderable + Intersectable {}
impl<T: Renderable + Intersectable> Object for T {}
