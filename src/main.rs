extern crate image;
extern crate cgmath;

mod primitives;
mod ray;
mod scene;
mod sphere;
mod plane;
mod light;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_scene_render() {
    use crate::plane::{Plane};
    use crate::scene::{Scene};
    use crate::sphere::{Sphere};
    use crate::light::{Light};
    use crate::primitives::{Point, Direction, Color};
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 60.0,
        objects: vec![
            &Sphere {
                center: Point {x: 0.0, y: 0.0, z: -5.0},
                radius: 1.0,
                color: Color {r: 0.4, g: 1.0, b: 0.4}
            },
            &Sphere {
                center: Point {x: -1.5, y: -1.5, z: -3.0},
                radius: 2.0,
                color: Color {r: 1.0, g: 0.8, b: 0.0}
            },
            &Plane {
                p0: Point {x: 0.0, y:-6.0, z:0.0},
                normal: Direction {x: 0.0, y: -1.0, z: 0.0},
                color: Color {r:0.3, g:0.3, b:0.3}
            }
        ],
        light: Light {
            direction: Direction {x: 0.0, y:-1.0, z: 0.0},
            color: Color {r:0.8, g:0.8, b:0.6},
            intensity: 1.0
        }
    };
    scene.render().save("test/sample.png").unwrap();
}
