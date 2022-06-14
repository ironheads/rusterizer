use std::sync::Arc;

use crate::{
    la::Vec3f,
    ray::Ray,
    hit::{Hittable,Hit},
    materials::Material,
};


pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, t_min: f32, t_max: f32, ray: &Ray) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.

        // Attention: there are two roots?
        // todo!()
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = ray.at(root);
        let mut hit = Hit::new(t, p);
        let outward_normal = (p - self.center) / self.radius;
        hit.set_face_normal(ray, outward_normal);
        hit.material = Some(self.material.clone());
        Some(hit)
    }
}