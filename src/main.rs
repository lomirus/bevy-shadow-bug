#![windows_subsystem = "windows"]

mod camera;
mod pause_menu;
mod player;

use std::f32::consts::FRAC_PI_4;

use bevy::{
    pbr::{CascadeShadowConfig, CascadeShadowConfigBuilder},
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
    let mut window = window.get_single_mut().unwrap();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;

    commands.spawn(PbrBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 10000.0,
            subdivisions: 0,
        })),
        material: materials.add(Color::rgb(0.2, 1.0, 0.45).into()),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            shadows_enabled: true,
            illuminance: 20000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -FRAC_PI_4,
            -FRAC_PI_4,
            -FRAC_PI_4,
        )),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("sphere.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0) * 4.0,
            ..default()
        },
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_player)
        .add_system(handle_change_view)
        .add_system(handle_player_move)
        .add_system(handle_pause_menu)
        .add_system(update_player_model)
        .add_system(update_camera)
        // .add_plugin(WorldInspectorPlugin)
        // .register_type::<Player>()
        .run();
}
