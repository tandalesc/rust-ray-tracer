# Rust Ray Tracer

A simple ray tracer written in Rust.

Features (so far):
- Supported Shapes: Spheres, Planes
- Supported Lights: Directional, Point
- Material Types: Diffuse, Reflective, Refractive
- Gamma Correction
- Simple Shadows

## Test

Run `cargo test` to render a test scene to `test/sample.png`.

## Use

See the test case in `main.rs` for how to get started. Roughly, you want to
create a Scene struct and call the `render` method. You can then save the file
or apply your own post-processing.
