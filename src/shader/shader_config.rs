
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
            occlusion: false,
        }
    }
}
