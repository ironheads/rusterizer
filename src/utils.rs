use std::f32::consts::PI;
use crate::la::{Matrix,MatrixI,Vec3f};

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
