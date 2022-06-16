use crate::{
    la::{Vec3f,Matrix},
};

use super::{
    Direction,
};

pub trait CameraTrait {
    fn u(&self) -> Vec3f;
    fn v(&self) -> Vec3f;
    fn w(&self) -> Vec3f;
    fn uvw(&self) -> (Vec3f,Vec3f,Vec3f);
    fn zoom(&self) -> f32;
    fn fov(&self) -> f32;
    fn fov_in_radians(&self) -> f32;
    fn aspect(&self) -> f32;
    fn position(&self) -> Vec3f;
    fn focus(&self) -> Vec3f;
    fn get_speed(&self) -> f32;
    fn get_up_vector(&self) -> Vec3f;
    fn set_position(&mut self, pos: Vec3f);
    fn set_up_vector(&mut self, v:Vec3f);
    fn set_speed(&mut self, speed:f32);
    fn set_focus(&mut self, focus:Vec3f);
    fn set_zoom(&mut self, zoom:f32);
    fn set_aspect(&mut self, aspect: f32);
    fn get_lookat(&self) -> &Matrix<4,4>;
    fn shift_camera(&mut self,direction:Direction);
}

pub trait Projectable: CameraTrait {
    fn get_projection(&self) -> &Matrix<4,4>;
    fn update_projection(&mut self);
}
