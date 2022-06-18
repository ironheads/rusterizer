use std::any::Any;

use crate::{
    tga::{Image,Color}, 
};
use super::{
    RenderConfig,
};
pub trait Render {
    fn render(&self, camera: &dyn Any, config: RenderConfig) -> Result<Image,&'static str>;
}