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
            -outward_normal
        };
        self.front_face = Some(front_face);
        self.normal = Some(normal);
    }
}

pub trait Hittable : Send + Sync {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Option<Hit>;
}


pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, t_min: f32, t_max: f32, r: &Ray) -> Option<Hit> {
        let mut temp_rec: Option<Hit> = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            let result = object.hit(t_min, closest_so_far, r);
            if let Some(rec) = result {
                // find the closest hittable object in list ant return the Hit struct
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }
}