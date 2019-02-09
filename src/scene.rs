
use cgmath::{InnerSpace};
use image::{DynamicImage, GenericImage};

use crate::ray::{Ray};
use crate::primitives::{Object, Color};
use crate::light::{Light};

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
    pub lights: Vec<&'a Light>
}
impl<'a> Scene<'a> {
    pub fn render<'b>(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let black = Color::black().to_rgba();
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = Ray::create_prime(x, y, self);
                if let Some(i) = self.trace(&ray) {
                    image.put_pixel(x, y, get_color(self, &ray, &i).to_rgba());
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

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.object.surface_normal(&hit_point);
    let mut color: Color = Color::black();
    for light in &scene.lights {
        let direction_to_light = -light.get_direction().normalize();
        let light_power = (surface_normal.dot(direction_to_light) as f64).max(0.0) * light.get_intensity();
        let light_reflected = intersection.object.albedo() / std::f64::consts::PI;
        color += intersection.object.color().clone() * light.get_color().clone() * light_power * light_reflected;
    }
    color.clamp()
}
