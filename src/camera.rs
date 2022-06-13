use crate::{
    la::Vec3f,
    la::Matrix,
    la::MatrixI,
    utils::degrees_to_radians,
};

pub trait CameraTrait {
    fn get_projection(&self) -> Matrix<4,4>;
    
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    // the position of the camera
    pub position: Vec3f,
    // the direction that the camera focus
    pub view: Vec3f,
    // the distance between the focus and the 
    pub up_vector: Vec3f,
    // the speed that the camera moves
    pub speed: f32,
    // fov
    pub fov: f32,
    // zoom
    pub zoom: f32
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
                fov: degrees_to_radians(60f32),
                zoom: 1f32, 
            }
    }
}
impl Camera {
    pub fn new(pos: Vec3f, foc: Vec3f) -> Self {
        Self { position: pos, view: foc, ..Default::default() }
    }

    pub fn rotate_view(&mut self,angle_in_radians :f32, rotate_diectrion: Vec3f) {
        todo!("ratate the camera according to the rotate direction")
    }

    pub fn yaw_camera(&mut self, move_distance:f32) { 
        let z = self.view.sub(&self.position);
        let x = z.cross(&self.up_vector).normalize();
        
        self.position = self.position.add(&x.mulf(move_distance));
        self.view = self.view.add(&x.mulf(move_distance));
    }

    pub fn move_camera(&mut self, move_distance:f32) {
        let z = self.view.sub(&self.position).normalize();
        self.position = self.position.add(&z.mulf(move_distance));
        self.view = self.view.add(&z.mulf(move_distance));
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


    pub fn get_lookat_view(&self) -> Matrix<4,4> {

        let z = self.position.sub(&self.view).normalize();
        let x = self.up_vector.cross(&z).normalize();
        let y = z.cross(&x).normalize();

        let minv = [
            [x.0, x.1, x.2, 0.0],
            [y.0, y.1, y.2, 0.0],
            [z.0, z.1, z.2, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let tr= [
            [1.0, 0.0, 0.0, -self.position.0],
            [0.0, 1.0, 0.0, -self.position.1],
            [0.0, 0.0, 1.0, -self.position.2],
            [0.0, 0.0, 0.0, 1.0],
        ];

        minv.mul(&tr) // 4x4
    }
    
}