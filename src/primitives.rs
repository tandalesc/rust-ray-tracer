
use std::ops::{Mul,Add,AddAssign,MulAssign};
use cgmath::{Vector3};
use image::{Rgba, Pixel};

use crate::ray::Ray;

//Types
pub type Point = Vector3<f64>;
pub type Direction = Vector3<f64>;

//Traits
pub trait Renderable {
    fn material(&self) -> &Material;
    fn surface_normal(&self, _: &Point) -> Direction;
    fn color(&self) -> &Color {
        &self.material().color
    }
    fn albedo(&self) -> f64 {
        self.material().albedo
    }
}
pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}
pub trait Object: Renderable + Intersectable {}
impl<T: Renderable + Intersectable> Object for T {}

//Enums
#[derive(Debug, Clone, Copy)]
pub enum SurfaceType {
    Diffuse,
    Reflective {
        reflectivity: f64,
        specular_color: Color,
    },
    Refractive {
        index: f64,
        transparency: f64,
        reflectivity: f64,
    }
}

//Structs
#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub albedo: f64,
    pub surface: SurfaceType,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Color {
    pub fn clamp(&self) -> Self {
        Color {
            r: self.r.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
        }
    }
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
    pub fn black() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0
        }
    }
}
impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}
impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}
impl Mul for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            b: self.b * other.b,
            g: self.g * other.g,
        }
    }
}
impl MulAssign for Color {
    fn mul_assign(&mut self, other: Color) {
        self.r *= other.r;
        self.g *= other.g;
        self.b *= other.b;
    }
}
impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, other: f32) -> Color {
        Color {
            r: self.r * other,
            b: self.b * other,
            g: self.g * other,
        }
    }
}
impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, other: f32) {
        self.r *= other;
        self.g *= other;
        self.b *= other;
    }
}
impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, other: f64) {
        *self *= other as f32;
    }
}
impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}
impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, other: f64) -> Color {
        self * other as f32
    }
}
fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0/2.2)
}
fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(2.2)
}
