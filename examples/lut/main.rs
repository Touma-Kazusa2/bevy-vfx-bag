#[path = "../examples_common.rs"]
mod examples_common;

use bevy::prelude::*;
use bevy_vfx_bag::{
    post_processing::lut::{Lut, LutPostProcessBindGroup},
    BevyVfxBagPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(examples_common::SaneDefaultsPlugin)
        .add_plugins(examples_common::ShapesExamplePlugin::without_3d_camera())
        .add_plugins(BevyVfxBagPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, change)
        .run();
}

fn setup(mut commands: Commands, mut bind_group_asset: ResMut<Assets<LutPostProcessBindGroup>>) {
    info!("Press [left|right] to change which LUT is in use");

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        Lut::neo(&mut bind_group_asset),
    ));
}

// Cycle through some preset LUTs.
fn change(
    mut choice: Local<usize>,
    mut commands: Commands,
    mut bind_group_asset: ResMut<Assets<LutPostProcessBindGroup>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<Entity, With<Camera>>,
) {
    let choice_now = if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        choice.saturating_sub(1)
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        (*choice + 1).min(3)
    } else {
        *choice
    };

    if *choice != choice_now {
        let entity = query.single_mut();

        *choice = choice_now;
        match *choice {
            0 => {
                commands
                    .entity(entity)
                    .insert(Lut::neo(&mut bind_group_asset));
                info!("Neo");
            }
            1 => {
                commands
                    .entity(entity)
                    .insert(Lut::arctic(&mut bind_group_asset));
                info!("Arctic");
            }
            2 => {
                commands
                    .entity(entity)
                    .insert(Lut::slate(&mut bind_group_asset));
                info!("Slate");
            }
            3 => {
                commands.entity(entity).remove::<Lut>();
                info!("Disabled (default Bevy colors)");
            }
            _ => unreachable!(),
        }
    }
}
