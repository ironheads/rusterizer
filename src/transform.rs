use crate::{
    la::Vec3f,
    la::Matrix,
    la::MatrixI,
};

const Z_DEPTH:f32 = 255.0;


pub fn get_viewport_matrix(w:i32, h:i32) -> Matrix<4,4> {
    let mut viewport = Matrix::identity();
    viewport[0][0] = w as f32/ 2.;
    viewport[1][1] = h as f32/ 2.;
    viewport[2][2] = Z_DEPTH / 2.;

    viewport[0][3] = (w-1) as f32/ 2.;
	viewport[1][3] = (h-1) as f32/ 2.;
	viewport[2][3] = Z_DEPTH / 2.;

    viewport
}

pub fn calculate_lookat_matrix(position: Vec3f, view : Vec3f, up_vector: Vec3f) -> Matrix<4,4> {
    let z = position.sub(&view).normalize();
    let x = up_vector.cross(&z).normalize();
    let y = z.cross(&x).normalize();

    let minv = [
        [x.0, x.1, x.2, 0.0],
        [y.0, y.1, y.2, 0.0],
        [z.0, z.1, z.2, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    let tr= [
        [1.0, 0.0, 0.0, -position.0],
        [0.0, 1.0, 0.0, -position.1],
        [0.0, 0.0, 1.0, -position.2],
        [0.0, 0.0, 0.0, 1.0],
    ];

    minv.mul(&tr) // 4x4
}


pub fn calculate_prespective_projection(fovy_in_radians:f32,aspect:f32, z_near:f32, z_far:f32, zoom: f32) -> Matrix<4,4>{
    let mut projection = Matrix::identity();
    let theta = fovy_in_radians/2f32;
    let zoom_tan_theta = theta.tan() / zoom;
    projection[0][0] = 1.0/(aspect*zoom_tan_theta);
    projection[1][1] = 1.0 / zoom_tan_theta;
    projection[2][2] = (z_far+z_near)/(z_near-z_far);

    projection[2][3] = 2f32*z_far*z_near/(z_near-z_far);
    projection[3][2] = -1f32;
    projection
    // Matrix::identity()
}

// calculate the bary centric coordniates 
// point p is in tri(a,b,c)
// get the Vec3f(u,v,w) where ua+vb+wc = p && u+v+w=1
pub fn barycentric(a: &Vec3f, b: &Vec3f, c: &Vec3f, p: (f32, f32)) -> Vec3f {
    let cross =
        Vec3f(c.0 - a.0, b.0 - a.0, a.0 - p.0).cross(&Vec3f(c.1 - a.1, b.1 - a.1, a.1 - p.1));
    Vec3f(
        1.0 - (cross.1 + cross.0) / cross.2,
        cross.1 / cross.2,
        cross.0 / cross.2,
    )
}

