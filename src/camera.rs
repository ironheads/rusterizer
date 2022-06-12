use crate::{
    la::Vec3f,
    la::Matrix,
    la::MatrixI
};
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    // the position of the camera
    pub position: Vec3f,

    // the position that the camera look at
    pub look_at: Vec3f
}

impl Default for Camera {
    fn default() -> Self {
        Self { position: Vec3f(5f32, 5f32, 5f32), look_at: Vec3f::zeroed() }
    }
}
impl Camera {
    pub fn new(pos: Vec3f, foc: Vec3f) -> Self {
        Camera {
            position: pos,
            look_at: foc
        }
    }


    pub fn get_lookat_view(self) -> Matrix<4,4> {
        let up = Vec3f(0.0, 1.0, 0.0);

        let z = self.position.sub(&self.look_at).normalize();
        let x = up.cross(&z).normalize();
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