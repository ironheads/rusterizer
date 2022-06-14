use crate::{
    ray::Ray,
    la::Vec3f,
    materials::Material,
};

use std::sync::Arc;

// the attributes that the ray hit the hittable object
pub struct Hit {
    pub t: f32,
    pub place: Vec3f,
    pub normal: Option<Vec3f>,
    pub front_face: Option<bool>,
    pub material: Option<Arc<dyn Material>>,
}

impl Default for Hit {
    fn default() -> Self {
        Self { 
            t: 0f32, 
            place: Vec3f(0f32, 0f32, 0f32), 
            normal: None, 
            front_face: None, 
            material: None
        }
    }
}
impl Hit {
    pub fn new(t: f32, place: Vec3f) -> Self {
        Self { t, place, normal: None, front_face: None, material: None }
    }
    pub fn set_face_normal(&mut self, ray:&Ray, outward_normal: Vec3f) {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            outward_normal
        };

    }
}

pub trait Hittable : Send + Sync {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Option<Hit>;
}

