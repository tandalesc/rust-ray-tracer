
use cgmath::{Vector3, InnerSpace, Zero};

use crate::scene::{Scene};
use crate::primitives::{Point, Direction};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3<f64>
}
impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Self {
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;
        Ray {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0
            }.normalize()
        }
    }
    pub fn create_reflection(surface_normal: Direction, direction: Direction, hit_point: Point, bias: f64) -> Self {
        Ray {
            origin: hit_point + (surface_normal * bias),
            direction: direction - (2.0 * direction.dot(surface_normal) * surface_normal)
        }
    }
    pub fn create_transmission(surface_normal: Direction, direction: Direction, hit_point: Point, bias: f64, index: f64) -> Option<Ray> {
        let mut d_dot_n = direction.dot(surface_normal);
        let mut eta_t = index;
        let mut eta_i = 1.0;
        let mut ref_n = surface_normal;
        if d_dot_n < 0.0 {
            d_dot_n = -d_dot_n;
        } else {
            ref_n = -surface_normal;
            eta_t = 1.0;
            eta_i = index;
        }

        let eta = eta_i / eta_t;
        let k = 1.0 - (eta*eta) * (1.0 - d_dot_n*d_dot_n);
        if k < 0.0 {
            None
        } else {
            Some(Ray {
                origin: hit_point + (ref_n * -bias),
                direction: (direction + d_dot_n * ref_n) * eta - ref_n * k.sqrt()
            })
        }
    }
}
