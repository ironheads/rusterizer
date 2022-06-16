use crate::{
    la::Vec3f,
    la::Matrix,
    transform::{calculate_lookat_matrix},
    utils::{degrees_to_radians,}, 
};

use super::{
    CameraTrait,
};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    // the origin position of the camera
    pub position: Vec3f,
    // the direction that the camera focus
    pub view: Vec3f,
    // the distance between the focus and the 
    pub up_vector: Vec3f,
    // the speed that the camera moves
    pub speed: f32,
    // look at matrix 
    pub lookat_matrix: Matrix<4,4>,

    pub fov: f32,

    pub aspect: f32,

    pub zoom: f32,
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


