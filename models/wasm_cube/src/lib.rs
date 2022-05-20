#![doc = include_str!("../README.md")]

use wasm_bindgen::prelude::*;

#[no_mangle]
#[wasm_bindgen]
pub fn model() -> fj::Shape {
    let x: f64 = 3.0;
    let y: f64 = 2.0;
    let z: f64 = 1.0;

    #[rustfmt::skip]
    let rectangle = fj::Sketch::from_points(vec![
        [-x / 2., -y / 2.],
        [ x / 2., -y / 2.],
        [ x / 2.,  y / 2.],
        [-x / 2.,  y / 2.],
    ]).with_color([100,255,0,200]);

    let cuboid = fj::Sweep::from_path(rectangle.into(), [0., 0., z]);

    cuboid.into()
}
