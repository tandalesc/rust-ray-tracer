
use image::{DynamicImage, GenericImage, Rgba, Pixel};

use crate::ray::{Ray};
use crate::primitives::{Color, Renderable, Intersectable};
use crate::light::{Light};

pub trait Object: Renderable + Intersectable {}
impl<T: Renderable + Intersectable> Object for T {}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Object,
}
impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, object: &'b Object) -> Intersection<'b> {
        Intersection {
            distance: distance,
            object: object
        }
    }
}

pub struct Scene<'a> {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<&'a Object>,
    pub light: Light
}
impl<'a> Scene<'a> {
    pub fn render<'b>(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let black = Rgba::from_channels(0,0,0,0);
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = Ray::create_prime(x, y, self);
                if let Some(i) = self.trace(&ray) {
                    image.put_pixel(x, y, to_rgba(&i.object.color()));
                } else {
                    image.put_pixel(x, y, black);
                }
            }
        }
        image
    }
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|&s| s.intersect(ray).map(|d| Intersection::new(d,s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

fn to_rgba(color: &Color) -> Rgba<u8> {
    Rgba::from_channels((color.r * 255.0) as u8,
                        (color.g * 255.0) as u8,
                        (color.b * 255.0) as u8,
                        0)
}
