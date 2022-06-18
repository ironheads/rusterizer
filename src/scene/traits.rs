use crate::models::{MeshTrait};
use std::sync::Arc;

pub trait SceneTrait {
    type ObjectType;

    fn new() -> Self;

    fn clear(&mut self);

    fn add(&mut self, object: <Self as SceneTrait>::ObjectType);
}
