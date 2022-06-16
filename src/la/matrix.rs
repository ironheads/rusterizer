
use super::{
    Vec3f,
};

pub trait MatrixI<const X: usize, const Y: usize> {
    fn zeroed() -> Self;
    fn identity() -> Self;
    fn inverse(&self) -> Self
    where
        [(); X * 2]: Sized;
    fn transpose(&self) -> Matrix<Y, X>;
    fn mul<const XX: usize, const YY: usize>(&self, matrix: &Matrix<XX, YY>) -> Matrix<XX, Y>;
}

pub type Matrix<const X: usize, const Y: usize> = [[f32; X]; Y];

impl<const T: usize> Into<Vec3f> for Matrix<1, T> {
    fn into(self) -> Vec3f {
        assert!(self.len() >= 3);
        assert!(self[0].len() == 1);
        if T == 4 {
            Vec3f(self[0][0]/self[3][0], self[1][0]/self[3][0], self[2][0]/self[3][0])
        } else { 
            Vec3f(self[0][0], self[1][0], self[2][0])
        }
    }
}



impl<const X: usize, const Y: usize> MatrixI<X, Y> for Matrix<X, Y> {
    fn zeroed() -> Self {
        [[0.0f32; X]; Y]
    }

    fn identity() -> Self {
        assert!(X == Y);
        let mut mat = [[0.0f32; X]; Y];
        for i in 0..X {
            mat[i][i]=1.0f32;
        }
        mat
    }

    fn inverse(&self) -> Self
    where
        [(); X * 2]: Sized,
    {
        assert!(self.len() == self[0].len());
        let n = self.len();
        let mut aug = {
            let mut r: Matrix<{ X * 2 }, Y> = Matrix::zeroed();
            for y in 0..n {
                for x in 0..n {
                    r[y][x] = self[y][x];
                }
            }
            for y in 0..n {
                for x in 0..n {
                    r[y][n + x] = if x == y { 1.0 } else { 0.0 };
                }
            }
            r
        };
        for y in 0..n {
            assert!(aug[y][y] != 0.0f32, "it's a bad idea to divide by zero");
            for x in 0..n {
                if x != y {
                    let r = aug[x][y] / aug[y][y];
                    for k in 0..n * 2 {
                        aug[x][k] -= r * aug[y][k];
                    }
                }
            }
        }

        for y in 0..n {
            for x in n..n * 2 {
                aug[y][x] /= aug[y][y];
            }
        }

        let mut res: Matrix<X, Y> = Matrix::zeroed();
        for y in 0..n {
            for x in n..n * 2 {
                res[y][x - n] = aug[y][x];
            }
        }

        res
    }

    fn transpose(&self) -> Matrix<Y, X> {
        let mut res = Matrix::zeroed();
        for x in 0..self[0].len() {
            for y in 0..self.len() {
                res[x][y] = self[y][x];
            }
        }
        res
    }

    fn mul<const XX: usize, const YY: usize>(&self, matrix: &Matrix<XX, YY>) -> Matrix<XX, Y> {
        assert!(self[0].len() == matrix.len());

        let mut res = Matrix::zeroed();
        // let tm = matrix.transpose(); // transposing doesn't give any speed improvement :/
        // looks like compilers are smart enough for this optimization
        // also probably isn't worth it in case of 3x3 matrices
        for y in 0..self.len() {
            for x in 0..matrix[0].len() {
                for p in 0..self[0].len() {
                    res[y][x] += self[y][p] * matrix[p][x];
                }
            }
        }
        res
    }
}