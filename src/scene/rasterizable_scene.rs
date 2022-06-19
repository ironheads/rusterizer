use super::SceneTrait;
use crate::{
    models::MeshObject,
};
pub struct RasterizableScene {
    // objects: ,
    pub objects: Vec<Box<MeshObject>>,
}

impl SceneTrait for RasterizableScene {
    type ObjectType = Box<MeshObject>;

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

