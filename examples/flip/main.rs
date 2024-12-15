#[path = "../examples_common.rs"]
mod examples_common;

use bevy::prelude::*;
use bevy_vfx_bag::{
    post_processing::flip::{Flip, FlipUniform},
    BevyVfxBagPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(examples_common::SaneDefaultsPlugin)
        .add_plugins(examples_common::ShapesExamplePlugin::without_3d_camera())
        .add_plugins(BevyVfxBagPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, examples_common::print_on_change::<Flip>)
        .add_systems(FixedUpdate, switch)
        .insert_resource(Time::<Fixed>::from_seconds(1.5))
        .run();
}

fn setup(mut commands: Commands) {
    info!("Flips the screen orientation every interval.");

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        Flip::default(),
    ));
}

// Switch flip modes every second.
fn switch(mut query: Query<(Entity, &mut Flip), With<Camera3d>>, mut commands: Commands) {
    let (entity, mut flip) = query.single_mut();

    *flip = match *flip {
        Flip::None => Flip::Horizontal,
        Flip::Horizontal => Flip::Vertical,
        Flip::Vertical => Flip::HorizontalVertical,
        Flip::HorizontalVertical => Flip::None,
    };
    let uniform: FlipUniform = FlipUniform::from(*flip);
    commands.entity(entity).insert(uniform);
}
