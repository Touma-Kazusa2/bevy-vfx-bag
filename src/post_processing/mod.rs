use bevy::{
    core_pipeline::core_3d::graph::{Core3d, Node3d},
    prelude::*,
    render::{render_graph::RenderGraphApp, RenderApp},
};
use blur::{Blur, BlurPostProcessLabel};
use chromatic_aberration::{ChromaticAberration, ChromaticAberrationPostProcessLabel};
use flip::{FlipPostProcessLabel, FlipUniform};
use lut::{Lut, LutPostProcessLabel};
use masks::{Mask, MaskPostProcessLabel};
use pixelate::{Pixelate, PixelatePostProcessLabel};
use post_process::PostProcessPlugin;
use raindrops::{Raindrops, RaindropsPostProcessLabel};
use simple_post_process::SimplePostProcessPlugin;
use test::{TestPostProcessLabel, TestPostProcessSettings};
use wave::{Wave, WavePostProcessLabel};

///TODO
pub mod simple_post_process;

///TODO
pub mod post_process;

///TODO
pub mod test;

///TODO
pub mod flip;

///TODO
pub mod raindrops;

///TODO
pub mod masks;

///TODO
pub mod lut;

///TODO
pub mod pixelate;

///TODO
pub mod wave;

///TODO
pub mod blur;

///TODO
pub mod chromatic_aberration;

///TODO
#[derive(Debug, Default)]
pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SimplePostProcessPlugin::<TestPostProcessSettings>::default(),
            SimplePostProcessPlugin::<FlipUniform>::default(),
            SimplePostProcessPlugin::<Raindrops>::default(),
            SimplePostProcessPlugin::<Mask>::default(),
            PostProcessPlugin::<Lut>::default(),
            SimplePostProcessPlugin::<ChromaticAberration>::default(),
            SimplePostProcessPlugin::<Blur>::default(),
            SimplePostProcessPlugin::<Wave>::default(),
            SimplePostProcessPlugin::<Pixelate>::default(),
        ));
    }
}

#[derive(Debug, Default)]
pub(crate) struct PostProcessingDefaultOrderPlugin;

impl Plugin for PostProcessingDefaultOrderPlugin {
    fn build(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.add_render_graph_edges(
            Core3d,
            (
                Node3d::Tonemapping,
                PixelatePostProcessLabel,
                TestPostProcessLabel,
                FlipPostProcessLabel,
                RaindropsPostProcessLabel,
                MaskPostProcessLabel,
                LutPostProcessLabel,
                ChromaticAberrationPostProcessLabel,
                BlurPostProcessLabel,
                WavePostProcessLabel,
                Node3d::EndMainPassPostProcessing,
            ),
        );
    }
}
