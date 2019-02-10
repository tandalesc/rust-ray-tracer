
use cgmath::{InnerSpace};
use image::{DynamicImage, GenericImage};

use crate::ray::{Ray};
use crate::primitives::{Point, Object, Color, Direction, SurfaceType::*};
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
    pub lights: Vec<&'a Light>,

    pub shadow_bias: f64,
    pub max_recursion_depth: u32,
}
impl<'a> Scene<'a> {
    pub fn render<'b>(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let black = Color::black().to_rgba();
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = Ray::create_prime(x, y, self);
                if let Some(i) = self.trace(&ray) {
                    image.put_pixel(x, y, get_color(self, &ray, &i, 1).to_rgba());
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

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.object.surface_normal(&hit_point);
    let material = intersection.object.material();
    match material.surface {
        Diffuse => {
            shade_diffuse(scene, hit_point, surface_normal, intersection.object)
        }
        Reflective{ reflectivity, specular_color } => {
            let mut color = shade_diffuse(scene, hit_point, surface_normal, intersection.object);
            let reflection = Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.shadow_bias);
            color *= 1.0 - reflectivity;
            color += cast_ray(scene, &reflection, depth+1) * reflectivity * specular_color;
            color
        }
        Refractive{ index, transparency, reflectivity } => {
            let kr = fresnel(surface_normal, ray.direction, index);
            let refraction_color = if kr < 1.0 {
                let transmission_ray = Ray::create_transmission(surface_normal, ray.direction, hit_point, scene.shadow_bias, index).unwrap();
                cast_ray(scene, &transmission_ray, depth + 1)
            } else { Color::black() };

            let reflection_ray = Ray::create_reflection(surface_normal, ray.direction, hit_point, scene.shadow_bias);
            let reflection_color = cast_ray(scene, &reflection_ray, depth + 1);
            let mut color = shade_diffuse(scene, hit_point, surface_normal, intersection.object);
            color *= 2.0 - transparency - reflectivity;
            color += reflection_color * reflectivity * kr + refraction_color * transparency * (1.0-kr);
            color
        }
    }
}

fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    if depth >= scene.max_recursion_depth {
        return Color::black();
    }

    match scene.trace(&ray) {
        Some(intersect) => {get_color(scene, &ray, &intersect, depth)}
        None => {Color::black()}
    }
}

fn shade_diffuse(scene: &Scene, hit_point: Point, surface_normal: Direction, object: &Object) -> Color {
    let mut color: Color = Color::black();
    for light in &scene.lights {
        let direction_to_light = -light.get_direction(hit_point).normalize();
        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * scene.shadow_bias),
            direction: direction_to_light
        };
        let in_light = match scene.trace(&shadow_ray) {
            Some(i) => {i.distance > light.get_distance_to(hit_point)}
            None => {true}
        };

        let light_intensity = if in_light { light.get_intensity(hit_point) } else { 0.0 };
        let light_power = (surface_normal.dot(direction_to_light) as f64).max(0.0) * light_intensity;
        let light_reflected = object.albedo() / std::f64::consts::PI;
        color += object.color().clone() * light.get_color(hit_point).clone() * light_power * light_reflected;
    }
    color.clamp()
}

fn fresnel(surface_normal: Direction, direction: Direction, index: f64) -> f64 {
    let d_dot_n = direction.dot(surface_normal);
    let mut eta_i = 1.0;
    let mut eta_t = index;
    if d_dot_n > 0.0 {
        eta_i = eta_t;
        eta_t = 1.0;
    }

    let sin_t = eta_i/eta_t * (1.0 - d_dot_n*d_dot_n).max(0.0).sqrt();
    if sin_t > 1.0 {
        //Total internal reflection
        1.0
    } else {
        let cos_t = (1.0 - sin_t*sin_t).max(0.0).sqrt();
        let cos_i = cos_t.abs();
        let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
        let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
        (r_s*r_s + r_p*r_p) / 2.0
    }
}
