use super::SceneTrait;
use crate::{
    models::MeshObject,
};
pub struct RasterizableScene {
    // objects: ,
    objects: Vec<Box<MeshObject>>,
}

