use cgmath::{InnerSpace};
use crate::ray::{Ray};
use crate::primitives::{Point, Direction, Material, Renderable, Intersectable};

pub struct Plane {
    pub p0: Point,
    pub normal: Direction,
    pub material: Material
}
impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = self.p0 - ray.origin;
            let distance = v.dot(*normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
}
impl Renderable for Plane {
    fn material(&self) -> &Material {
        &self.material
    }
    fn surface_normal(&self, _: &Point) -> Direction {
        -self.normal
    }
}
