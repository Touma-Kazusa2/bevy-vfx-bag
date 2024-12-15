use bevy::{
    prelude::*,
    render::{extract_component::ExtractComponent, render_graph::RenderLabel, render_resource::*},
};

//use std::fmt::Display;

use super::simple_post_process::SimplePostProcess;

///TODO
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel, Default)]
pub struct WavePostProcessLabel;

/// Pixelate settings.
#[derive(Default, Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct Wave {
    /// How many waves in the x axis.
    pub waves_x: f32,

    /// How many waves in the y axis.
    pub waves_y: f32,

    /// How fast the x axis waves oscillate.
    pub speed_x: f32,

    /// How fast the y axis waves oscillate.
    pub speed_y: f32,

    /// How much displacement the x axis waves cause.
    pub amplitude_x: f32,

    /// How much displacement the y axis waves cause.
    pub amplitude_y: f32,
}

// impl Display for Wave {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Pixelate block size: {}", self.block_size)
//     }
// }

impl SimplePostProcess for Wave {
    fn shader_path() -> String {
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/", "wave.wgsl").into()
    }
    type Label = WavePostProcessLabel;
}
