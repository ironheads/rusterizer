use crate::{
    la::{Vec3f,Matrix},
    utils::{degrees_to_radians},
    transform::{calculate_prespective_projection},
};

use super::{
    Camera,
    CameraTrait,
    Direction,
    Projectable,
};

#[derive(Clone, Copy, Debug)]
pub struct PerspectiveCamera {
    pub camera: Camera,
    // z_near,
    pub znear: f32,
    // z_far,
    pub zfar: f32,
    // projection matrix
    pub projection_matrix: Matrix<4,4>,
    
}

impl PerspectiveCamera {
    pub fn new(fov: f32, aspect: f32, znear:f32, zfar:f32) -> Self {
        Self{
            znear: znear,
            zfar: zfar,
            projection_matrix: calculate_prespective_projection(degrees_to_radians(fov), aspect, znear, zfar, 1f32),
            camera: Camera{
                fov: fov,
                aspect: aspect,
                ..Default::default()
            }
        }
    }
}



impl Default for PerspectiveCamera {
    fn default() -> Self {
        Self { 
            camera: Default::default(), 
            znear: 0.01f32,
            zfar: 1000f32,
            projection_matrix: calculate_prespective_projection(degrees_to_radians(50f32), 1f32, 0.01f32, 1000f32, 1f32),
        }
    }
}

impl CameraTrait for PerspectiveCamera {
    fn get_lookat(&self) -> &Matrix<4,4> {
        self.camera.get_lookat_view()
    }

    fn position(&self) -> Vec3f {
        self.camera.position()
    }

    fn focus(&self) -> Vec3f {
        self.camera.focus()
    }

    fn get_speed(&self) -> f32 {
        self.camera.get_speed()
    }

    fn get_up_vector(&self) -> Vec3f {
        self.camera.get_up_vector()
    }

    fn set_position(&mut self, pos: Vec3f) {
        self.camera.set_position(pos);
    }

    fn set_up_vector(&mut self, v:Vec3f) {
        self.camera.set_up_vector(v);
    }

    fn set_speed(&mut self, speed:f32) {
        self.camera.set_speed(speed);
    }

    fn set_focus(&mut self, focus:Vec3f) {
        self.camera.set_focus(focus);
    }

    fn set_zoom(&mut self, zoom:f32) {
        self.camera.set_zoom(zoom);
        self.update_projection();
    }

    fn shift_camera(&mut self, direction:Direction) {
        self.camera.shift_camera(direction)
    }

    fn set_aspect(&mut self, aspect: f32) {
        self.camera.set_aspect(aspect);
        self.update_projection();
    }

    fn u(&self) -> Vec3f {
        self.camera.u()
    }

    fn v(&self) -> Vec3f {
        self.camera.v()
    }

    fn w(&self) -> Vec3f {
        self.camera.w()
    }

    fn uvw(&self) -> (Vec3f,Vec3f,Vec3f) {
        self.camera.uvw()
    }

    fn zoom(&self) -> f32 {
        self.camera.zoom()
    }

    fn fov(&self) -> f32 {
        self.camera.fov()
    }

    fn fov_in_radians(&self) -> f32 {
        self.camera.fov_in_radians()
    }

    fn aspect(&self) -> f32 {
        self.camera.aspect()
    }

}

impl Projectable for PerspectiveCamera {
    fn get_projection(&self) -> &Matrix<4,4> {
        &self.projection_matrix
    }

    fn update_projection(&mut self) {
        self.projection_matrix = calculate_prespective_projection(self.fov_in_radians(), self.aspect(), self.znear,self.zfar, self.zoom());
    }
}