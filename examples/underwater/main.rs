#[path = "../examples_common.rs"]
mod examples_common;

use bevy::prelude::*;
use bevy_vfx_bag::{
    post_processing::{
        chromatic_aberration::ChromaticAberration,
        lut::{Lut, LutPostProcessBindGroup},
        masks::Mask,
        raindrops::Raindrops,
        wave::Wave,
    },
    BevyVfxBagPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(examples_common::SaneDefaultsPlugin)
        .add_plugins(examples_common::ShapesExamplePlugin::without_3d_camera())
        .add_plugins(BevyVfxBagPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut bind_group_asset: ResMut<Assets<LutPostProcessBindGroup>>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        Raindrops::default(),
        ChromaticAberration {
            magnitude_r: 0.003,
            magnitude_g: 0.003,
            magnitude_b: 0.003,
            ..default()
        },
        Wave {
            waves_x: 1.,
            speed_x: 0.1,
            amplitude_x: 0.07,
            waves_y: 10.,
            speed_y: 0.3,
            amplitude_y: 0.01,
        },
        Lut::arctic(&mut bind_group_asset),
        Mask::vignette(),
    ));
}
