use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::pause_menu::PauseMenuUI;

pub(crate) const PLAYER_HEIGHT: f32 = 64.0;
const PLAYER_MOVE_SPEED: f32 = 0.1;

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub(crate) struct Player {
    #[inspector()]
    pub(crate) coord: Vec3,
}

pub(crate) fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player {
            coord: Vec3::new(0.0, 0.0, 0.0),
        },
        SceneBundle {
            scene: asset_server.load("player.glb#Scene0"),
            ..default()
        },
    ));
}

pub(crate) fn handle_player_move(
    mut player: Query<&mut Player>,
    game_paused: Query<&PauseMenuUI>,
    camera: Query<&Transform, With<Camera3d>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !game_paused.is_empty() {
        return;
    }

    let mut player = player.iter_mut().next().unwrap();
    let camera = camera.iter().next().unwrap();

    if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::S) {
        let forward = camera.forward();
        let forward = Vec2::new(forward.x, forward.z).normalize();

        if keyboard_input.pressed(KeyCode::W) {
            player.coord.x += forward.x * PLAYER_MOVE_SPEED;
            player.coord.z += forward.y * PLAYER_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) {
            player.coord.x -= forward.x * PLAYER_MOVE_SPEED;
            player.coord.z -= forward.y * PLAYER_MOVE_SPEED;
        }
    }

    if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::D) {
        let left = camera.left();

        if keyboard_input.pressed(KeyCode::A) {
            player.coord.x += left.x * PLAYER_MOVE_SPEED;
            player.coord.z += left.z * PLAYER_MOVE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::D) {
            player.coord.x -= left.x * PLAYER_MOVE_SPEED;
            player.coord.z -= left.z * PLAYER_MOVE_SPEED;
        }
    }

    if keyboard_input.pressed(KeyCode::LControl) {
        player.coord.y -= PLAYER_MOVE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        player.coord.y += PLAYER_MOVE_SPEED;
    }
}

pub(crate) fn update_player_model(mut player: Query<(&Player, &mut Transform)>) {
    let (player, mut transform) = player.iter_mut().next().unwrap();
    transform.translation = Vec3::new(
        player.coord.x,
        player.coord.y ,
        player.coord.z,
    );
}
