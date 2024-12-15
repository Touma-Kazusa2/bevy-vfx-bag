use bevy::{
    prelude::*,
    render::{
        extract_component::ExtractComponent, render_graph::RenderLabel, render_resource::*,
        renderer::RenderDevice,
    },
};
use binding_types::uniform_buffer;

use std::fmt::Display;

use super::simple_post_process::{SimplePostProcess, TextureInputs};
///TODO
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct FlipUniform {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl From<Flip> for FlipUniform {
    fn from(flip: Flip) -> Self {
        let uv = match flip {
            Flip::None => [0.0, 0.0],
            Flip::Horizontal => [1.0, 0.0],
            Flip::Vertical => [0.0, 1.0],
            Flip::HorizontalVertical => [1.0, 1.0],
        };

        Self { x: uv[0], y: uv[1] }
    }
}

/// Which way to flip the texture.
#[derive(Debug, Default, Copy, Clone, Component)]
pub enum Flip {
    /// Don't flip.
    None,

    /// Flip horizontally.
    #[default]
    Horizontal,

    /// Flip vertically.
    Vertical,

    /// Flip both axes.
    HorizontalVertical,
}

impl Display for Flip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl SimplePostProcess for FlipUniform {
    fn shader_path() -> String {
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/", "flip.wgsl").into()
    }
    type Label = FlipPostProcessLabel;
    fn layout(device: &RenderDevice) -> BindGroupLayout {
        device.create_bind_group_layout(
            "flip_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (uniform_buffer::<FlipUniform>(true),),
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
pub struct FlipPostProcessLabel;
