mod camera;
mod pause_menu;
mod player;

use std::f32::consts::FRAC_PI_4;

use bevy::{
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use camera::{handle_change_view, setup_camera, update_camera};
use pause_menu::handle_pause_menu;
use player::{handle_player_move, setup_player, update_player_model};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let mut window = window.single_mut().unwrap();
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Locked;

    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 1.0, 0.45))),
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(1.0, 1.0, 1.0),
            shadows_enabled: true,
            illuminance: light_consts::lux::FULL_DAYLIGHT,
            ..default()
        },
        CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 0.1,
            maximum_distance: 1000.0,
            first_cascade_far_bound: 5.0,
            overlap_proportion: 0.2,
        }
        .build(),
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -FRAC_PI_4,
            -FRAC_PI_4,
            -FRAC_PI_4,
        )),
    ));

    commands.spawn((
        SceneRoot(asset_server.load("sphere.glb#Scene0")),
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0) * 4.0,
            ..default()
        },
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_player)
        .add_systems(Update, handle_change_view)
        .add_systems(Update, handle_player_move)
        .add_systems(Update, handle_pause_menu)
        .add_systems(Update, update_player_model)
        .add_systems(Update, update_camera)
        // .add_plugin(WorldInspectorPlugin)
        // .register_type::<Player>()
        .run();
}
