use bevy::{
    prelude::*,
    render::{extract_component::ExtractComponent, render_graph::RenderLabel, render_resource::*},
};

use std::fmt::Display;

use super::simple_post_process::SimplePostProcess;

///TODO
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel, Default)]
pub struct PixelatePostProcessLabel;

/// Pixelate settings.
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct Pixelate {
    /// How many pixels in the width and height in a block after pixelation. One block has a constant color within it.
    ///
    /// The shader sets a lower bound to 1.0, since that would not change the outcome.
    pub block_size: f32,
}

impl Default for Pixelate {
    fn default() -> Self {
        Self { block_size: 8.0 }
    }
}

impl Display for Pixelate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pixelate block size: {}", self.block_size)
    }
}

impl SimplePostProcess for Pixelate {
    fn shader_path() -> String {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/shaders/",
            "pixelate.wgsl"
        )
        .into()
    }
    type Label = PixelatePostProcessLabel;
}
