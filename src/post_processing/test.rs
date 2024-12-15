use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponent, render_graph::RenderLabel, render_resource::*,
        renderer::RenderDevice,
    },
};
use binding_types::uniform_buffer;

use super::simple_post_process::{SimplePostProcess, TextureInputs};
///TODO
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct TestPostProcessSettings {
    ///TODO
    pub intensity: f32,
    // WebGL2 structs must be 16 byte aligned.
    //#[cfg(feature = "webgl2")]
    //_webgl2_padding: Vec3,
}

impl SimplePostProcess for TestPostProcessSettings {
    fn shader_path() -> String {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/shaders/",
            "post_processing.wgsl"
        )
        .into()
    }
    type Label = TestPostProcessLabel;
    fn layout(device: &RenderDevice) -> BindGroupLayout {
        device.create_bind_group_layout(
            "flip_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (uniform_buffer::<TestPostProcessSettings>(true),),
            ),
        )
    }
    fn bind_group(
        _world: &World,
        device: &RenderDevice,
        layout: &BindGroupLayout,
        buffer: BindingResource,
        _textures: &TextureInputs,
    ) -> BindGroup {
        device.create_bind_group(
            "flip_bind_group",
            layout,
            &BindGroupEntries::sequential((buffer,)),
        )
    }
}

///TODO
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel, Default)]
pub struct TestPostProcessLabel;
