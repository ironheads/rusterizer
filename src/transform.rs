use crate::{
    la::Vec3f,
    la::Matrix,
    la::MatrixI,
};

const Z_DEPTH:f32 = 255.0;

pub fn viewport(v: &Vec3f, width: i32, height: i32) -> Vec3f {
    let x0 = (v.0 + 1.) * (width - 1) as f32 / 2.;
    let y0 = (v.1 + 1.) * (height - 1) as f32 / 2.;
    Vec3f(x0, y0, ((v.2 + 1.) / 2.) * 255.0)
}

pub fn get_viewport_matrix(w:i32, h:i32) -> Matrix<4,4> {
    let mut viewport = Matrix::identity();
    viewport[0][0] = w as f32/ 2.;
    viewport[1][1] = h as f32/ 2.;
    viewport[2][2] = Z_DEPTH / 2.;

    viewport[0][3] = w as f32/ 2.;
	viewport[1][3] = h as f32/ 2.;
	viewport[2][3] = Z_DEPTH / 2.;

    viewport
}

pub fn get_prespective_projection(fovyInRadians:f32,aspect:f32, zNear:f32, zFar:f32) -> Matrix<4,4>{
    let mut projection = Matrix::identity();
    let theta = fovyInRadians/2f32;
    projection[0][0] = 1.0/(aspect*theta.tan());
    projection[1][1] = 1.0 / theta.tan();
    projection[2][2] = (zFar+zNear)/(zNear-zFar);

    projection[2][3] = 2f32*zFar*zNear/(zNear-zFar);
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

