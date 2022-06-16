use crate::tga::Image;

pub trait ModelTrait {
    fn texture(&self) -> Image;
    fn normal_map(&self) -> Image;
    
}