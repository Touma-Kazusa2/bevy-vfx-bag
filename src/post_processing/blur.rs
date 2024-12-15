use bevy::{
    prelude::*,
    render::{extract_component::ExtractComponent, render_graph::RenderLabel, render_resource::*},
};

use std::fmt::Display;

use super::simple_post_process::SimplePostProcess;

///TODO
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel, Default)]
pub struct BlurPostProcessLabel;

/// Pixelate settings.
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct Blur {
    /// How blurry the output image should be.
    /// If `0.0`, no blur is applied.
    /// `1.0` is "fully blurred", but higher values will produce interesting results.
    pub amount: f32,

    /// How far away the blur should sample points away from the origin point
    /// when blurring.
    /// This is in UV coordinates, so small (positive) values are expected (`0.01` is a good start).
    pub kernel_radius: f32,
}

impl Default for Blur {
    fn default() -> Self {
        Self {
            amount: 0.5,
            kernel_radius: 0.01,
        }
    }
}

impl Display for Blur {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Blur amount: {}, radius: {}",
            self.amount, self.kernel_radius
        )
    }
}

impl SimplePostProcess for Blur {
    fn shader_path() -> String {
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/", "blur.wgsl").into()
    }
    type Label = BlurPostProcessLabel;
}
