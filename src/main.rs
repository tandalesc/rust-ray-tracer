extern crate image;
extern crate cgmath;

use cgmath::{Vector3, InnerSpace, Zero};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba, Pixel};

type Color = Vector3<f32>;
type Point = Vector3<f64>;

struct Ray {
    origin: Point,
    direction: Vector3<f64>
}

trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

struct Sphere {
    center: Point,
    radius: f64,
    color: Color,
}
impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        let o_ray: Vector3<f64> = self.center - ray.origin;
        //obj ray projected in direction of light ray
        let o_proj_ray = o_ray.dot(ray.direction);
        //pythagorean theorem -- solve for missing side (skip sqrt)
        let d2 = o_ray.dot(o_ray) - (o_proj_ray*o_proj_ray);
        //if d2 is less than radius squared, that ray intersects this sphere
        d2 < (self.radius * self.radius)
    }
}

struct Scene {
    width: u32,
    height: u32,
    fov: f64,
    sphere: Sphere
}

impl Ray {
    fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
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
}

fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0,0,0,0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);
            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, to_rgba(&scene.sphere.color));
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
}

fn to_rgba(color: &Color) -> Rgba<u8> {
    Rgba::from_channels((color.x * 255.0) as u8,
                        (color.y * 255.0) as u8,
                        (color.z * 255.0) as u8,
                        0)
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_scene_render() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0
            },
            radius: 1.0,
            color: Color {
                x: 0.4,
                y: 1.0,
                z: 0.4
            }
        }
    };
    let image = render(&scene);
    let image_path = "test/sample.png";
    image.save(image_path).unwrap();
    assert_eq!((scene.width, scene.height), (image.width(), image.height()));
}
