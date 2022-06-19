use std::ops::{Neg, Add, Sub, Mul, Div};
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use lodepng::RGB;
use crate::tga;

use super::{
    Matrix,
    MatrixI,
};
#[derive(Clone, Debug, Copy)]
pub struct Vec3f(pub f32, pub f32, pub f32);

impl Default for Vec3f {
    fn default() -> Self {
        Self(Default::default(), Default::default(), Default::default())
    }
}

impl Neg for Vec3f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3f(-self.0,-self.1,-self.2)
    }
}

impl Add<Vec3f> for Vec3f {
    type Output = Self;

    fn add(self, rhs: Vec3f) -> Self::Output {
        Vec3f(self.0+rhs.0,self.1+rhs.1,self.2+rhs.2)
    }
}

impl Sub<Vec3f> for Vec3f {
    type Output = Self;
    fn sub(self, rhs: Vec3f) -> Self::Output {
        Vec3f(self.0-rhs.0,self.1-rhs.1,self.2-rhs.2)
    }
}

impl Mul<Vec3f> for Vec3f {
    type Output = Self;
    fn mul(self, rhs: Vec3f) -> Self::Output {
        Vec3f(self.0*rhs.0,self.1*rhs.1,self.2*rhs.2)
    }
}

impl Mul<f32> for Vec3f {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3f(self.0 * rhs, self.1 * rhs, self.2 *rhs)
    }
}

impl Mul<Vec3f> for f32 {
    type Output = Vec3f;
    fn mul(self, rhs: Vec3f) -> Self::Output {
        rhs*self
    }
}

impl Div<f32> for Vec3f {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3f(self.0/rhs,self.1/rhs,self.2/rhs)
    }
}

// generate the standard distribution of vec3f
impl Distribution<Vec3f> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3f {
        Vec3f(rng.gen(), rng.gen(), rng.gen())
    }
}


impl Vec3f {
    pub fn zeroed() -> Self {
        Vec3f(0.0, 0.0, 0.0)
    }

    pub fn to_u8(&self) -> [u8; 3] {
        fn u(f: f32) -> u8 {
            if f < 0.0 {
                0
            } else if f >= 1.0 {
                255
            } else {
                (f * 255.999) as i32 as u8
            }
        }
        [u(self.0), u(self.1), u(self.2)]
    }

    pub fn to_rgb(&self) -> RGB<u8> {
        let rgb = &self.to_u8();
        RGB::new(rgb[0], rgb[1], rgb[2])
    }

    pub fn to_rgb_sampled(&self, samples_per_pixel: usize) -> RGB<u8> {
        let scale = 1.0 / (samples_per_pixel as f32);
        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let r = (scale * self.0 as f32).sqrt();
        let r = (256.0 * f32::clamp(r, 0.0, 0.999)) as u8;
        let g = (scale * self.1 as f32).sqrt();
        let g = (256.0 * f32::clamp(g, 0.0, 0.999)) as u8;
        let b = (scale * self.2 as f32).sqrt();
        let b = (256.0 * f32::clamp(b, 0.0, 0.999)) as u8;
        RGB::new(r, g, b)
    }

    pub fn to_color(&self, samples_per_pixel: usize) -> tga::Color {
        let scale = 1.0 / (samples_per_pixel as f32);
        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let r = (scale * self.0 as f32).sqrt();
        let r = (256.0 * f32::clamp(r, 0.0, 0.999)) as u8;
        let g = (scale * self.1 as f32).sqrt();
        let g = (256.0 * f32::clamp(g, 0.0, 0.999)) as u8;
        let b = (scale * self.2 as f32).sqrt();
        let b = (256.0 * f32::clamp(b, 0.0, 0.999)) as u8;
        tga::Color(r,g,b)
    }

    pub fn embed<const L: usize>(&self, i: f32) -> Matrix<1, L> {
        assert!(L > 3);
        let mut v = [[i]; L];
        v[0][0] = self.0;
        v[1][0] = self.1;
        v[2][0] = self.2;
        v
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(&self)
    }

