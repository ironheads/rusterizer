use crate::camera::{
    CameraTrait,
    Camera,
    Direction,
};

use crate::{
    la::{Vec3f,Matrix},
    utils::random_unit_disk_vector,
};

use super::{
    Ray,
};

pub trait Exposure: CameraTrait {
    fn set_radius(&mut self, radius: f32);
    fn set_aperture(&mut self, aperture: f32);
    fn radius(&self) -> f32;
    fn exposure_ray(&self,s: f32, t:f32) -> Ray;
}


pub struct ExposureCamera {
    camera: Camera,
    // the size of the camera aperture
    aperture: f32,
}

impl Default for ExposureCamera {
    fn default() -> Self {
        Self { camera: Default::default(), aperture: 1f32 }
    }
}
impl CameraTrait for ExposureCamera {
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
    }

    fn get_lookat(&self) -> &Matrix<4,4> {
        self.camera.get_lookat_view()
    }

    fn shift_camera(&mut self,direction:Direction) {
        self.camera.shift_camera(direction);
    }

    fn set_aspect(&mut self, aspect: f32) {
        self.camera.set_aspect(aspect);
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

impl Exposure for ExposureCamera {
    fn exposure_ray(&self,s: f32, t:f32) -> Ray {
        let rd = self.radius() * random_unit_disk_vector();
        let (u,v,_) = self.uvw();

        let offset = rd.x() * u + rd.y() * v;
        let focus_distance = (self.focus()-self.position()).length();
        let theta = self.fov_in_radians();
        Ray {
            origin: self.position() + offset, 
            direction: self.focus() 
                        - (theta/2.0).tan() * self.aspect() * focus_distance * u
                        - (theta/2.0).tan() * focus_distance * v
                        + s * 2.0 * (theta/2.0).tan() * self.aspect() * focus_distance * u
                        + t * 2.0 * (theta/2.0).tan()  * focus_distance * v
                        - self.position() - offset
        }
    }

    fn radius(&self) -> f32 {
        self.aperture / 2.0
    }

    fn set_radius(&mut self, radius: f32) {
        self.aperture = radius * 2.0;
    }

    fn set_aperture(&mut self, aperture: f32) {
        self.aperture = aperture;
    }

}