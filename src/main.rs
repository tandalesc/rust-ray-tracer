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
    use crate::primitives::{Point, Direction, Color, Material, SurfaceType::*};
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 60.0,
        objects: vec![
            &Sphere {
                center: Point {x: -1.0, y: 0.0, z: -3.0},
                radius: 1.0,
                material: Material {
                    color: Color {r: 0.1, g: 0.9, b: 0.1},
                    albedo: 1.0,
                    surface: Refractive {
                        index: 1.02,
                        transparency: 0.8,
                        reflectivity: 0.8
                    },
                }
            },
            &Sphere {
                center: Point {x: 0.6, y: 0.0, z: -5.0},
                radius: 1.0,
                material: Material {
                    color: Color {r: 1.0, g: 0.0, b: 0.0},
                    albedo: 1.0,
                    surface: Reflective {
                        reflectivity: 0.8,
                        specular_color: Color {r: 1.0, g: 0.0, b: 0.0}
                    }
                }
            },
            &Sphere {
                center: Point {x: 2.2, y: 0.0, z: -7.0},
                radius: 1.0,
                material: Material {
                    color: Color {r: 0.1, g: 0.2, b: 1.0},
                    albedo: 1.0,
                    surface: Diffuse
                }
            },
            &Plane {
                p0: Point {x: 0.0, y: -1.1, z: 0.0},
                normal: Direction {x: 0.0, y: -1.0, z: 0.0},
                material: Material {
                    color: Color {r: 0.3, g: 0.3, b: 0.3},
                    albedo: 0.8,
                    surface: Reflective {
                        reflectivity: 0.8,
                        specular_color: Color {r: 1.0, g: 1.0, b: 1.0}
                    }
                }
            },
            &Plane {
                p0: Point {x: -3.0, y: 0.0, z: -20.0},
                normal: Direction {x: -1.0, y: 0.0, z: -1.0},
                material: Material {
                    color: Color {r: 0.3, g: 0.3, b: 0.3},
                    albedo: 1.0,
                    surface: Diffuse
                }
            },
            &Plane {
                p0: Point {x: 3.0, y: 0.0, z: -20.0},
                normal: Direction {x: 1.0, y: 0.0, z: -1.0},
                material: Material {
                    color: Color {r: 0.3, g: 0.3, b: 0.3},
                    albedo: 1.0,
                    surface: Diffuse
                }
            },
            &Plane {
                p0: Point {x: 3.0, y: 0.0, z: 20.0},
                normal: Direction {x: 1.0, y: 0.0, z: 1.0},
                material: Material {
                    color: Color {r: 0.3, g: 0.3, b: 0.3},
                    albedo: 1.0,
                    surface: Diffuse
                }
            },
            &Plane {
                p0: Point {x: 3.0, y: 0.0, z: 20.0},
                normal: Direction {x: -1.0, y: 0.0, z: 1.0},
                material: Material {
                    color: Color {r: 0.3, g: 0.3, b: 0.3},
                    albedo: 1.0,
                    surface: Diffuse
                }
            },
        ],
        lights: vec![
            &DirectionalLight {
                direction: Direction {x: 0.0, y: -1.0, z: 0.0},
                color: Color {r: 0.8, g: 0.9, b: 0.9},
                intensity: 1.0
            },
            &PointLight {
                position: Point {x: 0.0, y: 8.0, z: 0.0},
                color: Color {r: 0.8, g: 0.8, b: 0.5},
                intensity: 50.0
            },
            &PointLight {
                position: Point {x: 0.0, y: 10.0, z: -5.0},
                color: Color {r: 1.0, g: 0.5, b: 0.3},
                intensity: 40.0
            },
            &PointLight {
                position: Point {x: -1.0, y: -0.5, z: -1.5},
                color: Color {r: 1.0, g: 0.2, b: 0.1},
                intensity: 30.0
            },
        ],
        shadow_bias: 1e-13,
        max_recursion_depth: 6
    };
    scene.render().save("test/sample.png").unwrap();
}
