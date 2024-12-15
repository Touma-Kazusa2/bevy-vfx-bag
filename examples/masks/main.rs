#[path = "../examples_common.rs"]
mod examples_common;

use bevy::prelude::*;
use bevy_vfx_bag::{
    post_processing::{
        masks::{Mask, MaskVariant},
        simple_post_process::PostProcessShaderDef,
    },
    BevyVfxBagPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(examples_common::SaneDefaultsPlugin)
        .add_plugins(examples_common::ShapesExamplePlugin::without_3d_camera())
        .add_plugins(BevyVfxBagPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, examples_common::print_on_change::<Mask>)
        .add_systems(Update, change)
        .run();
}

fn setup(mut commands: Commands) {
    info!("Flips the screen orientation every interval.");

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        Mask::default(),
    ));
}

fn change(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Mask, With<Camera>>,
    mut shader_def: ResMut<PostProcessShaderDef<Mask>>,
) {
    let mut mask = query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Digit1) {
        *mask = Mask::square();
        shader_def.set_shader_defs(vec![MaskVariant::Square.into()]);
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        *mask = Mask::crt();
        shader_def.set_shader_defs(vec![MaskVariant::Crt.into()]);
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        *mask = Mask::vignette();
        shader_def.set_shader_defs(vec![MaskVariant::Vignette.into()]);
    }

    let shader_defined = shader_def.shader_defs().iter().next().unwrap().into();

    // Let user change strength in increments via up, down arrows
    let increment = || match shader_defined {
        MaskVariant::Square => 1.,
        MaskVariant::Crt => 1000.,
        MaskVariant::Vignette => 0.05,
    };

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        mask.strength += increment();
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        mask.strength -= increment();
    };

    if keyboard_input.pressed(KeyCode::PageUp) {
        mask.fade += 0.01;
    } else if keyboard_input.pressed(KeyCode::PageDown) {
        mask.fade -= 0.01;
    };

    //mask.fade = mask.fade.clamp(0.0, 1.0);

    // Let user go to low- and high strength values directly via L and H keys
    let low = || match shader_defined {
        MaskVariant::Square => 3.,
        MaskVariant::Crt => 3000.,
        MaskVariant::Vignette => 0.1,
    };

    let high = || match shader_defined {
        MaskVariant::Square => 100.,
        MaskVariant::Crt => 500000.,
        MaskVariant::Vignette => 1.5,
    };

    if keyboard_input.just_pressed(KeyCode::KeyL) {
        mask.strength = low();
    } else if keyboard_input.just_pressed(KeyCode::KeyH) {
        mask.strength = high();
    };
}