    pub fn length(&self) -> f32 {
        self.dot(&self).sqrt()
    }



    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }
    
    pub fn cross(&self, v: &Vec3f) -> Self {
        Vec3f(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
        )
    }

    pub fn normalize(&self) -> Self {
        let mag = self.length();
        let mag = if mag == 0.0 { f32::MIN } else { mag };
        Vec3f(self.0 / mag, self.1 / mag, self.2 / mag)
    }

    pub fn sub(&self, v: &Vec3f) -> Self {
        Vec3f(self.0 - v.0, self.1 - v.1, self.2 - v.2)
    }

    pub fn add(&self, v: &Vec3f) -> Self {
        Vec3f(self.0 + v.0, self.1 + v.1, self.2 + v.2)
    }

    pub fn mul(&self, v: &Vec3f) -> f32 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    pub fn dot(&self, v: &Vec3f) ->f32 {
        self.mul(v)
    }

    pub fn mulf(&self, v: f32) -> Vec3f {
        Vec3f(self.0 * v, self.1 * v, self.2 * v)
    }

    pub fn rotate(&self, x: f32, y: f32) -> Vec3f {
        let xm: Matrix<3, 3> = [
            [1.0, 0.0, 0.0],
            [0.0, x.cos(), -x.sin()],
            [0.0, x.sin(), x.cos()],
        ];

        let ym: Matrix<3, 3> = [
            [y.cos(), 0.0, y.sin()],
            [0.0, 1.0, 0.0],
            [-y.sin(), 0.0, y.cos()],
        ];

        ym.mul(&xm.mul(&self.into())).into()
    }

    /// Return true if the vector is close to zero in all dimensions.
    pub fn near_zero(&self) -> bool {
        let s: f32 = 1e-8;
        (libm::fabsf(self.0) < s) && (libm::fabsf(self.1) < s) && (libm::fabsf(self.2) < s)
    }
}

impl Into<Matrix<1, 3>> for &Vec3f {
    fn into(self) -> Matrix<1, 3> {
        [[self.0], [self.1], [self.2]]
    }
}


pub fn find_t(a: f32, b: f32, m: f32) -> f32 {
    (m - a) / (b - a)
}

pub fn interpolate(a: f32, b: f32, t: f32) -> f32 {
    a * (1f32 - t) + b * t
}

pub fn interpolatev(a: &Vec3f, b: &Vec3f, t: f32) -> Vec3f {
    Vec3f(
        interpolate(a.0, b.0, t),
        interpolate(a.1, b.1, t),
        interpolate(a.2, b.2, t),
    )
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3f() {
        let vec = Vec3f(2.0,2.0,2.0);
        println!("{:?}",vec.length_squared());
    }
    // #[test]
    // fn test_time() {
    //     let mut acc = 0.0;
    //     let matrices: Vec<(Matrix, Matrix)> = (0..100).map(|_| (Matrix::random(200, 200), Matrix::random(200, 200))).collect();
    //     let start = Instant::now();
    //     for (m1, m2) in matrices {
    //         let r = m2.mul(&m1);
    //         acc += r.0[0][0];
    //     }
    //     println!("{:?}, blackhole: {:?}", start.elapsed(), acc);
    // }

    #[test]
    fn test_matrix_inv() {
        // let m = Matrix(vec![
        //     vec![1.0, 2.0, 1.0, 0.0],
        //     vec![0.0, 2.0, 1.0, 3.0],
        //     vec![1.0, 2.0, 0.0, 1.0],
        //     vec![1.0, 2.0, 0.0, 3.0],
        // ]);
        // println!("{:?}", m.inverse());
    }

    #[test]
    fn test_matrix() {
        // assert_eq!(
        //     0.0,
        //     get_angle(&(0.0, 0.0, 0.0), &(0.0, 0.0, 0.0), &(0.0, 0.0, 0.0))
        // );
        // let mut m1 = Matrix::zeroed(3, 3);
        // m1.0 = vec![vec![1.0, 1.1, 1.2]];
        // let mut m2 = Matrix::zeroed(3, 3);
        // m2.0 = vec![vec![1.0], vec![2.0], vec![3.0]];

        // // let v = Vec3f(1.0, 2.0, 3.0);

        // println!("{:?}", m1.mul(&m2));
        // println!("{:?}", m2.mul(v.into()));
        // println!("{:?}", m1.transpose());
    }
}
