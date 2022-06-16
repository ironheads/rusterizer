use super::{
    ShaderConf,
    Shader,

};

use crate::{
    la::{Matrix,Vec3f,MatrixI},
    models::MeshObject,
    tga::{self,Color},
    transform::get_viewport_matrix,
};


pub struct BasicShader<'a> {
    pub conf: ShaderConf,
    pub light_dir: Vec3f,
    pub lookat_m: Matrix<4, 4>,
    pub lookat_mi: Matrix<4, 4>,
    pub model: &'a MeshObject,
    pub out_texture: &'a mut tga::Image,
    pub light_texture: &'a mut tga::Image,
    pub z_buffer: &'a mut tga::ZBuffer,
    pub project_m: Matrix<4, 4>,
    pub varying_uv: Matrix<3, 2>,
    pub varying_xy: Matrix<3, 3>,
    pub vertices: [Vec3f; 3],
    pub normal_face_vec: Option<Vec3f>,
}

impl Shader for BasicShader<'_> {
    fn vertex(&mut self, face: usize, vertex: usize) -> Vec3f {
        let v = self.model.vertex(face, vertex);
        let t = self.model.texture_coords(face, vertex);

        for i in 0..2 {
            self.varying_uv[i][vertex] = t[i];
        }

        let perspective_matrix = self.project_m;

        let viewport_matrix = get_viewport_matrix(self.out_texture.width, self.out_texture.height);

        let model_view = self.lookat_m;
        
        let ss = viewport_matrix.mul(&perspective_matrix)
                                        .mul(&model_view)
                                        .mul(&v.embed::<4>(1f32))
                                        .into();

        self.vertices[vertex] = ss;

        self.varying_xy[0][vertex] = ss.0;
        self.varying_xy[1][vertex] = ss.1;
        self.varying_xy[2][vertex] = ss.2;

        // todo refactor
        // set vector that is perpendicular to current triangle
        if vertex == 2 {
            self.normal_face_vec = Some(
                self.vertices[1]
                    .sub(&self.vertices[0])
                    .cross(&self.vertices[2].sub(&self.vertices[1]))
                    .normalize(),
            );
        }

        ss
    }

    fn fragment(&mut self, bar: &Vec3f) {
        // check inside a triangle
        if bar.0 < 0.0 || bar.1 < 0.0 || bar.2 < 0.0 {
            return;
        }
        let bar_mtrx = bar.into();
        let [[x], [y], [z]] = self.varying_xy.mul(&bar_mtrx);
        let x = x.round() as i32;
        let y = y.round() as i32;

        // todo!("")
        // 非线形插值的坐标
        if z >= self.z_buffer.pixel_at(x, y)
            || x < 0
            || x >= self.out_texture.width
            || y < 0
            || y >= self.out_texture.height
            || z < 0.0
        {
            return;
        }

        let [[u], [v]] = self.varying_uv.mul(&bar_mtrx);

        let txt = if self.conf.texture {
            self.model.texture(u, v)
        } else {
            Color(150, 150, 150)
        };
        let normal_vec = if self.conf.normals {
            self.lookat_mi
                .mul(&(self.model.normal(u, v)).embed::<4>(0.0))
                .into()
        } else {
            *self.normal_face_vec.as_ref().unwrap()
        };
        let normal_vec = normal_vec.normalize();

        let light = normal_vec.mul(&self.light_dir);
        let reflected = normal_vec
            .mulf(normal_vec.mul(&self.light_dir) * 2.0)
            .sub(&self.light_dir)
            .normalize();
        let light_spec = reflected.2.powf(23.0); // cam on z

        let mut highlight = if self.conf.diff_light { light } else { 0.0f32 };
        highlight += if self.conf.spec_light { light_spec * 0.9 } else { 0.0 };

        let hc = (((highlight + 2.0) / 2.0) * 255.0 / 2.0).round() as u8;
        self.light_texture.set_pixel(x, y, Color(hc, hc, hc));

        self.out_texture.set_pixel(
            x,
            y,
            if self.conf.occlusion {
                txt
            } else {
                txt.highlight(highlight)
            },
        );
        self.z_buffer.set_pixel(x, y, z)
    }
}
