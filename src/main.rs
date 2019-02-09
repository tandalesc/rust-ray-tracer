extern crate image;
extern crate cgmath;

mod primitives;
mod ray;
mod scene;
mod sphere;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_scene_render() {
    use image::GenericImageView;
    use crate::scene::{Scene};
    use crate::sphere::{Sphere};
    use crate::primitives::{Point, Color};
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 60.0,
        spheres: vec![
            Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0
                },
                radius: 1.0,
                color: Color {
                    r: 0.4,
                    g: 1.0,
                    b: 0.4
                }
            },
            Sphere {
                center: Point {
                    x: 0.0,
                    y: -2.0,
                    z: -3.0
                },
                radius: 2.0,
                color: Color {
                    r: 1.0,
                    g: 0.8,
                    b: 0.0
                }
            }
        ]
    };
    let image = scene.render();
    let image_path = "test/sample.png";
    image.save(image_path).unwrap();
    assert_eq!((scene.width, scene.height), (image.width(), image.height()));
}
