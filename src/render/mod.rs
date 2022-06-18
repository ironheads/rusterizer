mod shader_render;
mod render_type;
mod traits;
mod config;

pub use shader_render::{triangle,line};
pub use traits::Render;
pub use config::{RenderConfig,RasterizationConfig,RayTracingConfig};
