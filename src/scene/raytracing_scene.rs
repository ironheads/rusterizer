use super::SceneTrait;
use crate::raytracing::{Hittable,Hit,Ray};

pub struct RayTracingScene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl SceneTrait for RayTracingScene {
    type ObjectType = Box<dyn Hittable>;

    fn new() -> Self {
        Self { objects: vec![] }
    }

    fn clear(&mut self) {
        self.objects.clear();
    }

    fn add(&mut self, object: <Self as SceneTrait>::ObjectType) {
        self.objects.push(object);
    }
}



impl Hittable for RayTracingScene {
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


