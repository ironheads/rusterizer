use crate::la::Vec3f;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

impl Ray {
    pub fn new(origin:Vec3f, direction: Vec3f) -> Self {
        Ray { origin: origin, direction: direction.normalize() }
    }

    pub fn at(&self, t: f32) -> Vec3f {
        self.origin.add(&self.direction.mulf(t))
    }
}