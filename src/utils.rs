use std::f32::consts::PI;
use crate::la::{Matrix,MatrixI,Vec3f};

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn calculate_lookat_matrix(position: Vec3f, view : Vec3f, up_vector: Vec3f) -> Matrix<4,4> {
    let z = position.sub(&view).normalize();
    let x = up_vector.cross(&z).normalize();
    let y = z.cross(&x).normalize();

    let minv = [
        [x.0, x.1, x.2, 0.0],
        [y.0, y.1, y.2, 0.0],
        [z.0, z.1, z.2, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    let tr= [
        [1.0, 0.0, 0.0, -position.0],
        [0.0, 1.0, 0.0, -position.1],
        [0.0, 0.0, 1.0, -position.2],
        [0.0, 0.0, 0.0, 1.0],
    ];

    minv.mul(&tr) // 4x4
}