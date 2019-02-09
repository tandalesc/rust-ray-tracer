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
    use crate::light::{DirectionalLight, PointLight};
    use crate::primitives::{Point, Direction, Color};
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 60.0,
        objects: vec![
            &Sphere {
                center: Point {x: -1.2, y: 0.3, z: -3.0},
                radius: 1.0,
                color: Color {r: 1.0, g: 0.05, b: 0.1}
            },
            &Sphere {
                center: Point {x: 0.0, y: 0.8, z: -5.0},
                radius: 1.0,
                color: Color {r: 0.0, g: 1.0, b: 0.2}
            },
            &Sphere {
                center: Point {x: 2.0, y: 1.3, z: -7.0},
                radius: 1.0,
                color: Color {r: 0.1, g: 0.1, b: 1.0}
            },
            &Plane {
                p0: Point {x: 0.0, y:-2.5, z:0.0},
                normal: Direction {x: 0.0, y: -1.0, z: 0.0},
                color: Color {r:0.3, g:0.3, b:0.3}
            }
        ],
        lights: vec![
            &DirectionalLight {
                direction: Direction {x: 0.0, y: 0.0, z: -1.0},
                color: Color {r: 1.0, g: 1.0, b: 1.0},
                intensity: 0.1
            },
            &PointLight {
                position: Point {x: 0.0, y: 4.0, z: 0.0},
                color: Color {r: 0.8, g: 0.9, b: 0.8},
                intensity: 50.0
            },
            &PointLight {
                position: Point {x: 0.0, y: -0.5, z: -5.0},
                color: Color {r: 0.1, g: 0.9, b: 0.2},
                intensity: 10.0
            },
            &PointLight {
                position: Point {x: -1.0, y: -0.5, z: -1.5},
                color: Color {r: 1.0, g: 0.2, b: 0.1},
                intensity: 20.0
            },
        ],
        shadow_bias: 1e-13
    };
    scene.render().save("test/sample.png").unwrap();
}
