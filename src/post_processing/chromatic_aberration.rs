use bevy::{
    prelude::*,
    render::{extract_component::ExtractComponent, render_graph::RenderLabel, render_resource::*},
};

use std::{f32::consts::PI, fmt::Display};

use super::simple_post_process::SimplePostProcess;

///TODO
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel, Default)]
pub struct ChromaticAberrationPostProcessLabel;

/// Pixelate settings.
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct ChromaticAberration {
    /// The direction (in UV space) the red channel is offset in.
    /// Will be normalized.
    pub dir_r: Vec2,

    /// How far (in UV space) the red channel should be displaced.
    pub magnitude_r: f32,

    /// The direction (in UV space) the green channel is offset in.
    /// Will be normalized.
    pub dir_g: Vec2,

    /// How far (in UV space) the green channel should be displaced.
    pub magnitude_g: f32,

    /// The direction (in UV space) the blue channel is offset in.
    /// Will be normalized.
    pub dir_b: Vec2,

    /// How far (in UV space) the blue channel should be displaced.
    pub magnitude_b: f32,
}

impl ChromaticAberration {
    /// Adds the given diff to the magnitude of all color channels.
    pub fn add_magnitude(&mut self, diff: f32) {
        self.magnitude_r += diff;
        self.magnitude_g += diff;
        self.magnitude_b += diff;
    }
}

impl Default for ChromaticAberration {
    fn default() -> Self {
        let one_third = (2. / 3.) * PI;

        Self {
            dir_r: Vec2::from_angle(0. * one_third),
            magnitude_r: 0.01,
            dir_g: Vec2::from_angle(1. * one_third),
            magnitude_g: 0.01,
            dir_b: Vec2::from_angle(2. * one_third),
            magnitude_b: 0.01,
        }
    }
}

impl Display for ChromaticAberration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base_angle = Vec2::new(1., 0.);
        let angle = |color_dir| base_angle.angle_to(color_dir) * 180. / PI + 180.;

        write!(
            f,
            "Chromatic Aberration [magnitude, angle]:  R: [{:.3}, {:4.0}°] G: [{:.3}, {:4.0}°] B: [{:.3}, {:4.0}°]",
            self.magnitude_r,
            angle(self.dir_r),
            self.magnitude_g,
            angle(self.dir_g),
            self.magnitude_b,
            angle(self.dir_b)
        )
    }
}

impl SimplePostProcess for ChromaticAberration {
    fn shader_path() -> String {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/shaders/",
            "chromatic-aberration.wgsl"
        )
        .into()
    }
    type Label = ChromaticAberrationPostProcessLabel;
}
