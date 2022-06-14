use crate::{
    la::Vec3f,
    la::Matrix,
    transform::{calculate_lookat_matrix,calculate_prespective_projection},
    utils::degrees_to_radians,
};

pub trait CameraTrait {
    fn position(&self) -> Vec3f;
    fn get_focus(&self) -> Vec3f;
    fn get_speed(&self) -> f32;
    fn get_up_vector(&self) -> Vec3f;
    fn get_zoom(&self) -> f32;
    fn set_position(&mut self, pos: Vec3f);
    fn set_up_vector(&mut self, v:Vec3f);
    fn set_speed(&mut self, speed:f32);
    fn set_focus(&mut self, focus:Vec3f);
    fn set_zoom(&mut self, zoom:f32);
    fn get_lookat(&self) -> &Matrix<4,4>;
    fn get_projection(&self) -> &Matrix<4,4>;
    fn shift_camera(&mut self,direction:Direction);
    fn update_projection(&mut self);
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
                lookat_matrix: calculate_lookat_matrix(Vec3f(5f32, 5f32, 5f32), Vec3f(0f32, 0f32, 0f32), Vec3f(0f32,1f32,0f32))
            }
    }
}


impl Camera {

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
        self.up_vector = up_vector;
        self.update_lookat_matrix();
    }

    pub fn new(pos: Vec3f, foc: Vec3f) -> Self {
        Self { position: pos, view: foc, up_vector:Vec3f(0f32, 1f32, 0f32), lookat_matrix: calculate_lookat_matrix(pos,foc, Vec3f(0f32, 1f32, 0f32)), ..Default::default() }
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

    pub fn shift_camera(&mut self, direction:Direction) {
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

    pub fn get_lookat_view(&self) -> &Matrix<4,4> {
        &self.lookat_matrix
    }



    pub fn update_lookat_matrix(&mut self) {
        self.lookat_matrix = calculate_lookat_matrix(self.position, self.view, self.up_vector)
    }
    
}


#[derive(Clone, Copy, Debug)]
pub struct PerspectiveCamera {
    pub camera:Camera,
    // fov
    pub fov: f32,
    // zoom ratio
    pub zoom: f32,
    // z_near,
    pub znear: f32,
    // z_far,
    pub zfar: f32,
    // aspect,
    pub aspect: f32,
    // projection matrix
    pub projection_matrix: Matrix<4,4>,
    
}

impl PerspectiveCamera {
    pub fn new(fov: f32, aspect: f32, znear:f32, zfar:f32) -> Self {
        Self{
            fov: fov,
            zoom: 1f32,
            znear: znear,
            zfar: zfar,
            aspect: aspect,
            projection_matrix: calculate_prespective_projection(degrees_to_radians(fov), aspect, znear, zfar, 1f32),
            ..Default::default()
        }
    }
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        Self { 
            camera: Default::default(), 
            fov: 50f32,
            znear: 0.01f32,
            zfar: 1000f32,
            aspect: 1f32,
            zoom: 1f32,
            projection_matrix: calculate_prespective_projection(degrees_to_radians(50f32), 1f32, 0.01f32, 1000f32, 1f32),
        }
    }
}

impl CameraTrait for PerspectiveCamera {
    fn get_lookat(&self) -> &Matrix<4,4> {
        self.camera.get_lookat_view()
    }

    fn get_projection(&self) -> &Matrix<4,4> {
        &self.projection_matrix
    }

    fn update_projection(&mut self) {
        self.projection_matrix = calculate_prespective_projection(degrees_to_radians(self.fov), self.aspect, self.znear,self.zfar, self.zoom);
    }

    fn position(&self) -> Vec3f {
        self.camera.position
    }

    fn get_focus(&self) -> Vec3f {
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

    fn get_zoom(&self) -> f32 {
        self.zoom
    }

    fn set_zoom(&mut self, zoom:f32) {
        self.zoom = zoom;
        self.update_projection();
    }

    fn shift_camera(&mut self, direction:Direction) {
        self.camera.shift_camera(direction)
    }

}

