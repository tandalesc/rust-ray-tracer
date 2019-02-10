
use cgmath::{Vector3, InnerSpace};
use crate::ray::{Ray};
use crate::primitives::{Point, Direction, Material, Renderable, Intersectable};

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
}
impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let o_ray: Vector3<f64> = self.center - ray.origin;
        //obj ray projected in direction of light ray
        let o_proj_ray = o_ray.dot(ray.direction);
        //pythagorean theorem -- solve for missing side (skip sqrt)
        let d2 = o_ray.dot(o_ray) - (o_proj_ray*o_proj_ray);
        //if d2 is greater than radius squared, that ray does not intersect this sphere
        let r2 = self.radius * self.radius;
        if d2 > r2 {
            return None;
        }
        //check on both sides of the sphere for the minimum distance
        let thickness = (r2 - d2).sqrt();
        let (t0,t1) = (o_proj_ray-thickness, o_proj_ray+thickness);
        if t0<0.0 && t1<0.0 {
            None
        } else if t0<0.0 {
            Some(t1)
        } else if t1<0.0 {
            Some(t0)
        } else {
            //return minimum distance
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }
}
impl Renderable for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }
    fn surface_normal(&self, hit_point: &Point) -> Direction {
        (*hit_point - self.center).normalize()
    }
}
