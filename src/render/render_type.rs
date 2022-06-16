use crate::scene::RayTracingScene;


pub enum RenderType {
    RayTracing(Box<RayTracingScene>),
    Rasterization(),
}