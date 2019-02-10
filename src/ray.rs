
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
    pub fn create_reflection(surface_normal: Direction, direction: Direction, hit_point: Point, shadow_bias: f64) -> Self {
        Ray {
            origin: hit_point + (surface_normal * shadow_bias),
            direction: direction - (2.0 * direction.dot(surface_normal) * surface_normal)
        }
    }
}
