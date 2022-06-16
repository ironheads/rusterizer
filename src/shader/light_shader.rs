use crate::{
    tga::{self,Color},
    models::MeshObject,
    la::{Vec3f,Matrix,MatrixI},
    transform::get_viewport_matrix,
};

use super::{
    Shader,
    ShaderConf,
};

pub struct LightShader<'a> {
    pub conf: ShaderConf,
    pub model: &'a MeshObject,
    pub out_texture: &'a mut tga::Image,
    pub light_texture: &'a mut tga::Image,
    pub occl_texture: &'a mut tga::Image,
    pub z_buffer: &'a mut tga::ZBuffer,
    pub varying_uv: Matrix<3, 2>,
    pub varying_xy: Matrix<3, 3>,
}

impl Shader for LightShader<'_> {

    fn vertex(&mut self, face: usize, vertex: usize) -> Vec3f {
        let v = self.model.vertex(face, vertex);
        // calculate the postion in out_texture
        let viewport_matrix = get_viewport_matrix(self.out_texture.width, self.out_texture.height);
        let ss: Vec3f = viewport_matrix.mul(&v.embed::<4>(1f32)).into();
        // the vertex(whose id = vertex)'s  x,y,z (z is calculated in [0,255]) assigned in varying_uv
        self.varying_xy[0][vertex] = ss.0;
        self.varying_xy[1][vertex] = ss.1;
        self.varying_xy[2][vertex] = ss.2;
        ss
    }
    
    fn fragment(&mut self, bar: &Vec3f) {
        // bar : the postion of the fragment
        if bar.0 < 0.0 || bar.1 < 0.0 || bar.2 < 0.0 {
            return;
        }
        let bar_mtrx = bar.into();
        let [[x], [y], _z] = self.varying_xy.mul(&bar_mtrx);
        let x = x.round() as i32;
        let y = y.round() as i32;
        if  x < 0
            || x >= self.out_texture.width
            || y < 0
            || y >= self.out_texture.height
        {
            return;
        }
        let current_z = self.z_buffer.pixel_at(x, y) / 255.0;
        // let [[u],[v]] = self.varying_uv.mul(&bar_mtrx);
        let mut total = 0.0;
        // hacky screen space ambient occlusion
        for yy in (y - 5).max(0)..(y + 5).min(self.out_texture.height) {
            for xx in (x - 5).max(0)..(x + 5).min(self.out_texture.width) {
                let surr_z = self.z_buffer.pixel_at(xx, yy) / 255.0;
                if current_z <= 0.01 {
                    continue;
                }
                if surr_z > current_z && surr_z - current_z > 0.01 {
                    total += (surr_z - current_z).min(0.05);
                }
            }
        }

        total /= 2.0;

        // check if not already set
        if self.occl_texture.pixel_at(x, y).0 == 0 {
            self.occl_texture.set_pixel(
                x,
                y,
                Color(
                    (total * 254.0).min(254.0) as u8 + 1,
                    (total * 254.0).min(254.0) as u8 + 1,
                    (total * 254.0).min(254.0) as u8 + 1,
                ),
            );
            let texture = self.out_texture.pixel_at(x, y);
            let mut light = (2.0 * self.light_texture.pixel_at(x, y).0 as f32 / 255.0) * 2.0 - 2.0;
            light -= total;
            self.out_texture.set_pixel(x, y, texture.highlight(light));
        }
    }
}
