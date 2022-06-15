use std::char::ParseCharError;

use crate::{
    la::Vec3f,
    la::Matrix,
    transform::{calculate_lookat_matrix,calculate_prespective_projection},
    utils::{degrees_to_radians, random_unit_disk_vector}, 
    ray::Ray,
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

pub trait Exposure: CameraTrait {
    fn set_radius(&mut self, radius: f32);
    fn set_aperture(&mut self, aperture: f32);
    fn radius(&self) -> f32;
    fn exposure_ray(&self,s: f32, t:f32) -> Ray;
}


#[derive(Clone, Copy, Debug)]
pub struct Camera {
    // the origin position of the camera
    position: Vec3f,
    // the direction that the camera focus
    view: Vec3f,
    // the distance between the focus and the 
    up_vector: Vec3f,
    // the speed that the camera moves
    speed: f32,
    // look at matrix 
    lookat_matrix: Matrix<4,4>,

    fov: f32,

    aspect: f32,

    zoom: f32,
}

#[derive(Debug)]
pub enum Direction {
    LEFT,
    RIGHT,
    FRONT,
    BACK
}


impl Default for Camera {
    fn default() -> Self {
        Self {
                position: Vec3f(5f32, 5f32, 5f32), 
                view: Vec3f(0f32, 0f32, 0f32) ,
                up_vector: Vec3f(0f32,1f32,0f32),
                speed: 0.3f32,
                aspect: 1f32,
                fov: 50f32,
                lookat_matrix: calculate_lookat_matrix(Vec3f(5f32, 5f32, 5f32), Vec3f(0f32, 0f32, 0f32), Vec3f(0f32,1f32,0f32)),
                zoom: 1f32,
            }
    }
}

impl Camera {

    pub fn new(pos: Vec3f, foc: Vec3f) -> Self {
        Self { position: pos, view: foc, up_vector:Vec3f(0f32, 1f32, 0f32), lookat_matrix: calculate_lookat_matrix(pos,foc, Vec3f(0f32, 1f32, 0f32)), ..Default::default() }
    }
    pub fn get_lookat_view(&self) -> &Matrix<4,4> {
        &self.lookat_matrix
    }

    pub fn update_lookat_matrix(&mut self) {
        self.lookat_matrix = calculate_lookat_matrix(self.position, self.view, self.up_vector)
    }

    pub fn rotate_view(&mut self,angle_in_radians :f32, rotate_diectrion: Vec3f) {
        todo!("ratate the camera according to the rotate direction");
        self.update_lookat_matrix();
    }

    pub fn yaw_camera(&mut self, move_distance:f32) { 
        let z = self.position.sub(&self.view);
        let x = z.cross(&self.up_vector).normalize();
        
        self.position = self.position.add(&x.mulf(move_distance));
        self.view = self.view.add(&x.mulf(move_distance));
        self.update_lookat_matrix();
    }

    pub fn move_camera(&mut self, move_distance:f32) {
        let z = self.view.sub(&self.position).normalize();
        self.position = self.position.add(&z.mulf(move_distance));
        self.view = self.view.add(&z.mulf(move_distance));
        self.update_lookat_matrix();
    }
}

impl CameraTrait for Camera {

    fn u(&self) -> Vec3f {
        (self.up_vector.cross(&self.w())).normalize()
    }

    fn v(&self) -> Vec3f {
        self.w().cross(&self.u())
    }

    fn w(&self) -> Vec3f {
        (self.position-self.view).normalize()
    }

    fn uvw(&self) -> (Vec3f,Vec3f,Vec3f) {
        let w = (self.position-self.view).normalize();
        let u = (self.up_vector.cross(&w)).normalize();
        let v = w.cross(&u).normalize();
        (u,v,w)
    }

    fn set_position(&mut self, pos: Vec3f) {
        self.position=pos;
        self.update_lookat_matrix();
    }
    
    fn set_focus(&mut self, v: Vec3f) {
        self.view=v;
        self.update_lookat_matrix();
    }

    fn set_speed(&mut self, speed:f32) {
        self.speed = speed;
    }

    fn set_up_vector(&mut self, up_vector:Vec3f) {
        self.up_vector = up_vector.normalize();
        self.update_lookat_matrix();
    }

    fn shift_camera(&mut self, direction:Direction) {
        match direction {
            Direction::FRONT => {
                self.move_camera(self.speed);
            }
            Direction::BACK => {
                self.move_camera(-self.speed);
            }
            Direction::LEFT => {
                self.yaw_camera(-self.speed);
            }
            Direction::RIGHT => {
                self.yaw_camera(self.speed);
            }
        }
    }

    fn fov(&self) -> f32 {
        self.fov
    }

    fn fov_in_radians(&self) -> f32 {
        degrees_to_radians(self.fov)
    }

    fn aspect(&self) -> f32 {
        self.aspect
    }

    fn position(&self) -> Vec3f {
        self.position
    }

    fn focus(&self) -> Vec3f {
        self.view
    }

    fn get_speed(&self) -> f32 {
        self.speed
    }

    fn get_up_vector(&self) -> Vec3f {
        self.up_vector
    }

    fn set_zoom(&mut self, zoom:f32) {
        self.zoom = zoom
    }

    fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect
    }

    fn get_lookat(&self) -> &Matrix<4,4> {
        self.get_lookat_view()
    }

    fn zoom(&self) -> f32 {
        self.zoom
    }
    
}


#[derive(Clone, Copy, Debug)]
pub struct PerspectiveCamera {
    pub camera:Camera,
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
        self.camera.position
    }

    fn focus(&self) -> Vec3f {
        self.camera.view
    }

    fn get_speed(&self) -> f32 {
        self.camera.speed
    }

    fn get_up_vector(&self) -> Vec3f {
        self.camera.up_vector
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
                        - (theta/2.0).tan() * focus_distance * v
                        - (theta/2.0).tan() * self.aspect() * focus_distance * u
                        + s * 2.0 * (theta/2.0).tan() * focus_distance * v
                        + t * 2.0 * (theta/2.0).tan() * self.aspect() * focus_distance * u
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