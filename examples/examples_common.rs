use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use core::f32::consts::PI;
use std::fmt::Display;

/// Adds some "sane defaults" for showing examples/development:
///
/// * The default Bevy plugins
/// * Hot reloading
/// * Close on ESC button press
pub struct SaneDefaultsPlugin;

#[allow(dead_code)]
pub(crate) fn print_on_change<T: Display + Component>(things: Query<&T, Changed<T>>) {
    for thing in &things {
        info!("{thing}");
    }
}

impl Plugin for SaneDefaultsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Update, close_on_esc_system);
    }
}

fn close_on_esc_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}

/// This plugin combines two Bevy examples:
///
/// https://github.com/bevyengine/bevy/blob/v0.8.1/examples/3d/shapes.rs
/// https://github.com/bevyengine/bevy/blob/v0.8.1/examples/ui/text.rs
///
/// This example can be started by just loading this plugin.
/// This is useful to separate this crate's code and role from regular upstream Bevy code.
pub struct ShapesExamplePlugin {
    add_3d_camera_bundle: bool,
}

impl ShapesExamplePlugin {
    pub fn without_3d_camera() -> Self {
        Self {
            add_3d_camera_bundle: false,
        }
    }
}

#[derive(Resource)]
pub(crate) struct ShouldAdd3dCameraBundle(bool);

impl Plugin for ShapesExamplePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShouldAdd3dCameraBundle(self.add_3d_camera_bundle))
            .add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, shapes::setup)
            .add_systems(Startup, ui::setup)
            .add_systems(Update, shapes::rotate)
            .add_systems(Update, ui::fps_text_update);
    }
}

#[derive(Component)]
pub(crate) struct Shape;

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;

mod shapes {
    use bevy::{asset::RenderAssetUsages, color::palettes::css::SILVER};

    use super::*;

    pub(crate) fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut images: ResMut<Assets<Image>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        add_3d_camera_bundle: Res<ShouldAdd3dCameraBundle>,
    ) {
        let debug_material = materials.add(StandardMaterial {
            base_color_texture: Some(images.add(uv_debug_texture())),
            ..default()
        });

        let shapes = [
            meshes.add(Cuboid::default()),
            meshes.add(Tetrahedron::default()),
            meshes.add(Capsule3d::default()),
            meshes.add(Torus::default()),
            meshes.add(Cylinder::default()),
            meshes.add(Cone::default()),
            meshes.add(ConicalFrustum::default()),
            meshes.add(Sphere::default().mesh().ico(5).unwrap()),
            meshes.add(Sphere::default().mesh().uv(32, 18)),
        ];

        let extrusions = [
            meshes.add(Extrusion::new(Rectangle::default(), 1.)),
            meshes.add(Extrusion::new(Capsule2d::default(), 1.)),
            meshes.add(Extrusion::new(Annulus::default(), 1.)),
            meshes.add(Extrusion::new(Circle::default(), 1.)),
            meshes.add(Extrusion::new(Ellipse::default(), 1.)),
            meshes.add(Extrusion::new(RegularPolygon::default(), 1.)),
            meshes.add(Extrusion::new(Triangle2d::default(), 1.)),
        ];

        let num_shapes = shapes.len();

        for (i, shape) in shapes.into_iter().enumerate() {
            commands.spawn((
                Mesh3d(shape),
                MeshMaterial3d(debug_material.clone()),
                Transform::from_xyz(
                    -SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
                    2.0,
                    Z_EXTENT / 2.,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                Shape,
            ));
        }

        let num_extrusions = extrusions.len();

        for (i, shape) in extrusions.into_iter().enumerate() {
            commands.spawn((
                Mesh3d(shape),
                MeshMaterial3d(debug_material.clone()),
                Transform::from_xyz(
                    -EXTRUSION_X_EXTENT / 2.
                        + i as f32 / (num_extrusions - 1) as f32 * EXTRUSION_X_EXTENT,
                    2.0,
                    -Z_EXTENT / 2.,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                Shape,
            ));
        }

        commands.spawn((
            PointLight {
                shadows_enabled: true,
                intensity: 10_000_000.,
                range: 100.0,
                shadow_depth_bias: 0.2,
                ..default()
            },
            Transform::from_xyz(8.0, 16.0, 8.0),
        ));

        // ground plane
        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
            MeshMaterial3d(materials.add(Color::from(SILVER))),
        ));

        if add_3d_camera_bundle.0 {
            commands.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ));
        }

        // #[cfg(not(target_arch = "wasm32"))]
        // commands.spawn((
        //     Text::new("Press space to toggle wireframes"),
        //     Node {
        //         position_type: PositionType::Absolute,
        //         top: Val::Px(12.0),
        //         left: Val::Px(12.0),
        //         ..default()
        //     },
        // ));
    }

    pub(crate) fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_secs() / 2.);
        }
    }

    /// Creates a colorful test pattern
    fn uv_debug_texture() -> Image {
        const TEXTURE_SIZE: usize = 8;

        let mut palette: [u8; 32] = [
            255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102,
            255, 198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
        ];

        let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
        for y in 0..TEXTURE_SIZE {
            let offset = TEXTURE_SIZE * y * 4;
            texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
            palette.rotate_right(4);
        }

        Image::new_fill(
            Extent3d {
                width: TEXTURE_SIZE as u32,
                height: TEXTURE_SIZE as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &texture_data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////
// UI
////////////////////////////////////////////////////////////////////////////////

mod ui {
    use bevy::{
        color::palettes::css::GOLD,
        diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    };

    use super::*;

    // A unit struct to help identify the FPS UI component, since there may be many Text components
    #[derive(Component)]
    pub(crate) struct FpsText;

    pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Text with multiple sections
        commands
            .spawn((
                // Create a Text with multiple child spans.
                Text::new("FPS: "),
                TextFont {
                    // This font is loaded and will be used instead of the default font.
                    font: asset_server.load(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/assets/fonts/FiraSans-Bold.ttf"
                    )),
                    font_size: 42.0,
                    ..default()
                },
            ))
            .with_child((
                TextSpan::default(),
                (
                    TextFont {
                        font: asset_server.load(concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/assets/fonts/FiraMono-Medium.ttf"
                        )),
                        font_size: 33.0,
                        ..Default::default()
                    },
                    TextColor(GOLD.into()),
                ),
                FpsText,
            ));
    }

    pub(crate) fn fps_text_update(
        diagnostics: Res<DiagnosticsStore>,
        mut query: Query<&mut TextSpan, With<FpsText>>,
    ) {
        for mut span in &mut query {
            if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(value) = fps.smoothed() {
                    // Update the value of the second section
                    **span = format!("{value:.2}");
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// MAIN
////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
fn main() {
    println!("Not an example, just shared code between examples")
}
