use std::mem;

use crate::{
    la::{Matrix, MatrixI, Vec3f},
    model::Model,
    tga::{self, Color},
    transform::{viewport,barycentric,get_viewport_matrix},
    utils::degrees_to_radians,

};

#[derive(Debug, Clone)]
pub struct ShaderConf {
    pub diff_light: bool,
    pub spec_light: bool,
    pub texture: bool,
    pub normals: bool,
    pub occlusion: bool,
}

impl ShaderConf {
    pub fn new() -> Self {
        ShaderConf {
            diff_light: true,
            spec_light: true,
            texture: true,
            normals: true,
            occlusion: true,
        }
    }
}

pub trait Shader {
    fn vertex(&mut self, face: usize, vertex: usize) -> Vec3f;
    fn fragment(&mut self, bar: &Vec3f);
}

pub struct LightShader<'a> {
    pub conf: ShaderConf,
    pub model: &'a Model,
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
        let t = self.model.texture_coords(face, vertex);

        for i in 0..2 {
            // the vertex(whose id = vertex)'s u and v assigned in varying_uv 
            self.varying_uv[i][vertex] = t[i];
        }

        // calculate the postion in out_texture
        let ss = viewport(&v, self.out_texture.width, self.out_texture.height);

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
        let [[x], [y], [z]] = self.varying_xy.mul(&bar_mtrx);
        let x = x.round() as i32;
        let y = y.round() as i32;
        
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

pub struct BasicShader<'a> {
    pub conf: ShaderConf,
    pub light_dir: Vec3f,
    pub lookat_m: Matrix<4, 4>,
    pub lookat_mi: Matrix<4, 4>,
    pub model: &'a Model,
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

pub fn triangle(v1: &Vec3f, v2: &Vec3f, v3: &Vec3f, sh: &mut dyn Shader) {
    let z = Vec3f(v2.0, v2.1, v2.2)
        .sub(&Vec3f(v1.0, v1.1, v1.2))
        .cross(&Vec3f(v3.0, v3.1, v3.2).sub(&Vec3f(v1.0, v1.1, v1.2)));

    // clip space 
    // clip z<0
    if z.2 < 0.0 {
        return;
    }

    let x0 = vec![v1.0, v2.0, v3.0]
        .iter()
        .fold(&v1.0, |xmin, x| if xmin > x { x } else { xmin })
        .round() as i32;
    let y0 = vec![v1.1, v2.1, v3.1]
        .iter()
        .fold(&v1.1, |ymin, y| if ymin > y { y } else { ymin })
        .round() as i32;
    let x1 = vec![v1.0, v2.0, v3.0]
        .iter()
        .fold(&v1.0, |xmax, x| if xmax < x { x } else { xmax })
        .round() as i32;
    let y1 = vec![v1.1, v2.1, v3.1]
        .iter()
        .fold(&v1.1, |ymax, y| if ymax < y { y } else { ymax })
        .round() as i32;

    for y in y0..=y1 {
        for x in x0..=x1 {
            let bc = barycentric(v1, v2, v3, (x as f32, y as f32));
            sh.fragment(&bc);
        }
    }
}

fn line(
    mut x0: i32,
    mut y0: i32,
    mut x1: i32,
    mut y1: i32,
    img: &mut tga::Image,
    color: tga::Color,
) {
    let dx = if x1 > x0 { x1 - x0 } else { x0 - x1 };
    let dy = if y1 > y0 { y1 - y0 } else { y0 - y1 };

    if dx > dy {
        if x1 < x0 {
            mem::swap(&mut x1, &mut x0);
            mem::swap(&mut y1, &mut y0);
        }
        for x in x0..=x1 {
            let t = ((x - x0) as f32) / ((x1 - x0) as f32);
            let y = (y0 as f32) * (1f32 - t) + (y1 as f32) * t;
            img.set_pixel(x as i32, y.round() as i32, color);
        }
    } else {
        if y1 < y0 {
            mem::swap(&mut x1, &mut x0);
            mem::swap(&mut y1, &mut y0);
        }
        for y in y0..=y1 {
            let t = ((y - y0) as f32) / ((y1 - y0) as f32);
            let x = (x0 as f32) * (1f32 - t) + (x1 as f32) * t;
            img.set_pixel(x.round() as i32, y as i32, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::Camera;

    #[test]
    fn test_matrixs(){
        let camera:Camera = Camera::new(Vec3f(0f32,0f32,50f32),Vec3f(0f32,0f32,0f32));
        let lookat_m = camera.get_lookat_view();
        // let perspective_matrix = _prespective_projection(degrees_to_radians(60f32),1f32,10f32,100f32);
        let mut viewport_matrix = get_viewport_matrix(512, 512);
        let model_view = lookat_m;
        // println!("{:?}",model_view);
        // println!("{:?}",viewport_matrix);
        // println!("{:?}",perspective_matrix);
        
        let v = Vec3f(0f32,0f32,50f32);
        let res:Vec3f = model_view
                        .mul(&v.embed::<4>(1f32))
                        .into();
        println!("{:?}",res);
                
    }

}
