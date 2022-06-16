use std::{f32::consts::PI, ops::Mul};
use crate::{
    la::{Vec3f},
};

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

// reflect
pub fn reflect(v: &Vec3f, n: &Vec3f) -> Vec3f {
    v.sub(&v.dot(n).mul(*n).mulf(2f32))
}

// refract 
pub fn refract(v: &Vec3f, n: &Vec3f, etai_over_etat: f32) -> Vec3f {
    let cos_theta = f32::min(v.mulf(-1f32).dot(n), 1.0);
    let r_out_perp = etai_over_etat * (*v + cos_theta * (*n));
    let r_out_parallel = libm::fabsf(1.0 - r_out_perp.length_squared()).sqrt() * -1.0 * *n;
    r_out_perp + r_out_parallel
}

pub fn other_refract(uv: Vec3f, n: Vec3f, etai_over_etat: f32) -> Vec3f {
    let cos_theta = (-uv).dot(&n);
    let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp = -(1.0 - r_out_parallel.length_squared()).sqrt() * n;
    r_out_parallel + r_out_perp
}

