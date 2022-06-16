use crate::la::Vec3f;

pub trait Shader {
    fn vertex(&mut self, face: usize, vertex: usize) -> Vec3f;
    fn fragment(&mut self, bar: &Vec3f);
}

