use crate::{
    shader::ShaderConf,
};
pub struct RayTracingConfig {
    pub height: u32,
    pub width: u32,
    pub max_depth: u32,
    pub sample_per_pixel: u32,
}

pub struct RasterizationConfig {
    pub height: u32,
    pub width: u32,
    pub shader_config: ShaderConf,
}

pub enum RenderConfig {
    Raytracing(RayTracingConfig),
    Rasterization(RasterizationConfig),
}