use std::any::Any;

use crate::camera::{
    self, 
    PerspectiveCamera,
    Projectable,
    CameraTrait
};
use crate::la::{
    Vec3f,
    Matrix,
    MatrixI,
};
use crate::models::MeshObject;
use crate::raytracing::{ExposureCamera, Exposure};
use crate::scene::{RayTracingScene, RasterizableScene};
use crate::shader::{BasicShader, Shader, LightShader, ShaderConf};
use crate::tga::ZBuffer;
use crate::utils::ray_color;
use crate::{
    tga::{Image,Color},
};
use super::{
    Render, 
    RenderConfig,
    RasterizationConfig,
    RayTracingConfig, triangle,
};
use rand::Rng;
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*;
pub enum RenderType {
    RayTracing(RayTracingScene),
    Rasterization(RasterizableScene),
}

// some problems: Any can not be cast to trait (like Exposure/Projectable), that seems wierd
impl Render for RenderType {
    fn render(&self, camera: &dyn Any, config: RenderConfig) -> Result<Image,&'static str> {
        match self {
            RenderType::RayTracing(rayscene) => {
                let cam = camera.downcast_ref::<ExposureCamera>();
                let cfg = match config {
                    RenderConfig::Raytracing(rcfg) => {
                        rcfg
                    }
                    RenderConfig::Rasterization(_) => {
                        return Err("wrong config type");
                    }
                };
                let width = cfg.width as i32;
                let height = cfg.height as i32;
                let max_depth = cfg.max_depth as usize;
                let sample_per_pixel = cfg.sample_per_pixel as usize;
                match cam {
                    Some(raycamera) => {
                        let mut image = Image::new(width,height);
                        let bands: Vec<(usize, &mut [Color])> = image.data.chunks_mut(width as usize).enumerate().collect();
                        bands.into_par_iter().for_each(|(row, band)| {
                            let h = height as usize - row;
                            let mut rng = rand::thread_rng();
                            for column in 0..width {
                                let mut pixel_color = Vec3f(0.0, 0.0, 0.0);
                                for _s in 0..sample_per_pixel {
                                    let u: f32 = (column as f32 + rng.gen_range(0.0,1.0)) / (width - 1) as f32;
                                    let v: f32 = (h as f32 + rng.gen_range(0.0,1.0)) / (height - 1) as f32;
                                    let r = raycamera.exposure_ray(u, v);
                                    pixel_color = pixel_color + ray_color(&r, &rayscene, max_depth);
                                }
                                let pixel = pixel_color.to_color(sample_per_pixel);
                                band[column as usize] = pixel;
                            }
                        });
                        Ok(image)
                    }
                    None => {
                        Err("render ray tracing camera but not use ray tracing camera")
                    }
                }
            },
            RenderType::Rasterization(rasterscene) => {
                let cam = match camera.downcast_ref::<PerspectiveCamera>() {
                    Some(t) => t,
                    None => {
                        return Err("wrong camera");
                    }
                };
                let cfg = match config {
                    RenderConfig::Raytracing(_) => {
                        return Err("wrong config")
                    }
                    RenderConfig::Rasterization(rcfg) => {
                        rcfg
                    }
                };
                let width = cfg.width as i32;
                let height = cfg.height as i32;
                let shader_config = cfg.shader_config;
                let mut out_texture = Image::new(width, height);
                let mut z_buffer = ZBuffer::new(width, height);
                let mut light_texture = Image::new(width, height);
                let lookat_m = cam.get_lookat().clone();
                let lookat_mi = lookat_m.inverse().transpose();
                let light_dir: Vec3f = Vec3f(1.0, -0.0, 0.5).normalize();
                for obj in rasterscene.objects.iter() {
                    let mut shader = BasicShader {
                        conf: shader_config.clone(),
                        normal_face_vec: None,
                        light_dir,
                        lookat_m,
                        lookat_mi,
                        model: obj,
                        out_texture: &mut out_texture,
                        z_buffer: &mut z_buffer,
                        varying_uv: Matrix::zeroed(),
                        varying_xy: Matrix::zeroed(),
                        vertices: [Vec3f::zeroed(); 3],
                        light_texture: &mut light_texture,
                        project_m: cam.get_projection().clone(),
                    };
                    for f in 0..obj.num_faces() {
                        let mut vertices = [Vec3f::zeroed(), Vec3f::zeroed(), Vec3f::zeroed()];
                        for v in 0..3 {
                            vertices[v] = shader.vertex(f, v);
                        }
                        triangle(&vertices[0], &vertices[1], &vertices[2], &mut shader);
                    }
                }

                let light_model = MeshObject::screen_texture_model();

                if shader_config.occlusion {
                    let mut occl_texture = Image::new(width, height);
                    let mut light_shader = LightShader {
                        conf: ShaderConf::new(),
                        model: &light_model,
                        out_texture: &mut out_texture,
                        light_texture: &mut light_texture,
                        z_buffer: &mut z_buffer,
                        varying_uv: Matrix::zeroed(),
                        varying_xy: Matrix::zeroed(),
                        occl_texture: &mut occl_texture,
                    };

                    for f in 0..light_model.num_faces() {
                        let mut vertices = [Vec3f::zeroed(), Vec3f::zeroed(), Vec3f::zeroed()];
                        for v in 0..3 {
                            vertices[v] = light_shader.vertex(f, v);
                        }
                        triangle(&vertices[0], &vertices[1], &vertices[2], &mut light_shader);
                    }
                }

                out_texture.apply_gamma(1.5);
                Ok(out_texture)
            },
        }
    }
}